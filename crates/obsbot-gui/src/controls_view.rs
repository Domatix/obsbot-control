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

    // T-212: the body is a tabbed `AdwViewStack` (Image · Move · AI ·
    // Extras) with the live-preview card pinned above it. Mount it,
    // then pin an `AdwViewSwitcher` in the header bar as the title
    // widget so the tabs are reachable. Error bodies (no video node,
    // empty control list) carry no stack and leave the header plain.
    let (body, preview_slot, view_stack) = build_body(cam);
    body_slot.set_child(Some(&body));

    let header_bar: adw::HeaderBar = builder
        .object("header_bar")
        .expect("controls-view.ui missing object 'header_bar'");

    if let Some(stack) = view_stack.as_ref() {
        let switcher = adw::ViewSwitcher::builder()
            .stack(stack)
            .policy(adw::ViewSwitcherPolicy::Wide)
            .build();
        header_bar.set_title_widget(Some(&switcher));
    }

    // T-200/T-210: pack the live-preview controls into the header bar.
    // pack_end stacks right-to-left, so the first pack_end ends up
    // rightmost. Final layout:
    //   [<back]  <ViewSwitcher>  …  [toggle][snapshot][mirror][grayscale]
    // Compiled only with the `live-preview` Cargo feature; a feature-off
    // build keeps the header switcher-only.
    #[cfg(feature = "live-preview")]
    if let Some(handles) = preview_slot {
        let toggle = build_preview_toggle(&handles);
        let snapshot = build_snapshot_button(&handles);
        let mirror = build_mirror_toggle(&handles);
        let grayscale = build_grayscale_toggle(&handles);
        header_bar.pack_end(&toggle);
        header_bar.pack_end(&snapshot);
        header_bar.pack_end(&mirror);
        header_bar.pack_end(&grayscale);

        // T-207: release the camera deterministically when the user
        // navigates away. `AdwNavigationPage::hidden` fires on pop
        // (back button) and on the T-110 `pop_to_tag` after a hot-plug
        // REMOVE — both cases where the preview must stop even though
        // the page widget (and its `Drop`) may linger in the
        // NavigationView's transition/cache for a while. Also register
        // this slot so the window's close handler can stop it (see
        // `window::build`).
        crate::preview::register_active(&handles.pipeline);
        let pipeline = handles.pipeline.clone();
        page.connect_hidden(move |_| {
            if let Some(p) = pipeline.borrow_mut().as_mut() {
                p.stop();
            }
        });
    }
    #[cfg(not(feature = "live-preview"))]
    let () = preview_slot;

    // T-108 / T-110: the toast surface that backs
    // `settings::surface_error` is a window-level
    // `Adw.ToastOverlay` declared in `window.blp` and bound once in
    // `window::build`. We do NOT bind a fresh per-page overlay here
    // — that would scope toasts to the controls page and lose them
    // on a hot-plug REMOVE that pops the page out from under them.

    page
}

fn build_body(cam: &CameraInfo) -> (gtk::Widget, PreviewSlot, Option<adw::ViewStack>) {
    let Some(path) = cam.video_path.as_deref() else {
        return (
            error_status(
                gettext("No video node"),
                gettext("This camera has no /dev/videoN path."),
            )
            .upcast(),
            empty_preview_slot(),
            None,
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
                None,
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
                None,
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

/// Build the controls body: the live-preview card (when the
/// `live-preview` feature is on) pinned above an `AdwViewStack` whose
/// pages group the controls into tabs — Image · Move · AI · Extras
/// (T-212). Returns the outer widget, the preview handles the
/// header-bar buttons bind to, and the `AdwViewStack` so
/// `build_controls_page` can drive an `AdwViewSwitcher` from the
/// header. Empty tabs (groups the camera does not advertise) are never
/// added. No control loses its wiring: the exact same group / row
/// builders run as before — they are merely distributed across tabs
/// instead of one long scrolling page.
fn render_controls(
    cam: &CameraInfo,
    controls: &[ControlDescriptor],
    path: &Path,
    serial: Option<&str>,
) -> (gtk::Widget, PreviewSlot, Option<adw::ViewStack>) {
    let outer = gtk::Box::new(gtk::Orientation::Vertical, 0);

    #[cfg(feature = "live-preview")]
    let preview_slot: PreviewSlot = {
        let (card, handles) = build_preview_widgets(path);
        outer.append(&card);
        Some(handles)
    };
    #[cfg(not(feature = "live-preview"))]
    let preview_slot: PreviewSlot = ();

    // Generic per-class groups — everything not consumed by a curated
    // group (PTZ / WB / Exposure). Built first so each lands in the
    // right tab below. Identical filtering + INACTIVE grey-out +
    // row-registry wiring as before.
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

    let stack = adw::ViewStack::new();
    stack.set_vexpand(true);

    // Image — white balance, exposure, and the generic User-class
    // sliders (brightness/contrast/saturation/hue/…).
    let mut image_groups: Vec<adw::PreferencesGroup> = Vec::new();
    if let Some(wb) = build_wb_group(controls, path, serial) {
        image_groups.push(wb);
    }
    if let Some(exposure) = build_exposure_group(controls, path, serial) {
        image_groups.push(exposure);
    }
    if let Some(g) = user_group {
        image_groups.push(g);
    }
    add_tab(
        &stack,
        "image",
        &gettext("Image"),
        "applications-graphics-symbolic",
        &image_groups,
    );

    // Move — the PTZ pad (pan/tilt/zoom/focus).
    let mut move_groups: Vec<adw::PreferencesGroup> = Vec::new();
    if let Some(ptz) = build_ptz_pad(controls, path, serial) {
        move_groups.push(ptz);
    }
    add_tab(
        &stack,
        "move",
        &gettext("Move"),
        "find-location-symbolic",
        &move_groups,
    );

    // AI — the marquee auto-framing / HDR / FOV vendor-XU group.
    let mut ai_groups: Vec<adw::PreferencesGroup> = Vec::new();
    if let Some(ai) = build_ai_effects_group(cam) {
        ai_groups.push(ai);
    }
    add_tab(
        &stack,
        "ai",
        &gettext("AI"),
        "applications-science-symbolic",
        &ai_groups,
    );

    // Extras — presets plus any remaining Camera / Other-class controls.
    let mut extras_groups: Vec<adw::PreferencesGroup> = Vec::new();
    if let Some(extras) = build_extras_group(cam) {
        extras_groups.push(extras);
    }
    if let Some(g) = camera_group {
        extras_groups.push(g);
    }
    if let Some(g) = other_group {
        extras_groups.push(g);
    }
    add_tab(
        &stack,
        "extras",
        &gettext("Extras"),
        "preferences-other-symbolic",
        &extras_groups,
    );

    outer.append(&stack);

    // Arrow-key + Home navigation of the PTZ (T-101b; one keypress =
    // one step since T-101d). Attached to the outer `Box` so any
    // descendant with focus bubbles unhandled keys up to the
    // controller (focused sliders still consume their own arrows).
    // Works regardless of the visible tab. Skips quietly when the
    // camera does not advertise pan / tilt.
    wire_keyboard_arrows(&outer, controls, path, serial);

    // No tab populated (camera advertised nothing we group) → no
    // switcher in the header.
    let view_stack = (stack.pages().n_items() > 0).then_some(stack);

    (outer.upcast(), preview_slot, view_stack)
}

/// Add `groups` to a fresh scrollable `AdwPreferencesPage` and register
/// it as a titled, icon'd page in `stack`. No-op when `groups` is empty
/// so a tab the camera cannot populate never appears.
fn add_tab(
    stack: &adw::ViewStack,
    name: &str,
    title: &str,
    icon: &str,
    groups: &[adw::PreferencesGroup],
) {
    if groups.is_empty() {
        return;
    }
    let page = adw::PreferencesPage::new();
    page.set_vexpand(true);
    for group in groups {
        page.add(group);
    }
    let stack_page = stack.add_titled(&page, Some(name), title);
    stack_page.set_icon_name(Some(icon));
}

fn make_group(title: &str) -> adw::PreferencesGroup {
    adw::PreferencesGroup::builder().title(title).build()
}

/// Handles for the live-preview machinery (T-200, reshaped in T-212) —
/// the `gtk::Stack` that swaps between the off-state placeholder and
/// the live Picture, plus the `Rc<RefCell>` holding the lazy-built
/// pipeline. The header-bar toggle's `connect_toggled` closure drives
/// both the stack (placeholder ↔ video) and the pipeline (NULL when
/// off, PLAYING when on).
#[cfg(feature = "live-preview")]
struct PreviewHandles {
    /// Swaps the rounded card between its `"off"` placeholder child
    /// and its `"on"` Picture child. The card itself is always
    /// visible (T-212): the video is the centre of gravity of the
    /// page now, not a collapsible strip.
    stack: gtk::Stack,
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
    /// "Start preview" button inside the placeholder — an in-card
    /// entry point that activates the header-bar toggle so all
    /// start/stop logic stays in one closure.
    start_button: gtk::Button,
}

/// Build the prominent preview **card** (T-212): a rounded, shadowed
/// frame (`.preview-card`) holding a `gtk::Stack` that crossfades
/// between an off-state placeholder (camera glyph + "Start preview"
/// pill) and the live `gtk::Picture`. Returned widget is a clamped
/// card the caller pins at the top of the controls body; it stays
/// visible whether or not the preview is running.
#[cfg(feature = "live-preview")]
fn build_preview_widgets(path: &Path) -> (gtk::Widget, PreviewHandles) {
    use std::cell::RefCell;
    use std::rc::Rc;

    let picture = gtk::Picture::builder()
        .content_fit(gtk::ContentFit::Contain)
        .height_request(260)
        .build();
    picture.add_css_class("preview-video");

    // Off-state placeholder: a soft accent panel with a big camera
    // glyph and a Start button, so the card never looks broken/empty.
    let placeholder = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .height_request(260)
        .build();
    placeholder.add_css_class("preview-placeholder");

    let ph_icon = gtk::Image::from_icon_name("camera-video-symbolic");
    ph_icon.set_pixel_size(56);
    let ph_label = gtk::Label::new(Some(&gettext("Live preview is off")));
    ph_label.add_css_class("dim-label");
    let start_button = gtk::Button::builder()
        .label(gettext("Start preview"))
        .halign(gtk::Align::Center)
        .css_classes(vec!["pill".to_string(), "suggested-action".to_string()])
        .build();
    placeholder.append(&ph_icon);
    placeholder.append(&ph_label);
    placeholder.append(&start_button);

    let stack = gtk::Stack::builder()
        .transition_type(gtk::StackTransitionType::Crossfade)
        .transition_duration(200)
        .build();
    stack.add_named(&placeholder, Some("off"));
    stack.add_named(&picture, Some("on"));
    stack.set_visible_child_name("off");

    // The rounded, shadowed card. `overflow: hidden` clips the inner
    // stack to the CSS corner radius; the card's own box-shadow is
    // painted outside that clip so it still shows.
    let card = gtk::Box::new(gtk::Orientation::Vertical, 0);
    card.add_css_class("preview-card");
    card.set_overflow(gtk::Overflow::Hidden);
    card.append(&stack);

    let clamp = adw::Clamp::builder()
        .maximum_size(640)
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .margin_bottom(6)
        .child(&card)
        .build();

    let handles = PreviewHandles {
        stack,
        picture,
        pipeline: Rc::new(RefCell::new(None)),
        path: path.to_path_buf(),
        start_button,
    };

    (clamp.upcast(), handles)
}

/// Wire a `gtk::ToggleButton` to the preview card stack + pipeline.
/// The caller packs this into the header bar of the controls page.
/// Honours the `preview-default-on` `GSettings` key — when true,
/// emits `toggled` once at construction so the pipeline starts and
/// the card shows the video on first render.
#[cfg(feature = "live-preview")]
fn build_preview_toggle(handles: &PreviewHandles) -> gtk::ToggleButton {
    let toggle = gtk::ToggleButton::builder()
        .icon_name("camera-video-symbolic")
        .tooltip_text(toggle_tooltip(false))
        .build();

    // The placeholder's "Start preview" button is an in-card entry
    // point: it flips the toggle, which dispatches the toggled closure
    // below — keeping all start/stop logic in one place.
    {
        let toggle_weak = toggle.downgrade();
        handles.start_button.connect_clicked(move |_| {
            if let Some(toggle) = toggle_weak.upgrade() {
                toggle.set_active(true);
            }
        });
    }

    let default_on = settings::preview_default_on();
    toggle.set_active(default_on);

    let stack = handles.stack.clone();
    let pipeline = handles.pipeline.clone();
    let picture = handles.picture.clone();
    let path = handles.path.clone();
    toggle.connect_toggled(move |btn| {
        let active = btn.is_active();
        btn.set_tooltip_text(Some(&toggle_tooltip(active)));

        if active {
            // Lazily build + start the pipeline. `ok` tracks success so
            // we never hold the pipeline borrow across the stack
            // switch or the `set_active(false)` re-entry below.
            let mut ok = true;
            {
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
                            ok = false;
                        }
                    }
                }
                if ok {
                    if let Some(p) = slot.as_mut() {
                        if let Err(err) = p.start(&path) {
                            settings::surface_error(&format!(
                                "{}: {err}",
                                gettext("Could not start preview")
                            ));
                            ok = false;
                        }
                    }
                }
            }
            if ok {
                stack.set_visible_child_name("on");
            } else {
                // Re-enters this closure with active == false, which
                // stops the (already-idle) pipeline and shows "off".
                btn.set_active(false);
            }
        } else {
            if let Some(p) = pipeline.borrow_mut().as_mut() {
                p.stop();
            }
            stack.set_visible_child_name("off");
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
    btn.add_css_class("preview-filter");
    let pipeline = handles.pipeline.clone();
    btn.connect_toggled(move |btn| {
        let on = btn.is_active();
        if let Some(p) = pipeline.borrow().as_ref() {
            p.set_grayscale(on);
        }
    });
    btn
}

/// Mirror-filter toggle (T-210). Flips the `videoflip` `method` on the
/// live pipeline between `none` and `horizontal-flip` so the user sees
/// a natural self-view (right hand on the right). Cheap — no pipeline
/// state change, no relink. Preview-only; resets to off on page
/// reopen, like the grayscale toggle.
#[cfg(feature = "live-preview")]
fn build_mirror_toggle(handles: &PreviewHandles) -> gtk::ToggleButton {
    let btn = gtk::ToggleButton::builder()
        .icon_name("object-flip-horizontal-symbolic")
        .tooltip_text(gettext("Toggle mirror (horizontal flip)"))
        .build();
    btn.add_css_class("preview-filter");
    let pipeline = handles.pipeline.clone();
    btn.connect_toggled(move |btn| {
        let on = btn.is_active();
        if let Some(p) = pipeline.borrow().as_ref() {
            p.set_mirror(on);
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
