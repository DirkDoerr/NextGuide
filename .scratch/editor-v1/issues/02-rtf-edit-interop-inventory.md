Type: research
Status: resolved

## Question

What practical **RTF ↔ editor** interop paths exist for Editor v1’s pragmatic rich-text subset (edit bold/italic/lists/basic formatting; best-effort display of exotic RTF from The Guide), in a Rust-hosted desktop app?

Inventory approaches with primary sources: pure-Rust RTF crates, convert-through-HTML (or another intermediate) for a webview editor, OS-native rich-text controls, and any The Guide / historical notes that constrain Round-trip. Capture known limits (lossy conversion, Unicode, lists). Do not pick the pipeline — that is a later grilling ticket.

## Answer

Four interop families, no pipeline pick: (1) pure-Rust RTF parse/write crates (`rtf-parser` family, `rclip-rtf`, etc.); (2) RTF↔HTML (or tool) for webview editors (TipTap speaks HTML/JSON, not RTF); (3) OS-native RTF — Windows Rich Edit (what The Guide uses), macOS `NSAttributedString`/`textutil`, Linux needs Ratify/custom (no GTK RTF widget); (4) external converters (Pandoc, LibreOffice). Format v1 keeps untouched bodies opaque; Guide fixtures are Msftedit/`\pn` RTF. Full inventory: [`docs/research/editor-v1-rtf-edit-interop.md`](../../../docs/research/editor-v1-rtf-edit-interop.md).
