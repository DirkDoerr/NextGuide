# Desktop UI toolkit inventory for Editor v1

**Date:** 2026-10-31  
**Ticket:** `.scratch/editor-v1/issues/01-desktop-ui-toolkit-inventory.md`  
**Scope:** Credible **desktop UI toolkit / shell** options that can ship **Editor v1** on Windows, macOS, and Linux from one shared codebase while linking the Rust `nextguide-gde` library **in-process** (or with a first-class Rust host), plus each option’s realistic story for a **rich-text editing** surface. Inventory only — **no winner**.

## Verdict

Several stacks meet the hard constraints (Win/macOS/Linux, shared codebase, Rust in-process or Rust-as-host). They cluster into three shapes:

1. **Rust host + OS webview** (Tauri 2, Dioxus desktop) — rich text via HTML/`contenteditable` / web editor libraries in the webview.
2. **Pure-Rust (or Rust-native) retained/immediate GUI** (egui/eframe, iced, Slint, GPUI, Floem) — rich text is **not** a first-class shippable editor widget in most of these; expect custom canvas editors or plain multiline text plus a pragmatic subset.
3. **Mature native widget toolkits with Rust bindings / FFI** (GTK4 via gtk-rs/Relm4; Qt via CXX-Qt; Flutter via `flutter_rust_bridge`) — strongest **built-in** rich-text **widgets** (GTK `TextView`/`TextTag`; Qt Quick `TextEdit` with formats; Flutter strong for **display** spans, editing often single-style `EditableText`).

**Electron + Node-API (napi-rs)** can load `nextguide-gde` as a native addon in-process with the Node/Electron host, but the **application host is not Rust** — listed for completeness against the “first-class Rust host” preference.

Claims below are tied to **primary / first-party** sources (official docs, project READMEs, crates.io metadata, project LICENSE/FAQ). Secondary blog round-ups are not used as evidence.

## Comparison at a glance

| Option | License (toolkit) | Platforms (claimed) | Maturity signal | Packaging story | Rust ↔ app | Rich-text / edit surface |
|--------|-------------------|---------------------|-----------------|-----------------|------------|---------------------------|
| Tauri 2 | Apache-2.0 OR MIT (`tauri` crate) | Win / macOS / Linux (+ mobile) | Stable **2.x** on crates.io (2.12.x); 3.x alpha | Official `tauri build` / distribute (NSIS/MSI, DMG, deb/RPM/AppImage, …) | **Rust core process**; UI in OS webview | Web stack (`contenteditable`, ProseMirror, etc.) |
| Dioxus desktop | MIT OR Apache-2.0 | Win / macOS / Linux (webview deps) | Stable **0.7.x**; 0.8 alpha | Official `dx bundle` / CLI | **Rust app**; desktop via webview | Same web rich-text path as Tauri-like shells |
| egui + eframe | MIT OR Apache-2.0 | Native Win / macOS / Linux (and web/Android via eframe) | Widely used; ongoing releases (eframe 0.37) | Ship as normal Rust binary; no first-party installer suite like Tauri | **In-process Rust crate** | Built-in `TextEdit` = plain string; rich edit needs custom / third-party caret crates |
| iced | MIT | Win / macOS / Linux / Web | **0.14**; docs call it **experimental** | Normal Rust binary; ecosystem renderers | **In-process Rust** | `TextEditor` = multiline plain (+ highlighter); `rich_text!` for **display** spans |
| Slint | GPLv3 **or** Royalty-free **or** Commercial | Desktop / mobile / web / embedded (product claims) | **1.x** (`slint` 1.16+ in Rust docs) | Native compile; product emphasizes desktop+embedded | **Rust host embeds `.slint` UI** | `TextEdit` = multiline plain; first-party rich-text **editor** still tracked as future work |
| GTK4 + gtk-rs / Relm4 | GTK: **LGPL**; gtk4-rs: **MIT**; Relm4: MIT OR Apache-2.0 | Linux / Windows / macOS (GTK + Relm4 books) | GTK4 mature; gtk-rs production apps; Relm4 0.9–0.11 line | Distro packages / Flatpak common on Linux; Win/macOS need GTK runtime bundling | **In-process Rust** calling GTK | **Strong**: `GtkTextBuffer` + `GtkTextTag` / `TextView` |
| Flutter desktop + flutter_rust_bridge | Flutter BSD-style (Flutter project); FRB per its LICENSE | Win / macOS / Linux (Flutter desktop + FRB) | Flutter desktop GA; FRB v2 | `flutter build windows\|macos\|linux` | Rust as **linked native lib** via FRB (Dart host) | `TextSpan`/`RichText` for display; `EditableText` uses a single `TextStyle` |
| Electron + napi-rs | Electron (Chromium/Node stack; see Electron LICENSE); napi-rs tooling separate | Win / macOS / Linux | Very mature desktop shell | Electron Forge / electron-builder | Rust **addon** in Node process (**not** Rust-as-host) | Full Chromium editing surface |
| GPUI (Zed) | Apache-2.0 | macOS / Windows / Linux (current platform docs) | **pre-1.0**; breaking changes expected | Ship as Rust binary (Zed’s own app is the existence proof) | **In-process Rust** | Built for a code editor; rich **prose** WYSIWYG not a documented stock widget |
| CXX-Qt + Qt | CXX-Qt: MIT OR Apache-2.0; **Qt**: Commercial or **LGPLv3** (modules vary) | CI: Linux / Windows / macOS | CXX-Qt early (“API changes frequently”); Qt mature | Qt deployment tooling + CMake/Cargo | Rust `QObject`s ↔ Qt/QML | Qt Quick `TextEdit`: “editable **formatted** text” |
| Floem | MIT | Win / macOS / Linux | **pre-1.0** (“still maturing”) | Normal Rust binary | **In-process Rust** | Custom / app-built editors; no first-party rich-text editor documented as shipped |

## Editor v1 constraints (from glossary / map)

- **Editor v1**: cross-platform desktop (Windows, macOS, Linux); Core-model Guides; pragmatic rich-text subset for Node bodies; greenfield UI.
- **Format v1 library**: `nextguide-gde` is a normal Rust library crate (`crates/nextguide-gde`) — in-process linking means a Rust dependency or a C ABI / FFI boundary from another host.
- Preference from charting: **shared desktop codebase** + **in-process** Core-model library.

---

## 1. Tauri 2

### Primary sources

- Product / docs home: https://v2.tauri.app/
- Process model: https://v2.tauri.app/concept/process-model/
- Prerequisites (OS matrix): https://v2.tauri.app/start/prerequisites/
- Distribute / bundle: https://v2.tauri.app/distribute/
- Crate metadata: https://crates.io/crates/tauri (License: **Apache-2.0 OR MIT**; stable line **2.12.x** as of research date; 3.0 alphas also published)

### What the sources establish

- **Cross-platform from one codebase**: “Build your app for Linux, macOS, Windows, Android and iOS - all from a single codebase. Write your frontend in JavaScript, application logic in Rust…” (https://v2.tauri.app/).
- **Rust is the core host**: “Each Tauri application has a **core process**… We chose **Rust** to implement Tauri…” The UI runs in WebView processes using OS webviews: **WebView2** (Windows), **WKWebView** (macOS), **webkitgtk** (Linux) (process-model page).
- **Packaging**: Official CLI `tauri build` / distribute docs cover Linux (deb, RPM, AppImage, Snap, Flatpak, AUR), macOS (App Bundle, DMG, App Store), Windows installers / Store (distribute page).
- **Prerequisites**: macOS Catalina 10.15+; Windows 7+; Linux needs WebKitGTK 4.1 stack (prerequisites page).

### Fit for `nextguide-gde` and rich text

- Link `nextguide-gde` as a normal Rust crate dependency of the Tauri core; expose commands/events to the webview over Tauri IPC.
- Rich-text story is the **web** story: HTML editors, `contenteditable`, or JS editor frameworks inside the webview. No Tauri-specific rich-text widget — the surface is the platform web engine.

---

## 2. Dioxus (desktop)

### Primary sources

- Getting started (desktop deps): https://dioxuslabs.com/learn/0.6/getting_started/
- Crate: https://crates.io/crates/dioxus (License: **MIT OR Apache-2.0**; stable **0.7.10**; 0.8 alphas)
- README claims: cross-platform including desktop; **`dx bundle`** for macOS, Linux, Windows; desktop feature uses webview stack (crates.io README)

### What the sources establish

- Desktop apps depend on **WebView2** on Windows and **WebKitGTK** (+ related packages) on Linux; macOS has no extra webview package listed (getting-started).
- CLI `dx` is the first-party build/serve/bundle tool; README documents `dx serve --desktop` and `dx bundle`.
- App logic is **Rust** (`dioxus::launch`); UI is RSX → rendered through the desktop/webview (or experimental native) backends described on the crate README.

### Fit

- Same embedding shape as Tauri for Core-model: Rust process owns `nextguide-gde`.
- Rich text: web-oriented editing inside the desktop webview (or custom widgets if using experimental native renderer — crates.io README marks native renderer **experimental**).

---

## 3. egui + eframe

### Primary sources

- egui repository README: https://github.com/emilk/egui/ (License: **MIT OR Apache-2.0**; “`eframe` is the official egui framework… Web, Linux, Mac, Windows, and Android”)
- eframe crate: https://crates.io/crates/eframe (MIT OR Apache-2.0)
- Built-in editor: https://docs.rs/egui/latest/egui/widgets/text_edit/struct.TextEdit.html (`TextEdit` edits a string buffer)
- Third-party rich caret (not egui-official): https://docs.rs/egui_richedit/latest/egui_richedit/ — documents that egui’s `TextEdit` is one string / one format; rich editing requires app-owned layout + caret crate (**MIT**)

### What the sources establish

- Pure Rust immediate-mode GUI; **in-process** `nextguide-gde` is a normal Cargo dependency.
- Official text editing is **plain** multiline/`TextEdit`.
- A shippable **rich** editor implies custom document model + layout (optionally assisted by community crates such as `egui_richedit`), not a stock egui control.

### Packaging

- No Tauri-like first-party cross-platform installer matrix in egui docs; distribution is “ship a native binary” (and optional web/wasm via eframe). Installer choice is left to the application.

---

## 4. iced

### Primary sources

- Site: https://iced.rs/
- Crate: https://crates.io/crates/iced (License: **MIT**; **0.14.0**; README: Windows, macOS, Linux, Web; “Iced is currently **experimental** software”)
- `TextEditor`: https://docs.rs/iced/latest/iced/widget/text_editor/struct.TextEditor.html — “A multi-line text input”; highlighter APIs for syntax coloring
- Widget module exports `rich_text` for **display** spans: https://docs.rs/iced/latest/iced/widget/

### Fit

- First-class Rust GUI; in-process `nextguide-gde`.
- Realistic rich-text path: custom widget / document model; stock `TextEditor` is not a WYSIWYG RTF/HTML body editor. `rich_text!` addresses styled **display**, not an editing surface with mixed runs under a caret (per widget docs’ separation of `TextEditor` vs `rich_text`).

---

## 5. Slint

### Primary sources

- Product: https://slint.dev/
- Rust embedding API: https://docs.slint.dev/latest/docs/rust/slint/
- `TextEdit` widget: https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/textedit/ — multiline plain text; selection/clipboard APIs; **no** per-run style API on the widget page
- Licensing FAQ: https://raw.githubusercontent.com/slint-ui/slint/master/FAQ.md — choose **Royalty-free**, **GPLv3**, or **Commercial**; Royalty-free requires attribution (AboutSlint / badge) and is for desktop/mobile/web (not embedded under that license)
- Rich-text editor tracking: https://github.com/slint-ui/slint/issues/2723 — maintainers state built-in rich-text **editing** is not current scope; viewing work tracked separately; discussion https://github.com/slint-ui/slint/discussions/10563 points back to #2723

### Fit

- Excellent **Rust host** story (`slint` crate embeds UI).
- Rich-text **editing** for Node bodies is a **gap** relative to GTK/Qt/web stacks unless the app builds a custom editor or waits on framework work.

---

## 6. GTK4 via gtk-rs / Relm4

### Primary sources

- GTK overview (platforms + LGPL): https://docs.gtk.org/gtk4/overview.html — “UNIX-like platforms, Windows, and macOS”; “GNU Lesser General Public License”
- Windows / macOS backends: https://docs.gtk.org/gtk4/windows.html , https://docs.gtk.org/gtk4/osx.html
- Install matrix: https://www.gtk.org/docs/installations/index
- Text widget overview (tags = attributes on ranges): https://docs.gtk.org/gtk4/section-text-widget.html
- gtk4-rs: https://github.com/gtk-rs/gtk4 (MIT); docs e.g. https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/struct.TextBuffer.html
- Relm4 book: https://relm4.org/book/stable/ — “based on gtk4-rs”; platforms Linux / Windows / macOS; Relm4 license MIT OR Apache-2.0 (crate docs)

### Fit

- **In-process Rust** bindings to a mature native toolkit.
- **Strongest first-party rich-text editing** among pure-desktop options reviewed: `GtkTextBuffer` + `GtkTextTag` applied to ranges, shown in `GtkTextView`, with undo support when enabled (GTK text widget overview).
- Packaging cost: on Linux, system GTK; on Windows/macOS, must ship or install a GTK runtime (GTK’s own Windows/macOS docs point at project bundling guidance). Relm4 is an Elm-style layer, not a different renderer.

---

## 7. Flutter desktop + flutter_rust_bridge

### Primary sources

- Flutter desktop: https://docs.flutter.dev/platform-integration/desktop — compile native Windows, macOS, or Linux from one project; `flutter build windows|macos|linux`
- flutter_rust_bridge: https://github.com/fzyzcjy/flutter_rust_bridge/ — “Android, iOS, Windows, Linux, MacOS, and Web”; codegen `create` / integrate
- Cross-platform overview: https://cjycode.com/flutter_rust_bridge/guides/cross-platform/overview
- Text display: https://api.flutter.dev/flutter/painting/TextSpan-class.html — styled spans; use `RichText` to display
- Editing: https://api.flutter.dev/flutter/widgets/EditableText-class.html — editable text with a **`TextStyle style`** (single style for the field in the public constructor surface documented there)

### Fit

- Shared Dart UI codebase across three desktops; `nextguide-gde` linked as a **Rust native library** called from Dart (FRB) — **in-process** with the Flutter engine, but the **UI host language is Dart**, not Rust.
- Rich text: first-party story is strong for **rendering** mixed styles (`TextSpan`); **editing** mixed styles is not the same as dropping in `EditableText` (single `TextStyle`). A pragmatic subset may need a custom editing pipeline or packages beyond what this inventory verified from Flutter’s own widget API pages.

---

## 8. Electron + Rust via Node-API (napi-rs)

### Primary sources

- Electron intro: https://www.electronjs.org/docs/latest — “JavaScript, HTML, and CSS”; Chromium + Node; Windows, macOS, Linux
- Packaging: https://www.electronjs.org/docs/latest/tutorial/tutorial-packaging — Electron Forge; OS installers; code signing guidance
- napi-rs Electron notes: https://napi.rs/docs/more/integrations — load `.node` in main/preload; `asarUnpack`; platform-specific binaries

### Fit

- Rich-text: full Chromium editing (same as any web app).
- Rust library: compile `nextguide-gde` (or a thin FFI crate) as a Node-API addon and load it **in-process with Electron’s Node**. This satisfies “in-process link” in a literal sense but **fails “first-class Rust host”** — the shell is Electron/JS.

---

## 9. GPUI (Zed Industries)

### Primary sources

- Site: https://gpui.rs/
- Crate: https://crates.io/crates/gpui (License: **Apache-2.0**; “still in active development… **pre-1.0**… breaking changes”)
- Tree README: https://github.com/zed-industries/zed/tree/main/crates/gpui — macOS / Linux / Windows feature notes for `gpui_platform`

### Fit

- Pure Rust GPU UI used to ship the Zed editor — credible as a **Rust host** with real desktop shipping experience behind it.
- Maturity for **third-party** apps: explicitly pre-1.0.
- Rich-text prose editing: not documented as a stock “RTF body” widget; Zed’s editing is the framework’s proving ground (code-centric). Expect custom text views.

---

## 10. CXX-Qt + Qt

### Primary sources

- CXX-Qt book: https://kdab.github.io/cxx-qt/book/ — safe Rust↔Qt via CXX; CI on Linux, Windows, macOS; “early development and the API changes frequently”
- Repository: https://github.com/KDAB/cxx-qt — MIT OR Apache-2.0; supports building Rust apps with Cargo or CMake
- Qt licensing: https://doc.qt.io/qt-6/licensing.html — Commercial **or** LGPLv3 (some modules GPLv3-only)
- Qt Quick `TextEdit`: https://doc.qt.io/qt-6/qml-qtquick-textedit.html — “Displays multiple lines of **editable formatted text**”; `textFormat`, `textDocument`, etc.

### Fit

- Rich-text editing is a **documented Qt strength** (`TextEdit` / text document APIs).
- Rust integrates by implementing `QObject`s in Rust and driving QML/Widgets — `nextguide-gde` stays in-process on the Rust side of the bridge.
- Cost center: **Qt licensing/compliance** (LGPL obligations or commercial) + C++/Qt toolchain, not just Cargo.

---

## 11. Floem

### Primary sources

- https://github.com/lapce/floem — MIT; Windows, macOS, Linux; “still maturing… occasional breaking changes… on our way to v1”
- https://lap.dev/floem/ — desktop Win/macOS/Linux; experimental web; mobile planned
- https://crates.io/crates/floem — 0.2.0 (as fetched)

### Fit

- Credible **young** pure-Rust desktop option with in-process linking.
- No first-party rich-text editor surfaced in the README/product pages reviewed; treat rich body editing as custom work. Lower maturity than egui/iced/Slint 1.x for a product bet.

---

## Options considered and de-emphasized

| Candidate | Why de-emphasized (primary-source basis) |
|-----------|------------------------------------------|
| **Xilem** | Still positioned as next-gen / research relative to production editors; not inventoried in depth here without a clear “ship desktop app” packaging+widget story matching the above. |
| **Pure C++/C#/.NET MAUI/WPF-only** | No first-class shared Rust host without an FFI story equivalent to FRB/CXX-Qt; out of “Rust host or in-process library” preference unless bridged. |
| **Mobile-only / web-only** | Outside Editor v1 desktop destination. |

---

## Decision-useful axes (no ranking)

When a later grilling ticket picks among these, primary sources above support asking:

1. **Is the process host Rust?** (Tauri, Dioxus, egui, iced, Slint, GPUI, Floem, Relm4/gtk-rs, CXX-Qt-as-Rust-app: yes. Flutter: Dart. Electron: Node.)
2. **Is rich-text editing a stock widget?** (GTK TextView+tags; Qt TextEdit formatted; webview/Electron HTML: yes. Slint/iced/egui stock: mostly plain; custom or web escape hatches.)
3. **What is the license coupling?** (Permissive Rust GUIs vs Slint tri-license vs GTK LGPL vs Qt LGPL/commercial vs webview OS components.)
4. **Who owns installers?** (Tauri/Dioxus/Flutter/Electron first-party distribute stories vs “ship a binary” Rust GUIs vs GTK/Qt runtime bundling.)

---

## Source index (URLs)

| Topic | URL |
|-------|-----|
| Tauri 2 home | https://v2.tauri.app/ |
| Tauri process model | https://v2.tauri.app/concept/process-model/ |
| Tauri distribute | https://v2.tauri.app/distribute/ |
| Tauri prerequisites | https://v2.tauri.app/start/prerequisites/ |
| tauri crate | https://crates.io/crates/tauri |
| Dioxus getting started | https://dioxuslabs.com/learn/0.6/getting_started/ |
| dioxus crate | https://crates.io/crates/dioxus |
| egui | https://github.com/emilk/egui/ |
| eframe | https://crates.io/crates/eframe |
| egui TextEdit | https://docs.rs/egui/latest/egui/widgets/text_edit/struct.TextEdit.html |
| egui_richedit | https://docs.rs/egui_richedit/latest/egui_richedit/ |
| iced | https://iced.rs/ / https://crates.io/crates/iced |
| iced TextEditor | https://docs.rs/iced/latest/iced/widget/text_editor/struct.TextEditor.html |
| Slint | https://slint.dev/ |
| Slint Rust docs | https://docs.slint.dev/latest/docs/rust/slint/ |
| Slint TextEdit | https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/textedit/ |
| Slint FAQ licenses | https://raw.githubusercontent.com/slint-ui/slint/master/FAQ.md |
| Slint rich-text issue | https://github.com/slint-ui/slint/issues/2723 |
| GTK4 overview | https://docs.gtk.org/gtk4/overview.html |
| GTK text widgets | https://docs.gtk.org/gtk4/section-text-widget.html |
| gtk4-rs | https://github.com/gtk-rs/gtk4 |
| Relm4 book | https://relm4.org/book/stable/ |
| Flutter desktop | https://docs.flutter.dev/platform-integration/desktop |
| flutter_rust_bridge | https://github.com/fzyzcjy/flutter_rust_bridge/ |
| Flutter TextSpan | https://api.flutter.dev/flutter/painting/TextSpan-class.html |
| Flutter EditableText | https://api.flutter.dev/flutter/widgets/EditableText-class.html |
| Electron | https://www.electronjs.org/docs/latest |
| Electron packaging | https://www.electronjs.org/docs/latest/tutorial/tutorial-packaging |
| napi-rs integrations | https://napi.rs/docs/more/integrations |
| GPUI | https://gpui.rs/ / https://crates.io/crates/gpui |
| GPUI in Zed tree | https://github.com/zed-industries/zed/tree/main/crates/gpui |
| CXX-Qt book | https://kdab.github.io/cxx-qt/book/ |
| Qt licensing | https://doc.qt.io/qt-6/licensing.html |
| Qt Quick TextEdit | https://doc.qt.io/qt-6/qml-qtquick-textedit.html |
| Floem | https://github.com/lapce/floem |
