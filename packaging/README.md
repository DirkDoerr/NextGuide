# Editor v1 release packaging (Windows + macOS)

First-wave release artifacts ship **`nextguide`** (GTK4 editor), a **bundled GTK runtime**, and **`nextguide-cli`** in the same download. Updates are **manual** (e.g. GitHub Releases). **Code signing / notarization are optional** for early releases — unsigned artifacts are acceptable.

**Linux installers are deferred** past this wave (the codebase still builds on Linux with a system GTK).

| Platform | Artifact | Tooling |
| --- | --- | --- |
| macOS | `NextGuide.app` inside a `.dmg` | Homebrew GTK4 + `dylibbundler` + `hdiutil` |
| Windows | NSIS `.exe` installer (folder install with GTK DLLs) | MSYS2 UCRT64 GTK4 + NSIS (`makensis`) |

Outputs land under `dist/` (gitignored).

## Prerequisites (shared)

- Rust toolchain matching `rust-toolchain.toml`
- Release build of both binaries:

```bash
cargo build --release -p nextguide -p nextguide-cli
```

## macOS (`.app` in DMG)

Run on an Apple Silicon or Intel Mac that can link against Homebrew GTK4 (architecture of the GTK install must match the binary).

```bash
# once per machine
brew install gtk4 dylibbundler

# from repo root
./packaging/macos/build-dmg.sh
```

Produces `dist/NextGuide-<version>-macos-<arch>.dmg` containing:

- `NextGuide.app` — launcher + `nextguide` with GTK dylibs under `Contents/Frameworks`, plus schemas / gdk-pixbuf loaders / Adwaita icons
- `nextguide-cli` next to the `.app` on the DMG (same release)

### Optional signing / notarization

Early releases may ship **ad-hoc signed** (`codesign -s -`), which the script does by default so Apple Silicon will launch the app.

For a Developer ID build (when certificates exist):

```bash
CODESIGN_IDENTITY="Developer ID Application: Example (TEAMID)" \
  NOTARIZE=1 APPLE_ID=… APPLE_TEAM_ID=… APPLE_APP_SPECIFIC_PASSWORD=… \
  ./packaging/macos/build-dmg.sh
```

Skip notarization anytime (`NOTARIZE` unset). Lack of a paid Apple account must not block shipping unsigned/ad-hoc builds.

## Windows (NSIS + bundled GTK)

Build and assemble on **Windows** inside an **MSYS2 UCRT64** shell (GTK cannot be statically linked; end users get the DLL tree from the installer).

```bash
# MSYS2 UCRT64 — install toolchain + GTK once
pacman -S --needed \
  mingw-w64-ucrt-x86_64-gtk4 \
  mingw-w64-ucrt-x86_64-pkgconf \
  mingw-w64-ucrt-x86_64-rust \
  mingw-w64-ucrt-x86_64-gcc \
  mingw-w64-ucrt-x86_64-binutils

# from repo root (UCRT64)
./packaging/windows/assemble-bundle.sh
```

That writes a self-contained tree to `dist/windows-bundle/` (`nextguide.exe`, `nextguide-cli.exe`, GTK DLLs, schemas, loaders, icons).

Then build the NSIS installer (Windows host with [NSIS](https://nsis.sourceforge.io/) on `PATH`, or from PowerShell):

```powershell
# from repo root, after assemble-bundle.sh
.\packaging\windows\build-installer.ps1
```

Produces `dist/NextGuide-<version>-windows-x64-setup.exe`.

Alternatively, zip the bundle for smoke tests without NSIS:

```bash
# still in MSYS2 after assemble-bundle.sh
cd dist && zip -r "NextGuide-${VERSION}-windows-x64-bundle.zip" windows-bundle
```

### Optional Authenticode signing

```powershell
$env:SIGNTOOL_ARGS = '/s /n "Your Cert Name" /fd SHA256 /tr http://timestamp.digicert.com /td SHA256'
.\packaging\windows\build-installer.ps1
```

Unsigned installers are fine for early Editor v1 releases.

## Manual updates

There is **no in-app auto-updater** in Editor v1. Publish installers on GitHub Releases (or equivalent); users download and install over the previous version. Mention that in release notes.

## Linux

Deferred. Developers run against distro GTK (`libgtk-4-dev` / equivalent). No `.deb` / AppImage / Flatpak in this wave.

## LGPL note

GTK is LGPL and is **dynamically linked** and redistributed as separate shared libraries inside the bundle. Keep source offer obligations in mind for formal releases (GTK / GLib / Pango / etc.).
