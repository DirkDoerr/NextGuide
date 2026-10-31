# NextGuide

A hierarchical tree based note taking SW.

This application is based on the project:

https://theguide.sourceforge.net/

in terms of functionality and allowing to read/write the same file format.

It targets Windows, MacOS and Linux. Optionally Android and iOS.

It is also an exercise in building with AI.

## Format v1

Format v1 is a greenfield library and CLI for The Guide **v2** `.gde` Core model (tree, titles, opaque RTF bodies).

Requires a stable Rust toolchain (`rust-toolchain.toml`; install via rustup or Homebrew `rust`).

- Library: `nextguide-gde` (`crates/nextguide-gde`)
- RTF edit model: `nextguide-rtf` (`crates/nextguide-rtf`) — pragmatic subset, no GTK types
- CLI: `nextguide-cli` (`crates/nextguide-cli`) — `dump` and `check` (`--byte-stable` for owned goldens)
- Editor: `nextguide` (`crates/nextguide`) — GTK4 + Relm4 **Editor v1** desktop app
- Fixtures: `data/`

```bash
cargo test
cargo test -p nextguide --no-default-features
cargo run -p nextguide-cli -- dump data/ImportSample.gde
cargo run -p nextguide-cli -- check data/ImportSample.gde
cargo run -p nextguide-cli -- check --byte-stable data/canonical-golden.gde
```

### Editor (`nextguide`)

Editor v1 opens and saves The Guide v2 `.gde` files with a two-pane desktop UI (tree left, body right): navigate and rename Nodes, edit bodies with bold / italic / underline / paragraphs / simple bullet and numbered lists, restructure the tree (insert / delete / reorder / indent / outdent), Undo/Redo (body vs tree by focus), New / Open / Save / Save As with dirty prompts, and a recent-files list. Non-Core attributes and untouched Node RTF bytes are preserved until you edit that body.

Needs a system GTK4 development install to build the UI (`ui` feature, on by default for that package). Workspace `default-members` exclude the editor so plain `cargo test` stays headless-friendly.

```bash
# macOS (Homebrew): brew install gtk4
# Debian/Ubuntu: sudo apt install libgtk-4-dev
cargo run -p nextguide
```

**Menus / shortcuts:** File and Edit (plus Format and Tree) menus in the header. Standard shortcuts: Ctrl/Cmd+N/O/S, Ctrl/Cmd+Shift+S (Save As), Undo/Redo (Ctrl/Cmd+Z / Shift+Z or Y), body styles Ctrl/Cmd+B/I/U, lists Ctrl/Cmd+Shift+8 (bullet) and Ctrl/Cmd+Shift+7 (numbered).

**Rich-text honesty:** The editable subset is bold / italic / underline / paragraphs / simple lists. Colours, highlights, hyperlinks, tables, images, and other exotic The Guide RTF are **subset display only** — they are not mapped into GTK colour/highlight tags in Editor v1, and may drop when that body is rewritten after an edit. Untouched bodies keep original RTF bytes on save.

**Manual check (UI smoke deferred in CI):** `cargo run -p nextguide -- data/ImportSample.gde` — confirm a resizable two-pane window (Dummy root hidden; titles in the tree), select a Node, edit a title and body (B/I/U + bullet/numbered lists; window title shows `*`), Save, reopen and confirm changes; sibling bodies you did not edit should keep exotic RTF. Bare `cargo run -p nextguide` opens Untitled.

### Release packaging (Windows + macOS)

First-wave installers ship **`nextguide` + bundled GTK runtime + `nextguide-cli`**. Updates are **manual** (e.g. GitHub Releases). **Signing / notarization are optional** for early releases. Linux installers are deferred.

See [`packaging/README.md`](packaging/README.md) for the full release-engineer path.

```bash
# macOS (Homebrew GTK4 + dylibbundler) → dist/NextGuide-*-macos-*.dmg
./packaging/macos/build-dmg.sh

# Windows (MSYS2 UCRT64) → dist/windows-bundle/, then NSIS setup.exe
./packaging/windows/assemble-bundle.sh
# PowerShell: .\packaging\windows\build-installer.ps1
```
