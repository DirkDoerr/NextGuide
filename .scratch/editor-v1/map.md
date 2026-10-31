# Editor v1 wayfinder map

Type: wayfinder:map

## Destination

A buildable product **spec** for **Editor v1**: a cross-platform desktop editor (Windows, macOS, Linux) that opens/saves Guides via Format v1’s Core model, supports The Guide’s **functionality** for that surface (navigate tree; edit titles and bodies; restructure Nodes; new Guide), hides the Dummy root in the UI, and uses a pragmatic rich-text subset — not a Windows UI clone, not mobile, not full attribute/RTF parity.

## Notes

- Domain language: `GLOSSARY.md`
- Tracker: local markdown under `.scratch/editor-v1/` (see `docs/agents/issue-tracker.md`)
- Builds on Format v1 (`nextguide-gde` / `nextguide`); greenfield UI, not a The Guide fork
- Charting posture: **decisions**, not deliverables — hand off to `/to-spec` when the frontier clears
- Spec published: [`spec.md`](spec.md) (`ready-for-agent`)
- Skills: `grilling` + `domain-modeling` for grilling tickets; `research` for research tickets; `prototype` only if a ticket is labelled that way
- Standing prefs from charting: shared desktop codebase; in-process link to the Rust Core-model library; Dummy root hidden in UI; workflows open/save/save-as, navigate, edit title/body, restructure, new Guide
- Standing prefs from toolkit grilling: **no web UI tech**; **GTK4 + Relm4**; app binary `nextguide`; CLI binary `nextguide-cli`
- Standing prefs from rich-text grilling: **`nextguide-rtf`** ↔ GTK tags; preserve untouched RTF bytes; subset rewrite on edit
- Standing prefs from packaging: **first release artifacts Win+macOS**; Linux packaging later; codebase still GTK cross-platform
- Standing prefs from fog clear-out: **standard desktop shortcuts** (not The Guide key parity); **menu/command** tree restructure (no DnD); keyboard-usable, single default look

## Decisions so far

- [Desktop UI toolkit inventory](issues/01-desktop-ui-toolkit-inventory.md): Options clustered (Tauri/Dioxus webview; egui/iced/Slint/etc.; GTK/Qt bridges; Flutter) — no winner; see `docs/research/editor-v1-desktop-ui-toolkits.md`
- [RTF ↔ editor interop inventory](issues/02-rtf-edit-interop-inventory.md): four families inventoried (pure-Rust RTF crates; convert-through-HTML for webview; OS-native Rich Edit / Cocoa / no GTK RTF; external Pandoc/LibreOffice) — no pipeline pick; see `docs/research/editor-v1-rtf-edit-interop.md`
- [Core-model mutation API](issues/04-core-model-mutation-api.md): Extend `nextguide-gde` with explicit ops + `NodePath`; Dummy root immovable; fail-closed typed edit errors; Preserve-through details deferred
- [Choose UI toolkit](issues/03-choose-ui-toolkit.md): GTK4 + Relm4, no web shell; `nextguide` editor + `nextguide-cli`; dynamic GTK / bundle on Win+macOS
- [Rich-text pipeline](issues/05-rich-text-pipeline.md): `nextguide-rtf` ↔ GTK tags; untouched bytes preserved; bold/italic/underline/paragraphs/lists subset on edit
- [Layout and Dummy root](issues/06-layout-and-dummy-root.md): Two-pane tree|body; Dummy root hidden; titles in tree; single selection; resizable split
- [Document session model](issues/07-document-session-model.md): One Guide per window; dirty prompts; explicit save; `nextguide [path]`; recent files (5)
- [Undo/redo policy](issues/08-undo-redo-policy.md): Two stacks (GTK body + tree commands); ~50 depth; Edit menu / shortcuts by focus
- [Preserve-through under edits](issues/09-preserve-through-under-edits.md): Bags follow Node/Guide; kept on title/RTF/move; dropped on delete; empty on insert; still private
- [Packaging and distribution](issues/10-packaging-and-distribution.md): First ship Win+macOS bundles (+GTK); `nextguide-cli` same release; manual updates; signing optional; Linux packages later
- [Testing and proof bar](issues/11-testing-and-proof-bar.md): Keep Format v1 gates; mutation + `nextguide-rtf` primary; thin UI smoke; The Guide human-only

## Not yet specified

<!-- empty: frontier cleared for this destination -->

## Out of scope

- Android / iOS (and other mobile) apps
- Visual clone of The Guide’s Windows UI (functionality, not chrome parity)
- Full The Guide 2.0 attribute **API** parity (icons, colors, checkboxes, hyperlinks, remembered expansion, etc.)
- Full Word-grade / exotic RTF parity (pragmatic subset + best-effort display of foreign RTF)
- Cloud sync, collaboration, or accounts
- Replacing or abandoning Format v1’s Round-trip gates
- Autosave (Editor v1 is explicit save only)
- Linux **installers** in the first Editor v1 release wave (codebase remains Win/macOS/Linux; packages follow later)
- Deep The Guide **keyboard / shortcut** parity (standard desktop shortcuts only)
- **Full-text search** across a Guide
- **Drag-and-drop** reparenting (menu/commands only in Editor v1)
- Deep accessibility audit beyond keyboard-usable
- Custom **themes** / appearance packs
- **Print** / export besides `.gde`
