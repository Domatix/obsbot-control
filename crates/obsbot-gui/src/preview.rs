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
//! gtk4paintablesink` GStreamer pipeline so the controls page can
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
//! crate. The Flatpak manifest enables it once the GStreamer plugin
//! packages land in `flathub.yaml`. See PLAN.md §T-200 for the
//! install incantation on Debian / Arch.
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
//! reads pan/tilt at 50 ms cadence: GStreamer's `v4l2src` holds the
//! capture queue open on its own file descriptor, while
//! `obsbot-core::write_control` opens a separate fd for each
//! ioctl. The kernel uvcvideo driver accepts both descriptors
//! concurrently — confirmed during T-101a hardware validation.

#![cfg(feature = "live-preview")]

use std::path::Path;

use gstreamer as gst;
use gstreamer::prelude::*;
use gtk4 as gtk;

use crate::i18n::gettext;

/// Errors surfacing from the preview pipeline. Mapped to user-
/// visible toasts by the caller via
/// [`settings::surface_error`](crate::settings::surface_error).
#[derive(Debug, thiserror::Error)]
pub enum PreviewError {
    /// GStreamer reported a missing element (e.g. `v4l2src` or
    /// `gtk4paintablesink` plugin not installed). The error string
    /// names the element so the user can `apt install` the
    /// matching plugin package.
    #[error("GStreamer element {0:?} not installed")]
    MissingElement(String),
    /// The pipeline could not transition to PLAYING — typically the
    /// camera is in use by another app. The wrapped string is the
    /// GStreamer error message.
    #[error("preview pipeline failed to start: {0}")]
    PipelineStart(String),
    /// `gst::init()` failed at startup. Very rare; usually means a
    /// broken GStreamer install.
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
    /// double-start that GStreamer would otherwise warn about.
    is_playing: bool,
}

impl PreviewPipeline {
    /// Build the pipeline in NULL state. Initializes GStreamer if
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
        let videoconvert = gst::ElementFactory::make("videoconvert")
            .build()
            .map_err(|_| PreviewError::MissingElement("videoconvert".to_string()))?;
        let sink = gst::ElementFactory::make("gtk4paintablesink")
            .build()
            .map_err(|_| PreviewError::MissingElement("gtk4paintablesink".to_string()))?;

        let paintable: gtk::gdk::Paintable = sink.property::<gtk::gdk::Paintable>("paintable");

        pipeline
            .add_many([&videoconvert, &sink])
            .expect("pipeline.add_many(videoconvert+sink) cannot fail with fresh elements");
        videoconvert
            .link(&sink)
            .expect("videoconvert → gtk4paintablesink link cannot fail");

        Ok(Self {
            pipeline,
            paintable,
            is_playing: false,
        })
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

        // Find the existing videoconvert and link the new source
        // into it. We deliberately do not cache videoconvert as a
        // field because `pipeline.iterate_elements()` is cheap and
        // keeps the surface minimal.
        let videoconvert = self
            .pipeline
            .by_name("videoconvert0")
            .or_else(|| {
                // gst-launch sometimes names anonymous elements
                // differently; fall back to the first videoconvert
                // in the iterator.
                let mut iter = self.pipeline.iterate_elements();
                while let Ok(Some(el)) = iter.next() {
                    if el.factory().is_some_and(|f| f.name() == "videoconvert") {
                        return Some(el);
                    }
                }
                None
            })
            .expect("videoconvert added in new() must still be in the pipeline");

        self.pipeline
            .add(&src)
            .expect("pipeline.add(v4l2src) on a fresh source cannot fail");
        src.link(&videoconvert)
            .map_err(|e| PreviewError::PipelineStart(e.to_string()))?;

        self.pipeline
            .set_state(gst::State::Playing)
            .map_err(|e| PreviewError::PipelineStart(e.to_string()))?;
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
