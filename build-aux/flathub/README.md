# Flathub submission bundle

Self-contained material for the Flathub submission of
`io.github.domatix.obsbot-control`. The three JSON files in this
directory are everything the submission pull request needs:

- `io.github.domatix.obsbot-control.json` — the submission
  manifest. The app source is the release tag (git + commit pin),
  not the local `type: dir` used by the in-repo dev manifest
  (`build-aux/io.github.domatix.obsbot-control.json`).
- `cargo-sources.json`, `cargo-sources-gst.json` — the hashed
  offline cargo sources (ADR-0031). flatpak-builder resolves the
  inline string sources relative to the manifest, so these must sit
  next to it in the submission PR.

## Submission steps (done once, by the app author — never automated)

Per the current Flathub docs
(<https://docs.flathub.org/docs/for-app-authors/submission>),
new-app submissions are pull requests against the `flathub/flathub`
repository. There is no web-portal submit button for new apps.
Flathub policy requires this PR to be opened by a human — do not
automate it or ask AI tools to open it.

1. Fork <https://github.com/flathub/flathub> with *"Copy the master
   branch only"* unchecked.
2. `git clone --branch=new-pr git@github.com:alvaro-domatix/flathub.git && cd flathub`
3. `git checkout -b obsbot-control-submission new-pr`
4. Copy the three JSON files of this directory into the repo root
   (the manifest must be at the top level, named after the app ID).
   Do **not** include application source code or build artifacts.
5. Commit, push, and open the PR against the `new-pr` base branch
   with the title `Add io.github.domatix.obsbot-control`.
6. Answer reviewer questions; when asked, comment `bot, build` to
   trigger the test build.
7. On approval, the reviewers create `flathub/io.github.domatix.obsbot-control`
   and invite you as maintainer (GitHub 2FA required). From then on,
   updates are plain commits to that repo and users get update
   notifications through their software center.

## Cutting a future release

1. In the app repo: bump the workspace version, add a metainfo
   `<release>` entry, tag, push (see `docs/PLAN.md` T-232 and
   ADR-0032).
2. In this directory: update the app source `tag` + `commit` to the
   new release, and if `Cargo.lock` moved, regenerate the
   `cargo-sources*.json` (ADR-0031) and copy them here.
3. Push the updated files to the `flathub/<app-id>` repo; Flathub CI
   rebuilds and publishes.

## Verifying the bundle locally

```
flatpak-builder --user --force-clean \
  --state-dir="$HOME/obsbot-fp-state" \
  --install-deps-from=flathub \
  "$HOME/obsbot-fp-verify" \
  io.github.domatix.obsbot-control.json
```

Notes: use a state-dir under `$HOME`, never `/tmp` (`rofiles-fuse`
cannot mount on a `nodev` tmpfs). The build is fully offline after
the download phase — no module uses `--share=network`.

Pre-submission lint (recommended by the docs):

```
flatpak install -y flathub org.flatpak.Builder
flatpak run --command=flatpak-builder-lint org.flatpak.Builder manifest io.github.domatix.obsbot-control.json
```

## Reviewer notes

- `--device=all` is deliberate: the vendor XU control surface needs
  raw `/dev/videoN` access plus USB ioctls; no portal covers that.
- GNOME Platform 50 / freedesktop SDK 25.08, with the
  `rust-stable` and `llvm20` SDK extensions.
- The app makes no network calls at runtime (SPEC §6.1).
