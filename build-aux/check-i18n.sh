#!/usr/bin/env bash
# SPDX-License-Identifier: GPL-3.0-or-later
#
# Translation coverage check.
#
# The meson `i18n.gettext()` pipeline only extracts strings from the
# Rust sources listed in `po/POTFILES.in`; the Blueprint templates mark
# their strings with `_("...")` but blueprint-compiler 0.22 has no `pot`
# subcommand, so those markers were never turned into .pot entries. This
# script closes that gap by comparing the whole user-visible surface
# (Rust `gettext("...")` calls plus `_("...")` markers in *.blp) against
# the catalogs, and fails when something is missing or left untranslated.
#
# Usage: `build-aux/check-i18n.sh` (no arguments, any directory).

set -euo pipefail

here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/.." && pwd)
cd "$root"

python3 - "$@" <<'PY'
import glob
import re
import sys

catalogs = sorted(glob.glob("po/*.po"))
if not catalogs:
    print("i18n coverage: no catalogs found")
    sys.exit(1)

# Every literal the code can look up.
sources = set()
for path in glob.glob("crates/obsbot-gui/src/*.rs"):
    text = open(path, encoding="utf-8").read()
    sources |= set(re.findall(r'gettext\(\s*"((?:[^"\\]|\\.)*)"', text))
for path in glob.glob("crates/obsbot-gui/resources/*.blp"):
    text = open(path, encoding="utf-8").read()
    sources |= set(re.findall(r'_\("((?:[^"\\]|\\.)*)"\)', text))

failed = False


def parse_entries(text):
    # Parse (msgid, msgstr) pairs, joining continuation lines so
    # wrapped entries (msgmerge wraps lines past ~80 columns) do not
    # fool the coverage check. Escapes are kept verbatim, matching
    # the extraction regexes above.
    entries = {}
    msgid_parts, msgstr_parts = [], []
    mode = None

    def commit():
        if mode == "msgstr" or msgstr_parts:
            key = "".join(msgid_parts)
            if key:
                entries[key] = "".join(msgstr_parts)

    for line in text.splitlines():
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            if stripped.startswith("#,") and "fuzzy" in stripped:
                # A fuzzy entry is not a translation: drop it.
                msgid_parts, msgstr_parts = [], []
                mode = None
            continue
        if stripped.startswith('msgid "'):
            commit()
            msgid_parts, msgstr_parts = [stripped[7:-1]], []
            mode = "msgid"
        elif stripped.startswith('msgstr "'):
            msgstr_parts = [stripped[8:-1]]
            mode = "msgstr"
        elif stripped.startswith('"') and mode:
            (msgid_parts if mode == "msgid" else msgstr_parts).append(stripped[1:-1])
    commit()
    return entries


for catalog in catalogs:
    text = open(catalog, encoding="utf-8").read()
    entries = parse_entries(text)
    missing = sorted(s for s in sources if s not in entries)
    empty = sorted(s for s in sources if s in entries and not entries[s])
    if missing:
        failed = True
        print(f"{catalog}: {len(missing)} string(s) missing from the catalog:")
        for s in missing:
            print(f"  {s!r}")
    if empty:
        failed = True
        print(f"{catalog}: {len(empty)} string(s) with an empty translation:")
        for s in empty:
            print(f"  {s!r}")
    if not missing and not empty:
        print(f"{catalog}: {len(sources)} strings, all translated")

# Toasts must never show a bare literal: either a gettext() template or
# a localised error from i18n_errors.
import glob as _glob
bad = []
for path in _glob.glob("crates/obsbot-gui/src/*.rs"):
    for number, line in enumerate(open(path, encoding="utf-8"), 1):
        stripped = line.strip()
        if re.match(r'surface_error\(&?"', stripped) or re.match(r'surface_error\(&format!\(\s*"', stripped):
            bad.append(f"{path}:{number}: {stripped}")
if bad:
    failed = True
    print("toasts with an untranslated literal:")
    for entry in bad:
        print(f"  {entry}")

if failed:
    sys.exit(1)
print("i18n coverage complete")
PY
