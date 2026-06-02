# SPDX-License-Identifier: GPL-3.0-or-later
#
# Top-level Meson build for Obsbot Cam Control (T-008).
#
# Meson is the orchestrator the GNOME tooling expects (Flatpak's
# `flatpak-builder` invokes it; distro packagers know the pattern).
# Cargo handles the actual Rust compile; this file wires the two
# together and installs the resulting binary at the prefix.
#
# Future tasks extend this file:
#   * T-009 lands .desktop + .metainfo + i18n via `subdir('data')`
#     and `subdir('po')`.
#   * T-010 adds icons.
#   * T-105 (v0.2) compiles the GSettings schema.

project(
  'obsbot-cam-control',
  version: '0.3.2',
  license: 'GPL-3.0-or-later',
  meson_version: '>= 1.0.0',
  default_options: [
    'buildtype=release',
  ],
)

# Project identity (resolved in ADR-0012).
app_id = 'io.github.domatix.ObsbotCamControl'
app_name = 'Obsbot Cam Control'

# Runtime libraries: presence + minimum-version checks per
# ARCHITECTURE §1. These are NOT linked by Meson (cargo does the
# linking via gtk4-sys / libadwaita-sys); we run them here so a
# missing system dependency is caught at `meson setup` time with a
# clear error instead of mid-cargo-build.
dependency('gtk4', version: '>= 4.14')
dependency('libadwaita-1', version: '>= 1.6')
dependency('glib-2.0', version: '>= 2.74')
dependency('gio-2.0', version: '>= 2.74')

# Cargo orchestration.
cargo_bin = find_program('cargo', required: true)
cargo_build_script = find_program('build-aux/cargo-build.sh', required: true)

prefix    = get_option('prefix')
bindir    = prefix / get_option('bindir')
datadir   = prefix / get_option('datadir')
localedir = prefix / get_option('localedir')

# Map meson buildtype → cargo profile.
#
# `plain` lands here as release too: in meson semantics it means
# "no meson-managed flags; respect distro CFLAGS / CARGOFLAGS",
# which for a pure-cargo build aligns with cargo's optimised
# profile. `arch-meson` (T-017) defaults to `--buildtype=plain`,
# so this mapping lets the Arch PKGBUILD ship the release binary
# without a `-Dbuildtype=release` override.
buildtype = get_option('buildtype')
if buildtype == 'release' or buildtype == 'minsize' or buildtype == 'plain'
  rust_profile = 'release'
else
  rust_profile = 'debug'
endif

cargo_target_dir = meson.project_build_root() / 'cargo'
manifest         = meson.project_source_root() / 'Cargo.toml'

# The binary cargo produces and that we install. Its name comes from
# crates/obsbot-gui/Cargo.toml's `[[bin]] name`, fixed by ADR-0012.
bin_name = 'obsbot-cam-control'

# Optional cargo feature flag (T-200 / T-203). When the `live-preview`
# meson option is true the wrapper appends --features live-preview to
# the cargo build invocation. Default off so a plain
# `meson setup builddir && meson install` keeps working on hosts
# without GStreamer dev libs.
live_preview_feature = get_option('live-preview') ? 'live-preview' : ''

cargo_build = custom_target(
  'cargo-build',
  output: bin_name,
  command: [
    cargo_build_script,
    manifest,
    cargo_target_dir,
    rust_profile,
    'obsbot-gui',
    bin_name,
    '@OUTPUT@',
    localedir,
    live_preview_feature,
  ],
  build_always_stale: true,
  build_by_default: true,
  install: true,
  install_dir: bindir,
  console: true,
)

# Data files (.desktop, AppStream metainfo) land in data/ from T-009.
# T-010 icons + T-105 GSettings schema are wired in data/meson.build.
# T-107 lands the gettext scaffolding in po/.
subdir('data')
subdir('po')

summary(
  {
    'App ID'  : app_id,
    'App name': app_name,
    'Profile' : rust_profile,
    'Prefix'  : prefix,
    'Bindir'  : bindir,
  },
  section: 'Configuration',
  bool_yn: true,
)
