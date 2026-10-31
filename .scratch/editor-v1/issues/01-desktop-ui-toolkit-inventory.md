Type: research
Status: resolved

## Question

What credible **desktop UI toolkit / shell** options can ship **Editor v1** on Windows, macOS, and Linux from one shared codebase while **linking the Rust `nextguide-gde` library in-process** (or with a first-class Rust host), and what is each option’s realistic story for a **rich-text editing** surface?

Inventory options with primary-source citations (official docs / project sources). Compare at decision-useful resolution: licensing, maturity, packaging story, how Rust embeds, and how a rich-text control or webview would fit. Do not choose the winner — that is a later grilling ticket.

## Answer

Credible options cluster as Rust+webview (Tauri 2, Dioxus), pure-Rust GUIs (egui/eframe, iced, Slint, GPUI, Floem), and native toolkits with Rust bridges (GTK4/Relm4, CXX-Qt, Flutter+FRB); Electron+napi-rs is in-process but not a Rust host. Stock rich-text editing is strongest on GTK TextView+tags, Qt TextEdit, and webviews; most pure-Rust GUIs need custom or plain multiline. Full inventory (no winner): [`docs/research/editor-v1-desktop-ui-toolkits.md`](../../../../docs/research/editor-v1-desktop-ui-toolkits.md).
