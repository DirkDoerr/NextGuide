# 18: Tree restructure commands

**What to build:** Users can add, delete, reorder, and reparent Nodes through **menu/commands** (no drag-and-drop), wired to `nextguide-gde` mutations, with the tree UI staying consistent and Preserve-through rules respected (bags move with Nodes; dropped on delete).

**Blocked by:** 16 — Open Guide — two-pane tree + body + save unchanged

**Status:** ready-for-agent

- [ ] Insert / remove / move (reorder/reparent) available via commands; Dummy root never deletable
- [ ] Tree refreshes to match Core-model state after each op
- [ ] Save after restructure Structural-Round-trips; Preserve-through behaviour matches the spec
