Type: grilling
Status: resolved
Blocked by: 02, 03

## Question

What **rich-text pipeline** should Editor v1 use to show and edit Node bodies under the pragmatic-subset bar, and how do load/save Round-trips of RTF bytes stay honest with Format v1?

Use the RTF interop inventory and the chosen UI toolkit. Decide the editor representation (HTML, native document model, etc.), conversion boundaries, and what happens when exotic The Guide RTF cannot be represented in the subset.

## Answer

**Pure-Rust RTF ↔ GTK `TextBuffer` / `TextTag`**, via a sibling library crate **`nextguide-rtf`** (editor uses it; Format v1 codec stays opaque-bytes).

- **No** HTML/webview, per-OS Rich Edit/Cocoa as the cross-platform path, or Pandoc/LibreOffice as a hard dependency.
- **Untouched bodies:** preserve original opaque RTF bytes on save; rewrite only bodies the editor changed.
- **Edit = subset rewrite:** bold, italic, underline, paragraphs, simple lists (+ plain text). Colours, highlights, hyperlinks, tables, images, non-default fonts: best-effort display only; may be lost once that body is edited.
- **Unparseable RTF:** soft UI fallback; keep original bytes until the user intentionally edits/replaces the body.
- **New/empty bodies:** minimal valid The Guide–style empty RTF, not zero-length bytes.
