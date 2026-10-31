# 18: Tree restructure commands

**What to build:** Users can add, delete, reorder, and reparent Nodes through **menu/commands** (no drag-and-drop), wired to `nextguide-gde` mutations, with the tree UI staying consistent and Preserve-through rules respected (bags move with Nodes; dropped on delete).

**Blocked by:** 16 — Open Guide — two-pane tree + body + save unchanged

**Status:** resolved

- [x] Insert / remove / move (reorder/reparent) available via commands; Dummy root never deletable
- [x] Tree refreshes to match Core-model state after each op
- [x] Save after restructure Structural-Round-trips; Preserve-through behaviour matches the spec

## Answer

Extended `EditorDoc` with `insert_child` (minimal `empty_rtf` + rollback on RTF set failure), `remove_node`, and `move_node` (dirty + Core-model ops). Relm4 Tree popover: Insert Child/Sibling, Delete, Move Up/Down, Indent/Outdent; tree refill from `visible_tree` after each op; Dummy root not selectable/deletable. Headless tests cover dirty/tree updates, Structural Round-trip after restructure, Preserve-through survival on move, and deleted-subtree absence after save.
