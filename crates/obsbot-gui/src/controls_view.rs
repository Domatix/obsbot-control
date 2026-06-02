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

//! V4L2 control detail page.
//!
//! Tapping an `AdwActionRow` in the camera list (see `window.rs`) pushes
//! the page built here onto the parent `AdwNavigationView`. The page
//! calls [`obsbot_core::read_controls`] synchronously on the main thread
//! (~100 ms on the user's hardware) and renders the results as one
//! `AdwPreferencesGroup` per V4L2 class. T-100 makes User-class
//! Integer / Boolean controls writable: integers use an `AdwActionRow`
//! with a `gtk::Scale` (drag-bar) suffix plus a live value label,
//! booleans use `AdwSwitchRow`. Camera-class and menu controls stay
//! read-only until their dedicated write paths (T-101 PTZ pad,
//! T-103 white balance, T-104 exposure, T-300+ vendor XU) land.

use std::path::Path;

use gtk4 as gtk;
use libadwaita as adw;

use adw::prelude::*;
use obsbot_core::{
    read_controls, CameraInfo, ControlClass, ControlDescriptor, ControlKind, ControlValue,
};

use crate::ai_effects_view::build_ai_effects_group;
use crate::exposure_group::{build_exposure_group, EXPOSURE_GROUP_IDS};
use crate::extras_view::build_extras_group;
use crate::i18n::gettext;
#[cfg(feature = "live-preview")]
use crate::preview::{toggle_tooltip, PreviewPipeline};
use crate::ptz_pad::{build_ptz_pad, wire_keyboard_arrows, PTZ_PAD_IDS};
use crate::settings;
use crate::wb_group::{build_wb_group, WB_GROUP_IDS};

/// Path to the controls-view shell inside the embedded `GResource`
/// (see `build.rs` + `resources/controls-view.blp` +
/// `resources/obsbot.gresource.xml`'s prefix).
const CONTROLS_UI: &str = "/io/github/domatix/ObsbotCamControl/controls-view.ui";

/// Optional preview-machinery handle returned by `build_body` /
/// `render_controls`. With the `live-preview` Cargo feature enabled
/// this carries the revealer + lazy pipeline + `/dev/videoN` path
/// so the controls page can pack a header-bar toggle that drives
/// both. With the feature disabled the slot collapses to `()`,
/// keeping the call-graph identical at zero runtime cost.
#[cfg(feature = "live-preview")]
type PreviewSlot = Option<PreviewHandles>;
#[cfg(not(feature = "live-preview"))]
type PreviewSlot = ();

/// Build the detail `AdwNavigationPage` for one camera.
pub fn build_controls_page(cam: &CameraInfo) -> adw::NavigationPage {
    let builder = gtk::Builder::from_resource(CONTROLS_UI);
    let page: adw::NavigationPage = builder
        .object("page")
        .expect("controls-view.ui missing object 'page'");
    let body_slot: adw::Bin = builder
        .object("body_slot")
        .expect("controls-view.ui missing object 'body_slot'");

    page.set_title(&cam.product);
    page.set_tag(Some(&format!("controls-{:04x}-{:04x}", cam.vid, cam.pid)));

    // T-111: reset the sensitivity-refresh row registry before
    // building. Each row builder downstream calls
    // `settings::register_row` so the post-write refresh path can
    // find them.
    settings::reset_row_registry(cam.video_path.clone());

    // T-200: pack the live-preview toggle into the header bar so it
    // is always reachable, while the Picture lives in a `Revealer`
    // above the scrolled page that only opens while the toggle is
    // active. Compiled only with the `live-preview` Cargo feature;
    // a feature-off build leaves the header empty (current
    // behaviour).
    #[cfg(feature = "live-preview")]
    {
        let (body, preview_slot) = build_body(cam);
        body_slot.set_child(Some(&body));
        if let Some(handles) = preview_slot {
            let header_bar: adw::HeaderBar = builder
                .object("header_bar")
                .expect("controls-view.ui missing object 'header_bar'");
            // pack_end stacks right-to-left: the first pack_end ends
            // up rightmost. Final layout:
            //     [<back]   …   [grayscale] [snapshot] [toggle]
            let toggle = build_preview_toggle(&handles);
            let snapshot = build_snapshot_button(&handles);
            let grayscale = build_grayscale_toggle(&handles);
            header_bar.pack_end(&toggle);
            header_bar.pack_end(&snapshot);
            header_bar.pack_end(&grayscale);
        }
    }
    #[cfg(not(feature = "live-preview"))]
    {
        let (body, ()) = build_body(cam);
        body_slot.set_child(Some(&body));
    }

    // T-108 / T-110: the toast surface that backs
    // `settings::surface_error` is a window-level
    // `Adw.ToastOverlay` declared in `window.blp` and bound once in
    // `window::build`. We do NOT bind a fresh per-page overlay here
    // — that would scope toasts to the controls page and lose them
    // on a hot-plug REMOVE that pops the page out from under them.

    page
}

fn build_body(cam: &CameraInfo) -> (gtk::Widget, PreviewSlot) {
    let Some(path) = cam.video_path.as_deref() else {
        return (
            error_status(
                gettext("No video node"),
                gettext("This camera has no /dev/videoN path."),
            )
            .upcast(),
            empty_preview_slot(),
        );
    };

    let initial = match read_controls(path) {
        Ok(controls) if controls.is_empty() => {
            return (
                error_status(
                    gettext("No controls exposed"),
                    gettext("The driver returned an empty control list."),
                )
                .upcast(),
                empty_preview_slot(),
            );
        }
        Ok(controls) => controls,
        Err(err) => {
            return (
                error_status(
                    gettext("Could not read V4L2 controls"),
                    format!("{path:?}: {err}"),
                )
                .upcast(),
                empty_preview_slot(),
            );
        }
    };

    // T-105 — restore saved per-camera values. Best-effort: writes
    // that fail (control inactive, driver mismatch) are logged and
    // skipped; we then re-read the controls so the rendered UI
    // reflects whatever the driver actually accepted.
    let serial = cam.serial.as_deref();
    let controls = restore_saved_values(path, &initial, serial).unwrap_or(initial);

    render_controls(cam, &controls, path, serial)
}

/// Empty `PreviewSlot` for the early-return error paths in
/// `build_body` (no video node, empty control list, read failure).
/// Without the feature the slot is `()` and we just return it; with
/// the feature it is `None` so the header-bar toggle stays off the
/// page.
#[cfg(feature = "live-preview")]
fn empty_preview_slot() -> PreviewSlot {
    None
}
#[cfg(not(feature = "live-preview"))]
fn empty_preview_slot() -> PreviewSlot {}

/// Replay every saved value for this camera, then re-read the V4L2
/// surface so callers can render the restored state. Returns `None`
/// when there are no saved values (caller falls back to the initial
/// read), or when the re-read after replay fails (caller sticks
/// with the pre-replay snapshot).
fn restore_saved_values(
    path: &Path,
    initial: &[ControlDescriptor],
    serial: Option<&str>,
) -> Option<Vec<ControlDescriptor>> {
    let serial = serial?;
    let saved = settings::load_for_camera(serial);
    if saved.is_empty() {
        return None;
    }

    for ctrl in initial {
        let Some(stored) = saved.get(&ctrl.name) else {
            continue;
        };
        let value = match &ctrl.kind {
            ControlKind::Integer { .. } => ControlValue::Integer(i64::from(*stored)),
            ControlKind::Boolean { .. } => ControlValue::Boolean(*stored != 0),
            ControlKind::Menu { .. } => ControlValue::Menu(i64::from(*stored)),
            _ => continue,
        };
        settings::write_and_save(path, ctrl.id, value, Some(serial), &ctrl.name);
    }

    read_controls(path).ok()
}

/// Build the controls body plus, when the `live-preview` feature
/// is enabled, the handles the header-bar toggle binds to. The
/// outer `gtk::Box` stacks (1) the sticky preview revealer and (2)
/// the scrollable `PreferencesPage`. With the feature off the
/// box only holds the page so the layout matches the pre-T-200
/// behaviour.
fn render_controls(
    cam: &CameraInfo,
    controls: &[ControlDescriptor],
    path: &Path,
    serial: Option<&str>,
) -> (gtk::Widget, PreviewSlot) {
    let outer = gtk::Box::new(gtk::Orientation::Vertical, 0);

    #[cfg(feature = "live-preview")]
    let preview_slot: PreviewSlot = {
        let (banner, revealer, handles) = build_preview_widgets(path);
        outer.append(&banner);
        outer.append(&revealer);
        Some(handles)
    };
    #[cfg(not(feature = "live-preview"))]
    let preview_slot: PreviewSlot = ();

    let page = adw::PreferencesPage::new();
    page.set_vexpand(true);

    // Curated groups (AI & Effects, PTZ pad, White balance,
    // Exposure) consume a specific subset of controls. Mount them
    // at the top of the page and filter the consumed IDs out of
    // the generic per-class render below.

    // AI & Effects (T-301) is the marquee v0.3 feature so it goes
    // first; the controls it surfaces are XU-only and don't overlap
    // with any V4L2 standard ID, so no filter list to merge.
    if let Some(ai_group) = build_ai_effects_group(cam) {
        page.add(&ai_group);
    }
    if let Some(extras_group) = build_extras_group(cam) {
        page.add(&extras_group);
    }
    if let Some(ptz_group) = build_ptz_pad(controls, path, serial) {
        page.add(&ptz_group);
    }
    if let Some(exposure_group) = build_exposure_group(controls, path, serial) {
        page.add(&exposure_group);
    }
    if let Some(wb_group) = build_wb_group(controls, path, serial) {
        page.add(&wb_group);
    }

    let mut user_group: Option<adw::PreferencesGroup> = None;
    let mut camera_group: Option<adw::PreferencesGroup> = None;
    let mut other_group: Option<adw::PreferencesGroup> = None;

    for ctrl in controls {
        if PTZ_PAD_IDS.contains(&ctrl.id)
            || WB_GROUP_IDS.contains(&ctrl.id)
            || EXPOSURE_GROUP_IDS.contains(&ctrl.id)
        {
            continue;
        }
        let group = match ctrl.class {
            ControlClass::User => {
                user_group.get_or_insert_with(|| make_group(&gettext("User Controls")))
            }
            ControlClass::Camera => {
                camera_group.get_or_insert_with(|| make_group(&gettext("Camera Controls")))
            }
            _ => other_group.get_or_insert_with(|| make_group(&gettext("Other"))),
        };
        let row = control_row(ctrl, path, serial);
        // Generic INACTIVE grey-out — covers WB Temperature while WB
        // Auto is on, Exposure Time while Auto Exposure is engaged, etc.
        row.set_sensitive(ctrl.is_active);
        settings::register_row(ctrl.id, &row);
        group.add(&row);
    }

    for group in [&user_group, &camera_group, &other_group]
        .into_iter()
        .flatten()
    {
        page.add(group);
    }

    outer.append(&page);

    // Arrow-key + Home navigation of the PTZ (T-101b; one keypress =
    // one step since T-101d). Attached to the outer `Box` so any
    // descendant with focus bubbles unhandled keys up to the
    // controller (focused sliders still consume their own arrows).
    // Skips quietly when the camera does not advertise pan / tilt.
    wire_keyboard_arrows(&outer, controls, path, serial);

    (outer.upcast(), preview_slot)
}

fn make_group(title: &str) -> adw::PreferencesGroup {
    adw::PreferencesGroup::builder().title(title).build()
}

/// Handles for the live-preview machinery (T-200) — the
/// `gtk::Revealer` that hosts the Picture and the `Rc<RefCell>`
/// holding the lazy-built pipeline. Owned by `build_preview_toggle`
/// once the header-bar toggle is wired; the toggle's
/// `connect_toggled` closure drives both the revealer (sticky
/// preview slot collapses when off) and the pipeline (NULL when
/// off, PLAYING when on).
#[cfg(feature = "live-preview")]
struct PreviewHandles {
    /// Reveals the sticky Picture above the scrollable
    /// `PreferencesPage`. `reveal_child = false` when the toggle is
    /// off so the page reclaims the vertical space.
    revealer: gtk::Revealer,
    /// `gtk::Picture` bound to the `gtk4paintablesink` paintable on
    /// first `PreviewPipeline::new` success; rebinding stays cheap
    /// after that because the paintable is stable for the
    /// pipeline's lifetime.
    picture: gtk::Picture,
    /// The pipeline itself, lazily constructed so opening the
    /// controls page does not pay the `GStreamer` init cost.
    pipeline: std::rc::Rc<std::cell::RefCell<Option<PreviewPipeline>>>,
    /// `/dev/videoN` path owned by value so the closure can outlive
    /// the borrowed `&Path` passed into `render_controls`.
    path: std::path::PathBuf,
    /// Discoverability banner pinned under the header bar while the
    /// preview is off. Hosts a "Start" button that activates the
    /// header-bar toggle; collapses the moment the toggle goes
    /// active so we do not double-announce the feature once the
    /// user has found it.
    banner: adw::Banner,
}

/// Build the sticky preview slot: a discoverability banner plus
/// the `Revealer` that hosts the Picture. The caller appends both
/// to its outer `Box` above the scrollable `PreferencesPage`. The
/// revealer starts hidden so the page is scrollable end-to-end
/// whenever the preview is off; the banner starts revealed so a
/// new user notices the feature exists.
#[cfg(feature = "live-preview")]
fn build_preview_widgets(path: &Path) -> (adw::Banner, gtk::Revealer, PreviewHandles) {
    use std::cell::RefCell;
    use std::rc::Rc;

    let picture = gtk::Picture::builder()
        .content_fit(gtk::ContentFit::Contain)
        // T-204: 192 px = 240 × 0.8, a 20% cut so the feed reads as a
        // banner above the controls rather than the page's centre of
        // gravity. `Contain` letterboxes the frame without distortion.
        .height_request(192)
        .build();

    // Clamp matches `AdwPreferencesPage`'s built-in 600 px clamp
    // so the preview lines up with the groups in the scrolled
    // page below.
    let clamp = adw::Clamp::builder()
        .maximum_size(600)
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .margin_bottom(12)
        .child(&picture)
        .build();

    let revealer = gtk::Revealer::builder()
        .transition_type(gtk::RevealerTransitionType::SlideDown)
        .transition_duration(200)
        .reveal_child(false)
        .child(&clamp)
        .build();

    let banner = adw::Banner::builder()
        .title(gettext(
            "Live preview is available — show the camera feed inside the app.",
        ))
        .button_label(gettext("Show preview"))
        .revealed(true)
        .build();

    let handles = PreviewHandles {
        revealer: revealer.clone(),
        picture,
        pipeline: Rc::new(RefCell::new(None)),
        path: path.to_path_buf(),
        banner: banner.clone(),
    };

    (banner, revealer, handles)
}

/// Wire a `gtk::ToggleButton` to the preview revealer + pipeline.
/// The caller packs this into the header bar of the controls page.
/// Honours the `preview-default-on` `GSettings` key — when true,
/// emits `toggled` once at construction so the pipeline starts and
/// the revealer opens on first render.
#[cfg(feature = "live-preview")]
fn build_preview_toggle(handles: &PreviewHandles) -> gtk::ToggleButton {
    let toggle = gtk::ToggleButton::builder()
        .icon_name("camera-video-symbolic")
        .tooltip_text(toggle_tooltip(false))
        .build();

    // The banner's "Show preview" button is an alternative entry
    // point for users who notice the banner before the icon. It
    // flips the toggle, which dispatches the existing toggled
    // closure below — keeping all start/stop logic in one place.
    {
        let toggle_weak = toggle.downgrade();
        handles.banner.connect_button_clicked(move |_| {
            if let Some(toggle) = toggle_weak.upgrade() {
                toggle.set_active(true);
            }
        });
    }

    let default_on = settings::preview_default_on();
    toggle.set_active(default_on);

    let banner = handles.banner.clone();
    let revealer = handles.revealer.clone();
    let pipeline = handles.pipeline.clone();
    let picture = handles.picture.clone();
    let path = handles.path.clone();
    toggle.connect_toggled(move |btn| {
        let active = btn.is_active();
        btn.set_tooltip_text(Some(&toggle_tooltip(active)));
        revealer.set_reveal_child(active);
        // Hide the discoverability banner while the preview is on
        // (the user has clearly found it); bring it back when the
        // preview is off so it remains a visual anchor.
        banner.set_revealed(!active);

        if active {
            let mut slot = pipeline.borrow_mut();
            if slot.is_none() {
                match PreviewPipeline::new() {
                    Ok(p) => {
                        picture.set_paintable(Some(&p.paintable()));
                        *slot = Some(p);
                    }
                    Err(err) => {
                        settings::surface_error(&format!(
                            "{}: {err}",
                            gettext("Could not initialize preview")
                        ));
                        btn.set_active(false);
                        return;
                    }
                }
            }
            if let Some(p) = slot.as_mut() {
                if let Err(err) = p.start(&path) {
                    settings::surface_error(&format!(
                        "{}: {err}",
                        gettext("Could not start preview")
                    ));
                    btn.set_active(false);
                }
            }
        } else if let Some(p) = pipeline.borrow_mut().as_mut() {
            p.stop();
        }
    });

    if default_on {
        toggle.emit_by_name::<()>("toggled", &[]);
    }

    toggle
}

/// Grayscale-filter toggle (T-202). Flips the `videobalance`
/// `saturation` property on the live pipeline between 1.0 (color)
/// and 0.0 (grayscale). Cheap — no pipeline state change, no
/// relink. State persists per page lifetime; closing and reopening
/// the page resets the toggle to off.
#[cfg(feature = "live-preview")]
fn build_grayscale_toggle(handles: &PreviewHandles) -> gtk::ToggleButton {
    let btn = gtk::ToggleButton::builder()
        .icon_name("view-reveal-symbolic")
        .tooltip_text(gettext("Toggle grayscale filter"))
        .build();
    let pipeline = handles.pipeline.clone();
    btn.connect_toggled(move |btn| {
        let on = btn.is_active();
        if let Some(p) = pipeline.borrow().as_ref() {
            p.set_grayscale(on);
        }
    });
    btn
}

/// Snapshot button (T-201). Captures the latest paintable into a
/// `gdk::Texture` via `GskRenderer::render_texture` and writes it
/// to `~/Pictures/obsbot-camera-<timestamp>.png`. Falls back to
/// `~/` if the user's `Pictures` XDG dir is missing. Surfaces both
/// success and failure via the toast overlay so the user gets
/// confirmation without a modal dialog.
#[cfg(feature = "live-preview")]
fn build_snapshot_button(handles: &PreviewHandles) -> gtk::Button {
    let btn = gtk::Button::builder()
        .icon_name("camera-photo-symbolic")
        .tooltip_text(gettext("Save snapshot to Pictures"))
        .build();
    let picture = handles.picture.clone();
    btn.connect_clicked(move |_| match save_snapshot(&picture) {
        Ok(path) => {
            settings::surface_error(&format!(
                "{}: {}",
                gettext("Snapshot saved"),
                path.display()
            ));
        }
        Err(msg) => {
            settings::surface_error(&format!("{}: {msg}", gettext("Snapshot failed")));
        }
    });
    btn
}

/// Render the latest frame from the gtk4paintablesink's paintable
/// into a `gdk::Texture` and save it as PNG. Returns the saved
/// path. The paintable is the same one bound to the `gtk::Picture`
/// (stable for the pipeline's lifetime); pulling a render-node
/// snapshot is the gtk4-native equivalent of pulling an appsink
/// buffer and avoids a second pipeline branch.
///
/// Failure modes:
/// - `intrinsic_width` / `intrinsic_height` are zero or negative
///   (preview is off or has not produced a first frame yet) →
///   `Err("No frame available")`.
/// - `gtk::Snapshot::to_node()` returns `None` (paintable rendered
///   an empty subtree, same idea as above) → `Err`.
/// - `Native::renderer()` returns `None` (widget not yet realized;
///   should not happen after the page is on-screen, but covered).
/// - `gdk::Texture::save_to_png` IO failure → propagates.
#[cfg(feature = "live-preview")]
fn save_snapshot(picture: &gtk::Picture) -> Result<std::path::PathBuf, String> {
    use gtk::prelude::*;

    let paintable = picture
        .paintable()
        .ok_or_else(|| "preview pipeline has no paintable".to_string())?;
    let width = paintable.intrinsic_width();
    let height = paintable.intrinsic_height();
    if width <= 0 || height <= 0 {
        return Err(gettext("No frame available — start the preview first"));
    }
    let snapshot = gtk::Snapshot::new();
    paintable.snapshot(
        snapshot.upcast_ref::<gtk::gdk::Snapshot>(),
        f64::from(width),
        f64::from(height),
    );
    let node = snapshot
        .to_node()
        .ok_or_else(|| gettext("Preview produced no frame yet"))?;
    let renderer = picture
        .native()
        .and_then(|n| n.renderer())
        .ok_or_else(|| "no GskRenderer available on this surface".to_string())?;
    #[allow(clippy::cast_precision_loss)]
    let bounds = gtk::graphene::Rect::new(0.0, 0.0, width as f32, height as f32);
    let texture = renderer.render_texture(node, Some(&bounds));

    let dir = glib::user_special_dir(glib::UserDirectory::Pictures).unwrap_or_else(glib::home_dir);
    let stamp = glib::DateTime::now_local()
        .ok()
        .and_then(|d| d.format("%Y%m%d-%H%M%S").ok())
        .map_or_else(|| "snapshot".to_string(), |s| s.to_string());
    let path = dir.join(format!("obsbot-camera-{stamp}.png"));
    texture.save_to_png(&path).map_err(|e| e.to_string())?;
    Ok(path)
}

/// Build a row for one control. Integer controls get an
/// `AdwActionRow` with a [`gtk::Scale`] (drag-bar), a [`gtk::SpinButton`]
/// (precise manual entry), and a reset-to-default button as suffixes;
/// Boolean controls get an [`AdwSwitchRow`]; Menu controls get an
/// [`adw::ComboRow`]. Every other shape stays a read-only
/// [`AdwActionRow`].
///
/// The User + Camera classes both go through this writable path
/// (T-104 generalized from T-102's User-only scope so the
/// `exposure_group` can render `auto_exposure` (Camera-class menu)
/// without ad-hoc code). The `Other` class stays read-only — we have
/// no hardware-tested write semantics for codec / image-source
/// controls.
///
/// `pub(crate)` so sibling group modules (`wb_group`, `exposure_group`)
/// can reuse the exact same widgets inside their dedicated groups
/// without duplicating any of the T-100 / T-102 widget builders.
pub(crate) fn control_row(
    ctrl: &ControlDescriptor,
    path: &Path,
    serial: Option<&str>,
) -> gtk::Widget {
    if matches!(ctrl.class, ControlClass::User | ControlClass::Camera) {
        match &ctrl.kind {
            ControlKind::Integer {
                current,
                min,
                max,
                step,
                default,
            } => {
                return integer_scale_row(ctrl, *current, *min, *max, *step, *default, path, serial)
                    .upcast()
            }
            ControlKind::Boolean { current, default } => {
                return boolean_switch_row(ctrl, *current, *default, path, serial).upcast();
            }
            ControlKind::Menu {
                current, options, ..
            } => {
                return menu_combo_row(ctrl, *current, options, path, serial).upcast();
            }
            // `ControlKind::Other(_)` and any future `#[non_exhaustive]`
            // variants fall through to the read-only renderer below.
            _ => {}
        }
    }
    readonly_action_row(ctrl).upcast()
}

#[allow(
    clippy::too_many_arguments,
    reason = "all values come straight from ControlKind::Integer"
)]
fn integer_scale_row(
    ctrl: &ControlDescriptor,
    current: i64,
    min: i64,
    max: i64,
    step: u64,
    default: i64,
    path: &Path,
    serial: Option<&str>,
) -> adw::ActionRow {
    // V4L2 standard User-class Integer controls store values as
    // `__s32` (see `struct v4l2_control` in `linux/videodev2.h`).
    // `ControlKind::Integer` widens to i64 to also cover the rarer
    // `V4L2_CTRL_TYPE_INTEGER64`, but the User-class branch we are in
    // here only ever sees s32-shaped values, so the conversions below
    // are lossless in practice. Clamp via `clamp_i64_to_i32` so an
    // out-of-spec driver can not panic the UI thread.
    let current_i32 = clamp_i64_to_i32(current);
    let min_i32 = clamp_i64_to_i32(min);
    let max_i32 = clamp_i64_to_i32(max);
    let default_i32 = clamp_i64_to_i32(default);
    // V4L2 step is positive by construction; clamp to ≥1 so the
    // Adjustment never receives a zero step_increment.
    let step_u32 = u32::try_from(step.max(1)).unwrap_or(u32::MAX);

    let adjustment = gtk::Adjustment::new(
        f64::from(current_i32),
        f64::from(min_i32),
        f64::from(max_i32),
        f64::from(step_u32),
        f64::from(step_u32), // page increment matches step — no PageUp/Down jumps yet
        0.0, // page_size = 0 keeps `value <= upper` (not `value <= upper - page_size`)
    );

    let scale = gtk::Scale::builder()
        .orientation(gtk::Orientation::Horizontal)
        .adjustment(&adjustment)
        .draw_value(false)
        .hexpand(true)
        .width_request(200)
        .valign(gtk::Align::Center)
        .build();
    scale.set_round_digits(0);
    // Mark default position so users can see where the "reset" sits.
    scale.add_mark(f64::from(default_i32), gtk::PositionType::Bottom, None);

    let spin_button = gtk::SpinButton::builder()
        .adjustment(&adjustment)
        .climb_rate(f64::from(step_u32))
        .digits(0)
        .numeric(true)
        .width_chars(5)
        .valign(gtk::Align::Center)
        .build();

    let reset_button = gtk::Button::builder()
        .icon_name("edit-undo-symbolic")
        .tooltip_text(format!("Reset to default ({default_i32})"))
        .valign(gtk::Align::Center)
        .css_classes(vec!["flat"])
        .build();
    {
        let adj = adjustment.clone();
        let reset_to = f64::from(default_i32);
        reset_button.connect_clicked(move |_| {
            adj.set_value(reset_to);
        });
    }

    let row = adw::ActionRow::builder()
        .title(&ctrl.name)
        .subtitle(format!(
            "range {min}..={max} step {step} · default {default_i32}"
        ))
        .activatable(false)
        .build();
    row.add_suffix(&scale);
    row.add_suffix(&spin_button);
    row.add_suffix(&reset_button);

    let id = ctrl.id;
    let name = ctrl.name.clone();
    let owned_path = path.to_path_buf();
    let owned_serial = serial.map(str::to_owned);
    adjustment.connect_value_changed(move |adj| {
        let value = f64_to_i32_saturating(adj.value().round());
        settings::write_and_save(
            &owned_path,
            id,
            ControlValue::Integer(i64::from(value)),
            owned_serial.as_deref(),
            &name,
        );
    });

    row
}

/// Saturating-clamp an `i64` to `i32`. Used to project V4L2 control
/// values into the `gtk::Adjustment` (f64) domain without precision
/// loss: standard V4L2 control values are `__s32` so this is lossless
/// for every well-behaved driver, and saturates for the pathological
/// case rather than panicking.
fn clamp_i64_to_i32(v: i64) -> i32 {
    if v > i64::from(i32::MAX) {
        i32::MAX
    } else if v < i64::from(i32::MIN) {
        i32::MIN
    } else {
        // try_from is infallible here (range checked just above), but
        // we keep the conversion explicit to stay clippy-clean.
        i32::try_from(v).unwrap_or(0)
    }
}

/// Saturating `f64 → i32` for slider read-back. Rust's `as i32` from
/// float saturates by spec (since 1.45), but clippy flags the cast as
/// a possible truncation; this wrapper documents intent and keeps the
/// callsite annotation-free.
#[allow(
    clippy::cast_possible_truncation,
    reason = "saturation is intentional: GtkAdjustment already clamps to [min, max]"
)]
fn f64_to_i32_saturating(v: f64) -> i32 {
    v as i32
}

fn boolean_switch_row(
    ctrl: &ControlDescriptor,
    current: bool,
    default: bool,
    path: &Path,
    serial: Option<&str>,
) -> adw::SwitchRow {
    let row = adw::SwitchRow::builder()
        .title(&ctrl.name)
        .subtitle(if default { "default On" } else { "default Off" })
        .active(current)
        .build();

    let id = ctrl.id;
    let name = ctrl.name.clone();
    let owned_path = path.to_path_buf();
    let owned_serial = serial.map(str::to_owned);
    row.connect_active_notify(move |row| {
        settings::write_and_save(
            &owned_path,
            id,
            ControlValue::Boolean(row.is_active()),
            owned_serial.as_deref(),
            &name,
        );
    });

    row
}

fn readonly_action_row(ctrl: &ControlDescriptor) -> adw::ActionRow {
    let subtitle = match &ctrl.kind {
        ControlKind::Integer {
            current,
            min,
            max,
            step,
            ..
        } => format!("{current} · range {min}..={max} step {step}"),
        ControlKind::Boolean { current, .. } => {
            if *current {
                gettext("Yes")
            } else {
                gettext("No")
            }
        }
        ControlKind::Menu {
            current, options, ..
        } => {
            let label = options
                .iter()
                .find(|(id, _)| *id == *current)
                .map_or("(unknown)", |(_, l)| l.as_str());
            format!("{label} · {} options", options.len())
        }
        ControlKind::Other(type_name) => format!("({type_name})"),
        _ => String::from("(unsupported)"),
    };

    adw::ActionRow::builder()
        .title(&ctrl.name)
        .subtitle(&subtitle)
        .build()
}

fn menu_combo_row(
    ctrl: &ControlDescriptor,
    current: i64,
    options: &[(i64, String)],
    path: &Path,
    serial: Option<&str>,
) -> adw::ComboRow {
    let labels: Vec<&str> = options.iter().map(|(_, label)| label.as_str()).collect();
    let model = gtk::StringList::new(&labels);

    let selected = options
        .iter()
        .position(|(id, _)| *id == current)
        .and_then(|i| u32::try_from(i).ok())
        .unwrap_or(0);

    let row = adw::ComboRow::builder()
        .title(&ctrl.name)
        .model(&model)
        .selected(selected)
        .build();

    let id = ctrl.id;
    let name = ctrl.name.clone();
    let option_ids: Vec<i64> = options.iter().map(|(menu_id, _)| *menu_id).collect();
    let owned_path = path.to_path_buf();
    let owned_serial = serial.map(str::to_owned);
    row.connect_selected_notify(move |row| {
        let Ok(idx) = usize::try_from(row.selected()) else {
            return;
        };
        let Some(menu_id) = option_ids.get(idx).copied() else {
            return;
        };
        settings::write_and_save(
            &owned_path,
            id,
            ControlValue::Menu(menu_id),
            owned_serial.as_deref(),
            &name,
        );
    });

    row
}

fn error_status(title: String, description: String) -> adw::StatusPage {
    adw::StatusPage::builder()
        .icon_name("dialog-error-symbolic")
        .title(title)
        .description(description)
        .build()
}
