# GTK4 + Relm4 for Editor v1 (no web UI)

Editor v1 needs a Win/macOS/Linux desktop UI that links `nextguide-gde` in-process and supports a pragmatic rich-text body editor, without adopting web tech (no OS webview shell, no HTML/`contenteditable` body surface). We chose **GTK4** via **gtk-rs** with **Relm4** as the app structure: strongest stock rich-text widgets (`TextView` / `TextBuffer` / `TextTag`) among non-web options with a Rust host. Cost accepted: **LGPL GTK** via dynamic linking and bundling a GTK runtime on Windows and macOS. The editor binary is **`nextguide`**; the Format v1 Round-trip CLI is renamed **`nextguide-cli`**.

## Considered Options

- **Tauri 2 / Dioxus (webview)** — excellent packaging and HTML rich-text ecosystem; rejected because Editor v1 explicitly avoids web UI tech.
- **Pure-Rust GUIs (egui, iced, Slint, …)** — clean Rust-only binaries; rejected as primary shell because stock controls are plain-text or display-only, so pragmatic rich editing becomes a custom editor project.
- **GTK4 + Relm4** (chosen) — Rust in-process host, mature cross-desktop toolkit, first-party tagged rich text; LGPL + Win/macOS runtime bundling.
- **Qt via CXX-Qt** — formatted `TextEdit` exists; rejected for Editor v1 over LGPL/commercial Qt complexity plus an early Rust bridge.
- **Flutter + flutter_rust_bridge** — shared desktop UI; rejected because the UI host is Dart, not Rust.
- **Electron + napi-rs** — full Chromium editing; rejected as web tech and a non-Rust application host.
