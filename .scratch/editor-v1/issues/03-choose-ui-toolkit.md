Type: grilling
Status: resolved
Blocked by: 01

## Question

Which **UI toolkit / desktop shell** should Editor v1 use, given the inventory and the standing prefs (Win/macOS/Linux; shared codebase; in-process Rust Core-model library; rich-text editing required)?

## Answer

**GTK4 + Relm4**, no web tech (no OS webview shell, no HTML/`contenteditable` body editor).

- Rejected for Editor v1: Tauri/Dioxus/Electron (web), pure-Rust GUIs as primary shell (weak stock rich-text), Qt/CXX-Qt (license/bridge tax), Flutter (Dart host).
- Rich text: GTK `TextView` / `TextBuffer` / `TextTag`; RTF ↔ buffer conversion in Rust (pipeline ticket assumes this, not HTML/webview).
- LGPL: dynamically link GTK; bundle GTK runtime on Windows/macOS as needed (packaging ticket).
- Crates: editor app binary **`nextguide`**; rename Format v1 CLI crate/binary to **`nextguide-cli`** (dump/check). Editor crate depends on `nextguide-gde` + Relm4/gtk4.
