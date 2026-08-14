# STATE

> Ultra-compact current-state pointer. First file Claude Code reads each session.
> Updated continuously: at task start, on significant sub-steps, on interruption.
> Maximum 30 lines. Machine-readable structure.

---

active_task: none  # T-230 and T-231 complete (gates/build green). T-232 pending user. Work uncommitted; commit pending user go-ahead.
active_task_state: IDLE
active_branch: main
last_completed_task: T-231  # AppStream screenshots wired (4 PNGs in data/screenshots/, metainfo <screenshots>). Prior: T-230 offline Flatpak build (closes #6).
last_milestone: v0.4.2  # v0.6 (Polish / Flathub) is the active milestone.
last_commit_on_main: e9fc39c  # ci: build a CachyOS .pkg.tar.zst test artifact (#18). Working tree holds T-230 + T-231 changes, not yet committed.
last_step: 2026-08-14 — Tiny 2 Lite (3564:fef9) connected; offline-built Flatpak exported to a local repo, reinstalled and launched (libgstgtk4.so bundled, no errors); user verified tabs/preview/zoom; user captured 6 screenshots; 4 selected into data/screenshots/; metainfo <screenshots> added; appstreamcli --no-net exit 0. gh CLI authed as alvaro-domatix.
next_step: user gives the go-ahead → 3 commits: (1) build(flatpak) T-230 manifest + cargo-sources, (2) feat(flatpak) T-231 screenshots + metainfo, (3) docs T-230/231 + ADR-0031; then push. After that T-232: cut release tag, switch manifest app source type:dir → pinned git, flathub.org login (user), submit, push to flathub/<app-id>.
blockers:
  - T-232 submit needs the user's flathub.org login (interactive OAuth) and a cut release tag.
  - native /usr/local install pending user sudo (from v0.4.2); T-017b Arch validation pending an Arch host.
working_tree:  # matches `git status --short`
  - M build-aux/io.github.domatix.ObsbotCamControl.json  # offline cargo build (T-230)
  - ?? build-aux/cargo-sources.json, build-aux/cargo-sources-gst.json  # generated vendor sources (T-230)
  - M data/io.github.domatix.ObsbotCamControl.metainfo.xml.in  # <screenshots> (T-231)
  - ?? data/screenshots/{main-page-with-preview,image-controls,ptz-zoom-controls,presets}.png  # captures (T-231)
  - M docs/{DECISIONS,PLAN,PROGRESS,STATE}.md  # ADR-0031, tasks T-230/231/232, journal
firmware_notes: Tiny 2 Lite fw 5.10 — XU Sleep IGNORED ~3s after streaming stops (cold Sleep immediate); rapid open/close/sleep churn can hang capture until USB replug (ADR-0025). Q9: pan/tilt_speed write but no motion; PTZ via discrete steps (T-101d).
flatpak_build_note: rofiles-fuse cannot mount on /tmp (nodev tmpfs) — run flatpak-builder with --state-dir and build dir under $HOME, never /tmp.
updated_at: 2026-08-14T12:35:00Z
