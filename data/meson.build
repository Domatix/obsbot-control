# SPDX-License-Identifier: GPL-3.0-or-later
#
# Data files for Obsbot Cam Control (T-009).
#
# Substitutes @APP_ID@ / @VERSION@ in the two .in templates with the
# values defined in the top-level meson.build, installs the resulting
# files at the freedesktop-standard locations, and wires the two
# validators required by T-009's acceptance criteria as `meson test`
# cases so every local and CI run enforces them.
#
# i18n via gettext is intentionally deferred — the templates carry
# no `_Name=` / `<_summary>` markers and no `subdir('po')` is in
# scope here. A later task adds gettext when actual translatable
# strings emerge (e.g. preset labels, control descriptions).

cdata = configuration_data()
cdata.set('APP_ID', app_id)
cdata.set('VERSION', meson.project_version())

desktop_file = configure_file(
  input: '@0@.desktop.in'.format(app_id),
  output: '@0@.desktop'.format(app_id),
  configuration: cdata,
  install: true,
  install_dir: datadir / 'applications',
)

metainfo_file = configure_file(
  input: '@0@.metainfo.xml.in'.format(app_id),
  output: '@0@.metainfo.xml'.format(app_id),
  configuration: cdata,
  install: true,
  install_dir: datadir / 'metainfo',
)

# Acceptance gates from PLAN T-009. `required: false` so that CI
# environments missing either tool degrade to "test skipped" rather
# than "meson setup failed"; on our dev machine and on Flathub
# builders both are present.
appstreamcli = find_program('appstreamcli', required: false)
if appstreamcli.found()
  test(
    'validate-metainfo',
    appstreamcli,
    args: ['validate', '--no-net', '--explain', metainfo_file],
  )
endif

desktop_file_validate = find_program('desktop-file-validate', required: false)
if desktop_file_validate.found()
  test(
    'validate-desktop',
    desktop_file_validate,
    args: [desktop_file],
  )
endif

# Application icons (T-010). The regular SVG is a placeholder Adwaita-
# palette webcam silhouette; the symbolic variant uses `currentColor`
# so GTK can recolor it for sidebars, About dialogs, and notifications.
# Filenames follow GTK's icon-theme convention (basename = App ID for
# the regular, basename = App ID + `-symbolic` for the symbolic).
install_data(
  'icons' / 'scalable' / 'apps' / '@0@.svg'.format(app_id),
  install_dir: datadir / 'icons' / 'hicolor' / 'scalable' / 'apps',
)
install_data(
  'icons' / 'symbolic' / 'apps' / '@0@-symbolic.svg'.format(app_id),
  install_dir: datadir / 'icons' / 'hicolor' / 'symbolic' / 'apps',
)

# Refresh the hicolor cache + the desktop database after install so
# GNOME Shell starts resolving the new icon and `.desktop` entry
# without requiring a session restart.
gnome = import('gnome')
gnome.post_install(
  gtk_update_icon_cache: true,
  update_desktop_database: true,
)
