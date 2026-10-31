# 21: Win/macOS release bundles (+ `nextguide-cli`)

**What to build:** First-wave **Windows and macOS** release artifacts that ship `nextguide` with a **bundled GTK runtime** and `nextguide-cli` in the same release; manual updates; signing optional (unsigned OK). Linux installers remain deferred.

**Blocked by:** 19 — Document session; 20 — Undo/redo

**Status:** resolved

- [x] Documented/automated path to produce Windows installer (MSI or NSIS) and macOS `.app` in a DMG including GTK
- [x] `nextguide-cli` is included in the same release artifacts
- [x] README (or release notes) describe manual updates and optional signing
- [x] Linux packaging explicitly left for a later wave

## Answer

Added `packaging/` with release-engineer scripts and docs:

- **macOS:** `packaging/macos/build-dmg.sh` builds `NextGuide.app` (GTK dylibs via `dylibbundler`, schemas/loaders/icons + launcher) and wraps it with `nextguide-cli` in a UDZO DMG under `dist/`. Ad-hoc codesign by default; optional `CODESIGN_IDENTITY` / `NOTARIZE=1`.
- **Windows:** `packaging/windows/assemble-bundle.sh` (MSYS2 UCRT64) collects `nextguide.exe` + GTK DLL closure + schemas/icons + `nextguide-cli.exe`; `nextguide.nsi` + `build-installer.ps1` produce an NSIS setup.exe. Optional Authenticode via `SIGNTOOL_ARGS`.
- **Docs:** `packaging/README.md` + README section cover manual updates, optional signing, and deferred Linux installers.

No signed binaries produced on this machine (tooling/certs not assumed). Path is scripted so a release host with GTK + NSIS / Homebrew can build artifacts.
