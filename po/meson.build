# SPDX-License-Identifier: GPL-3.0-or-later
#
# gettext wiring for Obsbot Cam Control (T-107).
#
# Translation domain = the installed binary name (kebab-case of the
# App ID's last segment per ADR-0012). `i18n.gettext()` does three
# jobs:
#   1. Generate `<domain>.pot` from the sources listed in
#      `POTFILES.in` (target name: `obsbot-cam-control-pot`).
#   2. Compile each `<lang>.po` into a `<lang>.gmo` (target name:
#      `obsbot-cam-control-update-po` and `obsbot-cam-control-gmo`).
#   3. Install the `.gmo` files at
#      `<localedir>/<lang>/LC_MESSAGES/<domain>.mo` so the runtime
#      `bindtextdomain` call in `src/i18n.rs` can find them.
#
# Blueprint (.blp) string extraction is deferred to v0.6 polish — the
# `_("...")` markers in *.blp files stay in place but POTFILES.in
# only lists .rs sources for now. The follow-up will add a custom
# target running `blueprint-compiler pot` per .blp and merging the
# fragments into the .pot.

i18n = import('i18n')

i18n.gettext(
  'obsbot-cam-control',
  preset: 'glib',
)
