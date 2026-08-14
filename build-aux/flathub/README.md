# Flathub submission bundle

Self-contained material for the Flathub submission of
`io.github.domatix.ObsbotCamControl`. The three JSON files in this
directory are everything the Flathub build bot needs:

- `io.github.domatix.ObsbotCamControl.json` — the submission
  manifest. The app source is the release tag (git + commit pin),
  not the local `type: dir` used by the in-repo dev manifest
  (`build-aux/io.github.domatix.ObsbotCamControl.json`).
- `cargo-sources.json`, `cargo-sources-gst.json` — the hashed
  offline cargo sources (ADR-0031). flatpak-builder resolves the
  inline string sources relative to the manifest, so these must sit
  next to it in the Flathub repo.

## Submission steps (done once, by a human)

1. Sign in at <https://flathub.org> with the GitHub account that
   owns the `Domatix` namespace (`alvaro-domatix`).
2. Submit the app through the Flathub portal, pointing at this
   repository. Flathub verifies the app-id namespace
   (`io.github.domatix.*` → `github.com/Domatix/`).
3. When the review passes, Flathub provisions the repo
   `flathub/io.github.domatix.ObsbotCamControl`. Push the three JSON
   files of this directory to that repo's default branch (the
   manifest conventionally lives at the repo root).
4. The Flathub CI builds the app; on green, it is published and
   users get update notifications through their software center
   (GNOME Software / KDE Discover) from then on.

## Cutting a future release

1. In the app repo: bump the workspace version, add a metainfo
   `<release>` entry, tag, push (see `docs/PLAN.md` T-232 and
   ADR-0032).
2. In this directory: update the app source `tag` + `commit` to the
   new release, and if `Cargo.lock` moved, regenerate the
   `cargo-sources*.json` (ADR-0031) and copy them here.
3. Push the three files to the `flathub/<app-id>` repo; Flathub CI
   rebuilds and publishes.

## Verifying the bundle locally

```
flatpak-builder --user --force-clean \
  --state-dir="$HOME/obsbot-fp-state" \
  --install-deps-from=flathub \
  "$HOME/obsbot-fp-verify" \
  io.github.domatix.ObsbotCamControl.json
```

Notes: use a state-dir under `$HOME`, never `/tmp` (`rofiles-fuse`
cannot mount on a `nodev` tmpfs). The build is fully offline after
the download phase — no module uses `--share=network`.

## Reviewer notes

- `--device=all` is deliberate: the vendor XU control surface needs
  raw `/dev/videoN` access plus USB ioctls; no portal covers that.
- GNOME Platform 50 / freedesktop SDK 25.08, with the
  `rust-stable` and `llvm20` SDK extensions.
- The app makes no network calls at runtime (SPEC §6.1).
