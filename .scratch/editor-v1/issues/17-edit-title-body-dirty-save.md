# 17: Edit titles and bodies + dirty/save

**What to build:** Users can edit the selected Node’s title in the tree and its body with the pragmatic rich-text subset; the Guide becomes dirty; Save writes Core-model mutations (subset RTF rewrite only for touched bodies; Preserve-through retained on those Nodes).

**Blocked by:** 16 — Open Guide — two-pane tree + body + save unchanged

**Status:** resolved

- [x] Title and body edits update the in-memory Guide via the mutation API / `nextguide-rtf`
- [x] Dirty indicator appears; Save clears dirty and persists `.gde`
- [x] Untouched sibling bodies keep original RTF bytes; edited bodies emit subset RTF
- [x] Library/UI path does not regress Structural Round-trip for the edited Guide’s Core model

## Answer

Extended GTK-free `EditorDoc` with `set_title` / `set_body` (mutation API + `nextguide-rtf` subset rewrite), `is_dirty`, and `save`/`save_to` that clear dirty. Window titles take a dirty flag (`name *`). Relm4 UI: editable tree titles, editable body TextView (buffer ↔ Document; list structure via `list-*` tags; Ctrl/Cmd+B/I/U), commit body on selection change/Save, dirty chrome. Headless tests cover dirty, sibling untouched RTF, Preserve-through after edits, and Structural Round-trip. Dirty prompts on Open/quit deferred to ticket 19.
