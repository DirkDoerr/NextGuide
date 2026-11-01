Status: done

# Editor v1 — Core-model Guide editor (GTK)

## Problem Statement

I can Round-trip The Guide **v2** `.gde` files with Format v1’s library and CLI, but I still cannot open, navigate, and edit Guides as a hierarchical rich-text outliner on my desktop. I need a first NextGuide editor that delivers The Guide’s **functionality** for the Core model (tree, titles, bodies) with Round-trip honesty — without cloning The Guide’s Windows UI, without web tech, and without waiting on mobile or full attribute/RTF parity.

## Solution

Ship **Editor v1**: a GTK4 + Relm4 desktop app binary **`nextguide`** that loads and saves Guides via Format v1’s Core model, hides the Dummy root in the UI, edits titles and a pragmatic RTF subset (via **`nextguide-rtf`** into GTK text tags), restructures Nodes through an extended `nextguide-gde` mutation API, and preserves non-Core attributes through edits. Rename the Format v1 Round-trip CLI to **`nextguide-cli`**. First release installers target Windows and macOS with a bundled GTK runtime; Linux packaging follows later while the codebase stays cross-platform.

## User Stories

1. As a Guide owner, I want to open an existing The Guide v2 `.gde` in NextGuide, so that I can work without The Guide’s Windows app.
2. As a Guide owner, I want to save my Guide back to `.gde`, so that changes persist and Round-trip to The Guide for the supported feature set.
3. As a Guide owner, I want Save As to a new path, so that I can duplicate or relocate a Guide.
4. As a Guide owner, I want to create a new empty Guide, so that I can start writing without an existing file.
5. As a Guide owner, I want a two-pane layout (tree left, body right), so that I get The Guide’s outliner functionality without its Windows chrome.
6. As a Guide owner, I want to navigate the Node tree and select one Node at a time, so that I can see and edit that Node’s body.
7. As a Guide owner, I want Node titles visible and editable in the tree, so that structure and labels stay together.
8. As a Guide owner, I want to edit a Node’s rich-text body with bold, italic, underline, paragraphs, and simple lists, so that everyday formatting works.
9. As a Guide owner, I want to add, delete, rename, reorder, and reparent Nodes via menu/commands, so that I can restructure the Guide.
10. As a Guide owner, I want the Dummy root hidden in the UI, so that I only see user-facing Nodes.
11. As a Guide owner, I want non-Core attributes (icons, colors, etc.) preserved when I edit titles, bodies, or tree topology, so that The Guide does not lose them on Round-trip.
12. As a Guide owner, I want untouched Node bodies kept as original RTF bytes, so that exotic The Guide markup is not rewritten until I edit that body.
13. As a Guide owner, I want a dirty indicator and Save/Discard/Cancel prompts on New, Open, and quit, so that I do not lose work accidentally.
14. As a Guide owner, I want explicit save only (no autosave), so that file writes are intentional.
15. As a Guide owner, I want Undo and Redo for body edits and for tree/title edits, so that mistakes are recoverable.
16. As a Guide owner, I want standard desktop shortcuts for file and edit commands, so that the app feels familiar.
17. As a Guide owner, I want a recent-files list of the last five Guides, so that I can reopen work quickly.
18. As a Guide owner, I want `nextguide path.gde` to open that Guide on launch, so that I can start from the shell or file associations later.
19. As a Guide owner, I want bare `nextguide` to start an untitled New Guide, so that cold launch is useful.
20. As a Guide owner, I want a resizable tree/body split, so that I can favour navigation or writing.
21. As a Guide owner, I want unparseable RTF to fail soft in the UI without crashing, so that odd files remain usable.
22. As a Guide owner, I want new/empty bodies stored as minimal valid RTF, so that The Guide accepts them.
23. As a NextGuide developer, I want Core-model mutations in `nextguide-gde` addressed by Node path, so that the editor and tests share one tree API.
24. As a NextGuide developer, I want mutation ops to fail closed and apply atomically with typed invalid-path/edit errors, so that the Guide never ends half-updated.
25. As a NextGuide developer, I want Preserve-through bags to stay private on the Core-model API, so that the public surface stays small.
26. As a NextGuide developer, I want `nextguide-rtf` as a library crate for RTF ↔ edit-model conversion, so that the Relm4 UI stays thin and conversion is testable without the widget tree.
27. As a NextGuide developer, I want `nextguide-rtf`’s public API free of GTK types, so that conversion stays toolkit-agnostic at the seam.
28. As a NextGuide developer, I want the editor to map GTK `TextBuffer`/`TextTag` to that edit model, so that Relm4 owns widgets only.
29. As a NextGuide developer, I want the Format v1 CLI renamed to `nextguide-cli`, so that the product binary name `nextguide` means the editor.
30. As a NextGuide developer, I want Format v1 Structural and Byte-stable CI gates kept green, so that editor work does not regress the codec.
31. As a CI job, I want mutation and `nextguide-rtf` tests to prove edit Round-trip honesty without The Guide GUI, so that pipelines stay portable.
32. As a CI job, I want only thin UI smoke for `nextguide` (if headless GTK is practical), so that CI does not own a full widget matrix.
33. As a packager, I want Windows and macOS installers that bundle GTK with `nextguide`, so that users are not told to hunt a runtime.
34. As a packager, I want `nextguide-cli` in the same release artifacts, so that Round-trip tooling ships with the editor.
35. As a packager, I want signing/notarization optional for early releases, so that shipping is not blocked on certificates.
36. As a Guide owner, I want manual updates via releases (no in-app updater yet), so that Editor v1 stays simple.
37. As a NextGuide developer, I want Linux packaging deferred while the GTK codebase still builds for Linux, so that first ship can focus on Win/macOS.
38. As a Guide owner, I want single selection only in Editor v1, so that body editing stays unambiguous.
39. As a Guide owner, I want an empty tree and disabled/empty body for a new Guide until I add a Node, so that empty state is clear.
40. As a Guide owner, I want window title to show the file name or Untitled plus a dirty mark, so that I know what I am editing.
41. As a NextGuide developer, I want move/reparent to carry Preserve-through bags with the Node, so that attributes survive restructuring.
42. As a NextGuide developer, I want delete to drop bags for removed Nodes, so that we do not keep orphaned opaque state.
43. As a NextGuide developer, I want new Nodes to start with empty Preserve-through bags, so that we do not invent non-Core attributes.
44. As a documentation reader, I want Editor v1 documented as GTK/Relm4 with no web shell (ADR-0003), so that stack choice is obvious.
45. As a product owner, I want full-text search, drag-and-drop reparent, themes, print, deep a11y audit, and The Guide shortcut parity out of this slice, so that Editor v1 can finish.
46. As a Guide owner, I want colours/highlights/hyperlinks/tables/images outside the edit subset to display best-effort and possibly drop on rewrite after I edit that body, so that expectations stay honest.
47. As a human tester, I want optional open-in-The-Guide checks after meaningful releases, so that real Round-trip gets a human gate without CI depending on it.
48. As a NextGuide developer, I want tree undo as an app-level stack separate from GTK body undo, so that structure edits undo without boiling a unified document history.
49. As a NextGuide developer, I want undo depth capped modestly (~50), so that memory stays bounded.
50. As a NextGuide developer, I want Edit-menu Undo/Redo to follow focus (body vs tree), so that the active stack is predictable.
51. As a Guide owner, I want one Guide per window (no tabs/MDI), so that session rules stay simple.
52. As an agent implementer, I want this spec marked ready-for-agent with wayfinder decisions settled, so that implementation does not re-litigate product calls.

## Implementation Decisions

- Deliver Editor v1 as a Cargo workspace evolution: keep `nextguide-gde`; add `nextguide-rtf`; add Relm4/GTK editor binary **`nextguide`**; rename Format v1 CLI crate/binary to **`nextguide-cli`** (dump/check behaviour unchanged aside from the name).
- UI stack: **GTK4 + Relm4**, no webview/HTML UI (ADR-0003). Dynamically link GTK; bundle GTK runtime on Windows and macOS installers.
- Extend `nextguide-gde` with Core-model mutation: public `NodePath`; `Guide::new_empty`; `set_title` / `set_rtf_body`; `insert_child`; `remove_node` (refuse Dummy root); `move_node` with same-parent index adjustment; index `len` = append. New Nodes: empty title, empty/minimal RTF via editor policy, no children, empty Preserve-through bag. Ops atomic; typed invalid-path/invalid-edit errors (not overloaded Corrupt).
- Dummy root remains in the library tree; UI never shows or selects it; visible rows map to Node paths under the Dummy root (first row ↔ `[0]`).
- Preserve-through: bags stay private; kept across title/RTF edits and moves; dropped on delete; Guide-level attrs survive tree/body edits.
- `nextguide-rtf`: pure-Rust RTF ↔ toolkit-agnostic edit model for pragmatic subset (bold/italic/underline/paragraphs/simple lists); editor applies model to GTK `TextBuffer`/`TextTag`. Untouched bodies keep original opaque bytes; edited bodies rewrite subset RTF. Unparseable RTF: soft UI fallback; preserve bytes until intentional edit. New/empty bodies: minimal valid The Guide–style empty RTF.
- Layout: two-pane resizable split (~⅓ tree / ~⅔ body); titles in tree; single selection; menubar/toolbar; menu/command restructure (no drag-and-drop).
- Session: one Guide per window; dirty tracking; Save / Save As; New/Open/quit prompts; untitled until Save As; `nextguide [path]` opens file; bare launch = New; recent files (5); explicit save only.
- Undo/redo: two stacks — GTK body undo while focused; app-level tree/title stack; ~50 depth; Edit menu + standard shortcuts; focus selects active stack; changing Node selection seals body undo for that view as usual.
- Packaging: first release artifacts Windows (MSI or NSIS) and macOS (`.app` in DMG) including `nextguide` + bundled GTK + `nextguide-cli`; manual updates; signing optional; Linux installers deferred.
- Fixtures remain under `data/`; do not regress Format v1 Structural/Byte-stable gates.
- Respect ADR-0001/0002 for the codec layer; Editor v1 may add justified dependencies (gtk-rs/Relm4, RTF crates) outside the Format v1 minimal-deps hard line for the codec crate itself.

## Testing Decisions

- Good tests assert **external behaviour** only: Core-model observables after mutations, Round-trip helpers, error categories, RTF subset convert results / untouched-byte policy, CLI exit codes after rename, and thin UI smoke — not private Preserve-through bag layout or GTK widget internals.
- **Primary seam A:** `nextguide-gde` public API (existing load/save/Round-trip gates **plus** mutation API and Preserve-through-under-edit outcomes observable via load/save).
- **Primary seam B:** `nextguide-rtf` public API (subset round-trips, untouched-bytes policy, soft-fail unparseable, empty RTF). Prefer no GTK types on this public surface.
- **Secondary:** `nextguide-cli` smoke after rename (dump/check still delegate to library helpers).
- **Tertiary:** thin `nextguide` UI smoke (start / open fixture headless if practical); no full widget matrix in CI.
- Prior art: Format v1 library fixture tests and CLI smokes under `crates/nextguide-gde` and the CLI crate; extend that pattern.
- Required coverage includes: mutation → Structural Round-trip on real fixtures; Preserve-through survival across title/RTF/move; `nextguide-rtf` subset tests; Format v1 gates remain green; optional human open-in-The-Guide after meaningful releases — never CI.

## Out of Scope

- Mobile (Android / iOS) apps.
- Visual clone of The Guide’s Windows UI.
- Full The Guide 2.0 attribute **API** parity beyond Preserve-through.
- Full Word-grade / exotic RTF edit fidelity.
- Web UI / webview / HTML body editors.
- Autosave; in-app auto-updater.
- Linux installers in the first release wave (codebase still targets Linux).
- Full-text search; drag-and-drop reparent; custom themes; print/export besides `.gde`.
- Deep The Guide keyboard-parity; deep accessibility audit beyond keyboard-usable.
- Cloud sync, collaboration, accounts.
- Replacing Format v1 Round-trip gates or adapting `libguide` as the codec.
- Unified single undo history across tree and body (two stacks only in Editor v1).
- Multi-document tabs/MDI in one window.

## Further Notes

- Wayfinder map and resolved decision tickets: `.scratch/editor-v1/`; domain language: `GLOSSARY.md`; ADR-0003 records GTK/Relm4 vs rejected stacks.
- Research inventories: `docs/research/editor-v1-desktop-ui-toolkits.md`, `docs/research/editor-v1-rtf-edit-interop.md`; normative `.gde` bytes still `docs/research/gde-v2-primary-sources.md`.
- Implementation tickets: `.scratch/editor-v1/issues/12`–`21` (`ready-for-agent`). Work blockers-first; frontier is 12, 13, and 14.
