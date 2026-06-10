// SPDX-License-Identifier: GPL-3.0-or-later
//
// Copyright (C) 2026 Domatix and contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Live preview pipeline (T-200, v0.4 milestone).
//!
//! Wraps a `v4l2src device=/dev/videoN ! videoconvert !
//! gtk4paintablesink` `GStreamer` pipeline so the controls page can
//! show the camera's frames in-app without launching Cheese / OBS /
//! `v4l2-ctl --stream-mmap` as a side process. The
//! `gtk4paintablesink` element exposes its rendered surface as a
//! `gdk::Paintable`, which the GUI mounts into a `gtk::Picture`
//! placed above the PTZ pad.
//!
//! ## Build gating
//!
//! This module is **only compiled when the `live-preview` Cargo
//! feature is enabled** (`cargo build -p obsbot-gui --features
//! live-preview`). The feature is off by default so contributors
//! without `libgstreamer1.0-dev` + `gstreamer1.0-gtk4` system
//! packages installed can still build and test the rest of the
//! crate. The Flatpak manifest enables it once the `GStreamer`
//! plugin packages land in `flathub.yaml`. See PLAN.md §T-200 for
//! the install incantation on Debian / Arch.
//!
//! ## Lifecycle
//!
//! - [`PreviewPipeline::new`] builds the pipeline in `NULL` state
//!   and snapshots the paintable so the `gtk::Picture` can already
//!   bind to it (will simply render nothing until `start`).
//! - [`PreviewPipeline::start`] transitions to `PLAYING` against a
//!   `/dev/videoN` path; failures (device busy, plugin missing)
//!   surface as `Err` so the caller can route them through
//!   [`settings::surface_error`](crate::settings::surface_error).
//! - [`PreviewPipeline::stop`] transitions back to `NULL` and
//!   releases the V4L2 device so other apps can claim it.
//! - Dropping the struct stops the pipeline implicitly via the
//!   `Drop` impl below — useful when the controls page is replaced
//!   (T-110 hot-plug REMOVE, navigation back to the camera list).
//!
//! ## Why two ioctls per click is fine
//!
//! Unrelated to preview, but worth noting since T-101a's hold path
//! reads pan/tilt at 50 ms cadence: `GStreamer`'s `v4l2src` holds
//! the capture queue open on its own file descriptor, while
//! `obsbot-core::write_control` opens a separate fd for each
//! ioctl. The kernel uvcvideo driver accepts both descriptors
//! concurrently — confirmed during T-101a hardware validation.

use std::cell::RefCell;
use std::path::Path;
use std::rc::{Rc, Weak};

use gstreamer as gst;
use gstreamer::prelude::*;
use gtk4 as gtk;

use crate::i18n::gettext;

thread_local! {
    /// Weak handle to the currently-visible controls page's pipeline
    /// slot (T-207). Set by `controls_view::build_controls_page` when
    /// it wires the preview, so a window-level handler
    /// (`window::build`'s `connect_close_request`) can release the
    /// V4L2 device on close without threading the `Rc` through the
    /// window. Weak (not strong) so the page's own teardown — and the
    /// `connect_hidden` stop wired alongside this — still owns the
    /// pipeline lifetime; this is purely a back-reference for the
    /// close path. A later page build supersedes the entry, and a
    /// failed upgrade (page already gone) is a silent no-op.
    static ACTIVE_PREVIEW: RefCell<Option<Weak<RefCell<Option<PreviewPipeline>>>>> =
        const { RefCell::new(None) };
}

/// Register the controls page's pipeline slot as the active preview
/// (T-207). See [`stop_active`]. Called once per controls-page build
/// right after the preview machinery is wired.
pub fn register_active(slot: &Rc<RefCell<Option<PreviewPipeline>>>) {
    ACTIVE_PREVIEW.with(|cell| *cell.borrow_mut() = Some(Rc::downgrade(slot)));
}

/// Stop whatever preview pipeline is currently active (T-207), if any.
/// Used by the window's close handler so the camera's capture node is
/// released deterministically on close instead of relying on `Drop`
/// ordering at process teardown. No-op when no preview is active or
/// the page that owned it is already gone.
pub fn stop_active() {
    ACTIVE_PREVIEW.with(|cell| {
        if let Some(slot) = cell.borrow().as_ref().and_then(Weak::upgrade) {
            if let Some(pipeline) = slot.borrow_mut().as_mut() {
                pipeline.stop();
            }
        }
    });
}

/// Errors surfacing from the preview pipeline. Mapped to user-
/// visible toasts by the caller via
/// [`settings::surface_error`](crate::settings::surface_error).
#[derive(Debug, thiserror::Error)]
pub enum PreviewError {
    /// `GStreamer` reported a missing element (e.g. `v4l2src` or
    /// `gtk4paintablesink` plugin not installed). The error string
    /// names the element so the user can `apt install` the
    /// matching plugin package.
    #[error("GStreamer element {0:?} not installed")]
    MissingElement(String),
    /// The pipeline could not transition to PLAYING — typically the
    /// camera is in use by another app. The wrapped string is the
    /// `GStreamer` error message.
    #[error("preview pipeline failed to start: {0}")]
    PipelineStart(String),
    /// `gst::init()` failed at startup. Very rare; usually means a
    /// broken `GStreamer` install.
    #[error("GStreamer init failed: {0}")]
    GstInit(String),
}

/// Owned handle to the preview pipeline. Holds the `gst::Pipeline`,
/// the `gtk4paintablesink` element, and the snapshot paintable so
/// the GUI can bind to it once and keep the same `gtk::Picture`
/// across start/stop cycles.
pub struct PreviewPipeline {
    pipeline: gst::Pipeline,
    /// Cached paintable from `gtk4paintablesink`; never changes for
    /// the lifetime of `self`, so callers can bind it to a
    /// `gtk::Picture` once at construction.
    paintable: gtk::gdk::Paintable,
    /// Whether the pipeline is currently in PLAYING. Avoids a
    /// double-start that `GStreamer` would otherwise warn about.
    is_playing: bool,
}

impl PreviewPipeline {
    /// Build the pipeline in NULL state. Initializes `GStreamer` if
    /// not already initialized (idempotent — repeated calls are
    /// cheap).
    ///
    /// # Errors
    /// - [`PreviewError::GstInit`] if `gst::init()` fails.
    /// - [`PreviewError::MissingElement`] if `videoconvert` or
    ///   `gtk4paintablesink` are not installed.
    pub fn new() -> Result<Self, PreviewError> {
        gst::init().map_err(|e| PreviewError::GstInit(e.to_string()))?;

        let pipeline = gst::Pipeline::new();

        // We omit `v4l2src` from initial construction — its
        // `device` property is set per `start(path)` call so the
        // pipeline can be reused across cameras without rebuilding.
        // For the paintable handoff we still need the gtk4
        // sink up-front.
        //
        // Why two `videoconvert` elements:
        // - `vc_pre` normalises whatever raw format `v4l2src` is
        //   negotiating (YUYV / NV12 / I420 — varies per UVC build)
        //   into a layout `videobalance` can mutate in place.
        //   Without this the sink sometimes dmabuf-imports the
        //   upstream buffer directly and `videobalance` falls back
        //   to passthrough, so the `saturation` property has no
        //   observable effect (T-202 grayscale silently no-ops).
        //   It also clears the spammy `gst_video_frame_map_id:
        //   assertion 'info->finfo->format == meta->format' failed`
        //   warnings caused by the upstream `GstVideoMeta` not
        //   matching the renegotiated downstream caps.
        // - `vc_post` lets the sink pick whatever format it
        //   prefers (commonly RGBA) without constraining what
        //   `videobalance` runs on.
        let videoconvert_pre = gst::ElementFactory::make("videoconvert")
            .name("vc_pre")
            .build()
            .map_err(|_| PreviewError::MissingElement("videoconvert".to_string()))?;
        // T-202: `videobalance` sits in the pipeline unconditionally
        // with saturation = 1.0 (identity transform — costs only the
        // YUV ↔ YUV passthrough on each frame). Toggling grayscale
        // is a property write on this element, no relink needed.
        let videobalance = gst::ElementFactory::make("videobalance")
            .name("vb_filter")
            .property("saturation", 1.0f64)
            .build()
            .map_err(|_| PreviewError::MissingElement("videobalance".to_string()))?;
        let videoconvert_post = gst::ElementFactory::make("videoconvert")
            .name("vc_post")
            .build()
            .map_err(|_| PreviewError::MissingElement("videoconvert".to_string()))?;
        let sink = gst::ElementFactory::make("gtk4paintablesink")
            .build()
            .map_err(|_| PreviewError::MissingElement("gtk4paintablesink".to_string()))?;

        let paintable: gtk::gdk::Paintable = sink.property::<gtk::gdk::Paintable>("paintable");

        pipeline
            .add_many([&videoconvert_pre, &videobalance, &videoconvert_post, &sink])
            .expect("pipeline.add_many on fresh elements cannot fail");
        gst::Element::link_many([&videoconvert_pre, &videobalance, &videoconvert_post, &sink])
            .expect("vc_pre → videobalance → vc_post → gtk4paintablesink link cannot fail");

        Ok(Self {
            pipeline,
            paintable,
            is_playing: false,
        })
    }

    /// Toggle the T-202 grayscale filter on or off. Implemented as a
    /// saturation property on the always-present `videobalance`
    /// element (1.0 = identity, 0.0 = pure luma). Cheap — no
    /// relinking, no pipeline state change.
    pub fn set_grayscale(&self, on: bool) {
        let saturation = if on { 0.0f64 } else { 1.0f64 };
        if let Some(el) = self.pipeline.by_name("vb_filter") {
            el.set_property("saturation", saturation);
        }
    }

    /// Borrow the paintable that callers bind to a `gtk::Picture`.
    /// Stable for the lifetime of the pipeline; rendering happens
    /// only while `start` has been called and the pipeline is in
    /// PLAYING.
    pub fn paintable(&self) -> gtk::gdk::Paintable {
        self.paintable.clone()
    }

    /// Build and connect a fresh `v4l2src` for `path`, then
    /// transition the pipeline to PLAYING. Idempotent: calling
    /// `start` while already playing is a no-op (does not rebuild
    /// the source).
    ///
    /// # Errors
    /// - [`PreviewError::MissingElement`] if `v4l2src` is missing.
    /// - [`PreviewError::PipelineStart`] if the state transition
    ///   fails — typically the V4L2 device is busy.
    pub fn start(&mut self, path: &Path) -> Result<(), PreviewError> {
        if self.is_playing {
            return Ok(());
        }

        // Build a fresh v4l2src each start so changing cameras
        // mid-session works without leaking the previous source.
        let src = gst::ElementFactory::make("v4l2src")
            .property("device", path.to_string_lossy().as_ref())
            .build()
            .map_err(|_| PreviewError::MissingElement("v4l2src".to_string()))?;

        // Link the new source into the always-present `vc_pre`
        // videoconvert (named in `new()` so we can find it back here
        // without iterating).
        let videoconvert_pre = self
            .pipeline
            .by_name("vc_pre")
            .expect("vc_pre added in new() must still be in the pipeline");

        self.pipeline
            .add(&src)
            .expect("pipeline.add(v4l2src) on a fresh source cannot fail");
        if let Err(e) = src.link(&videoconvert_pre) {
            let _ = self.pipeline.remove(&src);
            return Err(PreviewError::PipelineStart(e.to_string()));
        }

        // `set_state` may return `Async` and only emit the actual
        // device-busy / missing-device error later on the bus. Block
        // on the state transition for up to 2 s so those errors
        // propagate as `Err` instead of getting buried — without
        // this, opening the camera in Cheese / OBS and then toggling
        // our preview on would silently leave the pipeline stuck
        // with no user feedback.
        let _ = self.pipeline.set_state(gst::State::Playing);
        let (result, _, _) = self.pipeline.state(Some(gst::ClockTime::from_seconds(2)));
        if result.is_err() {
            let msg = drain_bus_error(&self.pipeline)
                .unwrap_or_else(|| "device unavailable or busy".to_string());
            let _ = self.pipeline.set_state(gst::State::Null);
            let _ = self.pipeline.remove(&src);
            return Err(PreviewError::PipelineStart(msg));
        }
        self.is_playing = true;
        Ok(())
    }

    /// Transition the pipeline back to NULL so V4L2 releases the
    /// capture node. Idempotent.
    pub fn stop(&mut self) {
        if !self.is_playing {
            return;
        }
        // Best-effort — failure to transition is logged but does
        // not propagate; Drop will retry on the way out.
        if let Err(err) = self.pipeline.set_state(gst::State::Null) {
            eprintln!("preview: failed to stop pipeline cleanly: {err}");
        }
        // Tear the v4l2src out so the next start() builds a fresh
        // one (device path may have changed).
        let mut iter = self.pipeline.iterate_elements();
        let mut to_remove = Vec::new();
        while let Ok(Some(el)) = iter.next() {
            if el.factory().is_some_and(|f| f.name() == "v4l2src") {
                to_remove.push(el);
            }
        }
        for el in to_remove {
            let _ = self.pipeline.remove(&el);
        }
        self.is_playing = false;
    }
}

impl Drop for PreviewPipeline {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Human-readable hint for the preview toggle button's tooltip.
/// Centralized here so the GUI does not duplicate the gettext key.
pub fn toggle_tooltip(active: bool) -> String {
    if active {
        gettext("Stop live preview")
    } else {
        gettext("Start live preview")
    }
}

/// Drain any pending `ERROR` message off the pipeline's bus and
/// return its human-readable representation. Used when
/// `set_state(Playing)` reports an async failure to give the user a
/// meaningful toast instead of the generic state-change-error.
fn drain_bus_error(pipeline: &gst::Pipeline) -> Option<String> {
    let bus = pipeline.bus()?;
    while let Some(msg) = bus.pop() {
        if let gst::MessageView::Error(err) = msg.view() {
            let cause = err.error();
            return Some(match err.debug() {
                Some(dbg) => format!("{cause} ({dbg})"),
                None => format!("{cause}"),
            });
        }
    }
    None
}
