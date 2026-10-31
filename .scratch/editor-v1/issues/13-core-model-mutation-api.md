# 13: Core-model mutation API in `nextguide-gde`

**What to build:** Callers can mutate a loaded Guide’s Core model via public ops addressed by **Node path** (`new_empty`, set title/RTF, insert/remove/move children), with fail-closed typed edit errors, atomic ops, Dummy root protected, and Preserve-through bags kept private but behaving correctly under edits. Library tests prove Structural Round-trip after mutations on real fixtures.

**Blocked by:** None (can start immediately)

**Status:** resolved

- [x] Public `NodePath` + mutation ops per Editor v1 spec; Dummy root cannot be deleted/reparented
- [x] Invalid path/edit errors are distinct and ops never leave a partial Guide
- [x] Preserve-through survives title/RTF/move; dropped on delete; empty on insert (observable via load/save, not bag exposure)
- [x] Structural Round-trip still passes for fixtures after representative mutations; Format v1 gates remain green

## Answer

Extended `nextguide-gde` with public `NodePath` and Guide mutation ops: `new_empty`, `node`, `set_title`, `set_rtf_body`, `insert_child`, `remove_node`, `move_node` (same-parent index adjustment; clone-swap for atomicity). Dummy root refuses delete/reparent. Errors: `InvalidPath` / `InvalidEdit` (distinct from `Corrupt`). Preserve-through bags stay private; survive title/move; drop with deleted subtree; empty on insert. Library tests cover ops + Structural Round-trip on ImportSample.
