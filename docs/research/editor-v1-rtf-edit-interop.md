# Editor v1: RTF ↔ editor interop inventory

**Date:** 2026-10-31  
**Ticket:** `.scratch/editor-v1/issues/02-rtf-edit-interop-inventory.md`  
**Scope:** Practical **RTF ↔ editor** interop paths for Editor v1’s pragmatic rich-text subset (edit bold/italic/lists/basic formatting; best-effort display of exotic RTF from The Guide) in a **Rust-hosted** desktop app (Windows, macOS, Linux). Inventory only — **no pipeline choice**.

## Verdict

Four families of paths are real and documented:

1. **Pure-Rust RTF crates** that parse (and sometimes write) clipboard-/subset-grade RTF into an owned document model, then drive whatever UI the toolkit provides.
2. **Convert-through an intermediate** (most often **HTML**, sometimes Markdown/JSON) so a **webview** editor (TipTap/ProseMirror/`contenteditable`) can edit a pragmatic subset, with a separate RTF→intermediate and intermediate→RTF stage.
3. **OS-native rich-text controls** that already speak RTF on **Windows** (Rich Edit `EM_STREAMIN`/`EM_STREAMOUT`) and **macOS** (`NSAttributedString` / Cocoa text system / `textutil`), with **no first-class GTK RTF widget** on Linux.
4. **External converters** (Pandoc, LibreOffice Writer filters, macOS `textutil`) as host-side tools rather than in-process libraries.

Format v1 still treats node bodies as **opaque RTF bytes** for Structural Round-trip; any edit path that rewrites RTF must keep The Guide 2.0’s Rich Edit reader happy, and untouched bodies should stay byte-identical.

## 1. What The Guide / Format v1 constrain

### 1.1 On-disk body = opaque RTF payload

| Claim | Source |
|-------|--------|
| Core model is tree + titles + each node’s RTF body; Format v1 Round-trip does **not** parse RTF | `GLOSSARY.md` (Core model, Format v1, Structural Round-trip) |
| On-disk `text_bytes` are node body bytes (RTF); opaque for Core model | `docs/research/gde-v2-primary-sources.md` §1.4–1.5 |
| `guide_nodedata_t.text` documented as UTF-8; sample files contain `{\rtf1...` | The Guide 2.0 `libguide/include/libguide/guide.h` (`text` field comment); same research note |
| Public Format v1 API: opaque `rtf_body` bytes; no RTF interpret/mutate API | `.scratch/format-v1/spec.md`; `crates/nextguide-gde/src/model.rs` |

**Interop implication:** load→save without body edits must preserve RTF bytes (Structural Round-trip). Editing a body replaces those bytes; Preserve-through of non-Core `.gde` attrs is orthogonal but still required for Round-trip honesty.

### 1.2 Historical writer = Windows Rich Edit (Msftedit)

| Claim | Source |
|-------|--------|
| Product is a two-pane **rich-text** outliner | The Guide 2.0 `README` (sandeep-datta/winguide mirror) |
| Right pane is `CGuideView : public CRichEditView`; document is `CRichEditDoc` | `guide/GuideView.h`, `guide/GuideDoc.cpp` |
| Window class forced to `RICHEDIT50W` | `GuideView.cpp` (`m_strClass = RICHEDIT50W`) |
| Persist: `GetRTF` → `EM_STREAMOUT` + `SF_RTF`; load: `SetRTF` → `EM_STREAMIN` + `SF_RTF` | `guide/LeftView.cpp` (`GetRTF` / `SetRTF`) |
| Empty / startup bodies are minimal RTF literals (`{\rtf1}`, ansi paragraph) | `GuideView.cpp` (`theEmptyRTFText`, `theStartupRTFText`) |
| Fixture bodies carry `{\*\generator Msftedit 5.41.21.2500;}` plus list (`\pn` / `\pntext`), color, highlight | Local `data/ImportSample.gde` (inspected binary) |

**Interop implication:** Round-trip with The Guide means emitted RTF must be acceptable to **Windows Rich Edit 5.x / Msftedit**, including common WordPad-style list markup — not only a theoretical RTF 1.9.1 subset. Apple Cocoa and Qt do **not** define that writer.

### 1.3 RTF language itself

Microsoft’s final published spec is **RTF 1.9.1** (Word 2007 era, March 2008). Old readers ignore unknown control words; that is the format’s forward-compat story, not a guarantee of lossless Word-grade fidelity.

- Spec PDF (archive): https://officeprotocoldoc.z19.web.core.windows.net/files/Archive_References/[MSFT-RTF].pdf  
- Cited from Microsoft Open Specs informative references: https://learn.microsoft.com/en-us/openspecs/exchange_server_protocols/ms-oxrtfcp/85c0b884-a960-4d1a-874e-53eeee527ca6  

## 2. Path A — Pure-Rust RTF crates

Inventory of crates that own RTF bytes in-process (crates.io / docs.rs / project READMEs). None of these is The Guide’s codec; all are third-party libraries evaluated as **interop ingredients**.

| Crate | Role (from primary docs) | Writer? | Stated limits relevant to Editor v1 |
|-------|--------------------------|---------|-------------------------------------|
| **`rtf-parser`** 0.4.3 (MIT) | Lexer + parser → `RtfDocument`; claims RTF 1.9; UTF-16/`\uN`; optional WASM | No (parse-focused) | `\bin` not handled (can mess lexing); base64 images not supported but “safely parsed”. Takes `&str` input in public examples. https://crates.io/crates/rtf-parser · https://github.com/d0rianb/rtf-parser |
| **`rtf-parser-tt`** 0.6 (MIT, fork) | Same family; code-page decoding; paragraphs/tables/footnotes; errors instead of panic | No | Maintainer README: real-world focus; CJK pages via optional `encoding_rs`. https://crates.io/crates/rtf-parser-tt |
| **`rtf-to-html`** 0.1.x (MIT/Apache-2.0) | `rtf_to_html` / `rtf_to_plain_text` over `rtf-parser-tt` | HTML out only | Explicitly **no** lists, colors/fonts sizes, hyperlinks, tables, images. Maps `\b`/`\i`/`\ul`/`\par` → `<strong>`/`<em>`/`<u>`/`<p>`. https://crates.io/crates/rtf-to-html |
| **`rtf-grimoire`** 0.2.1 (MIT) | Tokenizer only (`nom`); enables custom parsers | No | No `\uN`/`\uc` semantics — caller’s job. Last publish 2023. https://crates.io/crates/rtf-grimoire |
| **`scrivener-rtf`** 0.1.0 (MIT) | AST parse **and** `Document::to_rtf` generate; Unicode/`\uN` claimed | Yes (AST round-trip) | Very new / low download count; oriented to Scrivener workflows. https://crates.io/crates/scrivener-rtf |
| **`rclip-rtf`** 0.1.0 (MIT OR Apache-2.0) | RTF **1.9.1** tokenizer + clipboard-grade styled runs; optional `alloc` **Writer** / `Document::to_rtf` | Yes | Self-documented **phase-0** gaps: **no list/table/picture model**; paragraph props not modelled; `\pn` contributes no structure; writer omits paragraph properties. Strong `\ucN` / code-page story; AppKit oracle notes (`\highlight` vs `\cb`). https://crates.io/crates/rclip-rtf · https://docs.rs/rclip-rtf |

**Practical shape for Editor v1:** Rust host parses RTF → internal run/block model → toolkit-specific view; on save, emit RTF from the subset (or from a writer crate). Exotic Guide features (Msftedit lists, highlights, hyperlinks) need **best-effort** display rules and an explicit policy for “untouched bytes vs rewrite” (later grilling / pipeline ticket).

**Known limits (crate-class):**

- Most popular parsers are **read-heavy**; few mature **subset writers** aimed at Rich Edit.
- List support in ImportSample-style `\pn` markup is weak or absent in the HTML converter and in `rclip-rtf`’s modelled surface.
- Unicode is a recurring failure mode across older crates (`\ucN` skip, code pages, surrogates); `rclip-rtf`’s README is the most explicit primary-source critique of peers.

## 3. Path B — Convert through HTML (or similar) for a webview editor

### 3.1 Web editors speak HTML/JSON, not RTF

| Surface | Interchange | Source |
|---------|-------------|--------|
| TipTap | `getHTML` / `setContent(html)`; `generateJSON` / `generateHTML` from HTML | https://tiptap.dev/docs/guides/output-json-html · https://tiptap.dev/docs/editor/api/utilities/html |
| TipTap Conversion product | DOCX / Markdown / PDF / ODT / EPUB — **not RTF** | https://tiptap.dev/docs/conversion/getting-started/overview |
| ProseMirror (via TipTap) | Schema drops unknown tags/attrs quietly when loading HTML | TipTap migration notes (same HTML guide) |

**Interop shape:** `RTF bytes → (Rust or tool) → HTML subset → webview editor → HTML → (Rust or tool) → RTF bytes`. The RTF↔HTML bridges are **not** provided by TipTap.

### 3.2 In-process RTF→HTML (Rust)

- **`rtf-to-html`**: basic character formatting + paragraphs only; **lists unsupported** (crate README) — below Editor v1’s “edit lists” bar unless extended.
- Custom mapping from `rtf-parser` / `rtf-parser-tt` / `rclip-rtf` runs → HTML is feasible but is **application code**, not a finished product.

### 3.3 Host / external HTML bridges

| Tool | Capability | Source |
|------|------------|--------|
| **Pandoc** | `rtf` is both **input and output**; FAQ: use `-s/--standalone` for valid RTF; RTF **reader targets RTF 1.5** (module comment) | https://pandoc.org/MANUAL.html (`-f`/`-t` lists); https://www.pandoc.org/faqs.html; https://hackage.haskell.org/package/pandoc-3.6/docs/src/Text.Pandoc.Readers.RTF.html |
| **LibreOffice** | Writer filter **“Rich Text Format”** (`.rtf`); CLI `--convert-to` / filter tables include HTML ↔ RTF | https://help.libreoffice.org/latest/en-US/text/shared/guide/convertfilters.html · https://help.libreoffice.org/latest/sid/text/shared/guide/start_parameters.html |
| **macOS `textutil`** | `-convert` / `-cat` among `txt`, `html`, `rtf`, `rtfd`, … via Cocoa text system | `man textutil` (Darwin) |

**Limits:** every HTML hop is **lossy** relative to Msftedit RTF (generator destination, exact `\pn` list encoding, highlight vs `\cb`, protected/link char formats The Guide clears in `FixSomeStuff`). Pandoc’s RTF **1.5** reader vs Guide **Msftedit** bodies is a version skew. External tools add packaging/weight and are awkward as a hard dependency for a shippable desktop app, but remain valid **interop** options (e.g. offline import assist).

## 4. Path C — OS-native rich-text controls

### 4.1 Windows — Rich Edit (matches The Guide)

| API | Role | Source |
|-----|------|--------|
| `EM_STREAMIN` / `EM_STREAMOUT` with `SF_RTF` | Stream RTF into/out of a rich edit control | https://learn.microsoft.com/en-us/windows/win32/controls/em-streamin · https://learn.microsoft.com/en-us/windows/win32/controls/use-streams |
| `EDITSTREAM` + callback | Buffer pump for stream in/out | https://learn.microsoft.com/en-us/windows/win32/api/richedit/ns-richedit-editstream |
| MFC `CRichEditCtrl::StreamIn` / `StreamOut` | Same messages wrapped | Microsoft cpp-docs `cricheditctrl-class.md` |

This is the **same class of control** The Guide uses (`CRichEditView` / `RICHEDIT50W`). Highest fidelity for exotic Guide RTF **on Windows only**. Cross-platform Editor v1 cannot rely on Rich Edit alone for macOS/Linux.

### 4.2 macOS — AppKit attributed strings / NSTextView

| API | Role | Source |
|-----|------|--------|
| `NSAttributedString` init/write with `NSRTFTextDocumentType` (and related document APIs) | Read/write RTF (RTFD for attachments) | Apple archive: [RTF Files and Attributed Strings](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/AttributedStrings/Tasks/RTFAndAttrStrings.html); [Formatted Documents and Attributed Strings](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/AttributedStrings/Articles/HTMLFilesandAttributedStrings.html) |
| `NSTextView` | UI front-end for the Cocoa text system | https://developer.apple.com/documentation/appkit/nstextview |
| `textutil` | CLI over the same Cocoa mechanisms | `man textutil` |

**Limits (primary / observed in crate oracle notes):** Apple extends RTF; AppKit treats `\cb` vs `\highlight` differently than Word/RichEdit (`rclip-rtf` docs.rs / crates.io README — verified against `NSAttributedString`). Round-trip through Cocoa then back to Msftedit can alter colour/list encoding even when visually close.

### 4.3 Linux — GTK TextView (no built-in RTF)

| Fact | Source |
|------|--------|
| No ready-made GTK3 rich-text-editor widget; apps use `GtkTextBuffer` tags or custom UI | GNOME Discourse (Emmanuele Bassi): https://discourse.gnome.org/t/is-richtexteditor-available-in-gtk3/29540 |
| `gtk_text_buffer_deserialize` requires a **registered** format — not RTF by default | https://docs.gtk.org/gtk3/method.TextBuffer.deserialize.html |
| **Ratify** — RTF import/export for `GtkTextBuffer` (third-party library) | https://github.com/ptomato/ratify |

**Interop implication:** Linux needs either Ratify-like RTF↔buffer code, a pure-Rust RTF path, a webview+HTML path, or an external converter — there is no system Rich Edit equivalent.

### 4.4 Toolkit note: Qt Scribe (common Rust desktop host)

Qt’s `QTextDocument` serializes **HTML** and **Markdown** only — **not RTF** (`Rich Text Processing` / `QTextDocumentWriter` supported formats: plaintext, HTML, markdown, ODF). https://doc.qt.io/qt-6/richtext.html · https://doc.qt.io/qt-6/qtextdocumentwriter.html  

So a Qt- or iced-/egui-style host still needs Path A/B (or platform widgets) for Guide RTF; Qt’s rich editor alone is an **HTML-subset** editor unless paired with a converter.

## 5. Lossiness / Round-trip honesty matrix (decision fuel, not a pick)

| Scenario | Opaque RTF bytes | Pragmatic subset edit | Exotic Guide RTF display |
|----------|------------------|-----------------------|---------------------------|
| Never open body in editor | Preserve bytes (Format v1 Structural) | n/a | n/a |
| Windows Rich Edit host | Stream-out may **normalize** RTF even for “no edits” (control rewrite) — The Guide itself streams via Rich Edit | High for bold/italic/lists Rich Edit supports | Best native fidelity |
| Cocoa `NSAttributedString` | Rewrite ≠ Msftedit byte-identical | Good for basic attrs; lists/colours diverge | Best-effort; Apple extensions |
| Pure-Rust parse → subset write | Only if policy keeps original bytes when clean | Depends on writer completeness | Drop or approximate unknown controls |
| RTF→HTML→editor→RTF | Almost never byte-stable | Works if HTML subset ⊇ edit bar; **lists** need a bridge beyond `rtf-to-html` today | Lossy; show fallback plaintext or degraded HTML |
| Pandoc / LibreOffice | Tool-dependent normalization | Broad, not Guide-tuned | Broad, not Guide-tuned |

**Product glossary reminder:** **Round-trip** means Guides open correctly both ways for the **supported** feature set (`GLOSSARY.md`), not Byte-stable identity of RTF after edits. Format v1’s Structural Round-trip still requires **unchanged** opaque bodies to survive load→save when the editor did not mutate them.

## 6. Implications for later Editor v1 tickets (non-decisions)

1. Pipeline grilling (`05-rich-text-pipeline`) must pick among (or combine) Paths A–C; this inventory does **not**.
2. Any path that always re-serializes RTF on node focus will fight Format v1 Structural tests unless the codec layer distinguishes “touched” bodies.
3. List editing is the sharp edge: Guide fixtures use Rich Edit `\pn` lists; several Rust HTML bridges omit lists entirely.
4. Cross-platform parity cannot be “always use system RTF control” — Linux lacks one; Qt lacks RTF I/O.

## 7. Sources consulted

### NextGuide / Format v1

- `GLOSSARY.md`
- `docs/research/gde-v2-primary-sources.md`
- `.scratch/format-v1/spec.md`
- `crates/nextguide-gde/src/model.rs`
- `data/ImportSample.gde` (local fixture bytes)

### The Guide 2.0 (Apache-2.0 mirror `sandeep-datta/winguide`)

- `README`, `libguide/include/libguide/guide.h`
- `guide/GuideView.h`, `guide/GuideView.cpp`, `guide/GuideDoc.cpp`
- `guide/LeftView.cpp` (`GetRTF` / `SetRTF`, `EM_STREAMIN`/`OUT`)
- `guide/Utils.cpp` (print concatenator / RTF helpers)

### RTF specification & OS APIs

- Microsoft RTF 1.9.1 PDF (archive link above); MS-OXRTFCP informative references
- Microsoft Learn: Rich Edit streams (`EM_STREAMIN`, `EDITSTREAM`, “Use Streams”)
- Apple: NSAttributedString RTF docs; NSTextView documentation; Darwin `man textutil`
- GTK: `GtkTextBuffer.deserialize`; GNOME Discourse (Bassi); Ratify README
- Qt 6: Rich Text Processing; `QTextDocument` / `QTextDocumentWriter`

### Crates & converters

- crates.io / docs.rs / GitHub READMEs: `rtf-parser`, `rtf-parser-tt`, `rtf-to-html`, `rtf-grimoire`, `scrivener-rtf`, `rclip-rtf`
- TipTap docs (HTML utilities, output guide, Conversion overview)
- Pandoc User’s Guide + FAQs; Pandoc RTF reader source header (RTF 1.5 target)
- LibreOffice help: convert filters / start parameters

### Secondary (navigation only)

- Qt Wiki “Handling Document Formats” (RTF listed as needing external handling relative to Scribe)
- Blog / third-party Rich Edit anecdotes — not used as normative claims
