Type: grilling
Status: resolved
Blocked by: 03

## Question

How should Editor v1 be **packaged and distributed** on Windows, macOS, and Linux (installers vs drop-a-folder, update story, signing if any) given the chosen UI toolkit?

## Answer

**First release packaging: Windows + macOS** (Linux installers follow later; GTK codebase stays cross-platform).

- **Win/macOS:** installable bundles that include `nextguide` **and a bundled GTK runtime** (not naked binaries hoping GTK is present). Targets: Windows MSI or NSIS; macOS `.app` in a DMG. Exact tooling at implement time.
- **`nextguide-cli`:** same release artifacts as the editor (adjacent binary / same download).
- **Updates:** manual (e.g. GitHub Releases) — no in-app auto-updater in Editor v1.
- **Signing / notarization:** best-effort / optional; unsigned early releases OK — do not block ship.
- **Linux:** packaging deferred past the first Editor v1 release wave (`.deb` / AppImage later).
