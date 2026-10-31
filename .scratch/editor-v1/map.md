# Editor v1 wayfinder map

Type: wayfinder:map

## Destination

A buildable product **spec** for **Editor v1**: a cross-platform desktop editor (Windows, macOS, Linux) that opens/saves Guides via Format v1’s Core model, supports The Guide’s **functionality** for that surface (navigate tree; edit titles and bodies; restructure Nodes; new Guide), hides the Dummy root in the UI, and uses a pragmatic rich-text subset — not a Windows UI clone, not mobile, not full attribute/RTF parity.

## Notes

- Domain language: `GLOSSARY.md`
- Tracker: local markdown under `.scratch/editor-v1/` (see `docs/agents/issue-tracker.md`)
- Builds on Format v1 (`nextguide-gde` / `nextguide`); greenfield UI, not a The Guide fork
- Charting posture: **decisions**, not deliverables — hand off to `/to-spec` when the frontier clears
- Skills: `grilling` + `domain-modeling` for grilling tickets; `research` for research tickets; `prototype` only if a ticket is labelled that way
- Standing prefs from charting: shared desktop codebase; in-process link to the Rust Core-model library; Dummy root hidden in UI; workflows open/save/save-as, navigate, edit title/body, restructure, new Guide

## Decisions so far

- [Desktop UI toolkit / shell inventory](issues/01-desktop-ui-toolkit-inventory.md): researched Win/macOS/Linux options for shared-codebase Editor v1 with in-process `nextguide-gde` (Tauri/Dioxus, egui/iced/Slint/GPUI/Floem, GTK/Qt/Flutter, Electron); no winner — see `docs/research/editor-v1-desktop-ui-toolkits.md`.

## Not yet specified

- How deep The Guide **keyboard / shortcut** parity goes for Editor v1 (beyond basic navigation)
- Whether **full-text search** across a Guide is in the first editor or later
- Drag-and-drop vs menu/command-only reparenting details
- Autosave vs explicit-save-only cadence
- Accessibility depth beyond “usable with keyboard”
- Themes / appearance beyond a single default desktop look
- Print / export besides `.gde`

## Out of scope

- Android / iOS (and other mobile) apps
- Visual clone of The Guide’s Windows UI (functionality, not chrome parity)
- Full The Guide 2.0 attribute **API** parity (icons, colors, checkboxes, hyperlinks, remembered expansion, etc.)
- Full Word-grade / exotic RTF parity (pragmatic subset + best-effort display of foreign RTF)
- Cloud sync, collaboration, or accounts
- Replacing or abandoning Format v1’s Round-trip gates
