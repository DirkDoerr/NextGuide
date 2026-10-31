# 13: Core-model mutation API in `nextguide-gde`

**What to build:** Callers can mutate a loaded Guide’s Core model via public ops addressed by **Node path** (`new_empty`, set title/RTF, insert/remove/move children), with fail-closed typed edit errors, atomic ops, Dummy root protected, and Preserve-through bags kept private but behaving correctly under edits. Library tests prove Structural Round-trip after mutations on real fixtures.

**Blocked by:** None (can start immediately)

**Status:** ready-for-agent

- [ ] Public `NodePath` + mutation ops per Editor v1 spec; Dummy root cannot be deleted/reparented
- [ ] Invalid path/edit errors are distinct and ops never leave a partial Guide
- [ ] Preserve-through survives title/RTF/move; dropped on delete; empty on insert (observable via load/save, not bag exposure)
- [ ] Structural Round-trip still passes for fixtures after representative mutations; Format v1 gates remain green
