Type: grilling
Status: resolved
Blocked by: 04

## Question

When the user edits Core-model fields (title, RTF body, tree topology), what must happen to **Preserve-through** non-Core / unknown attributes so The Guide does not lose them on Round-trip?

Pin the rules for attrs on edited vs untouched Nodes, deleted Nodes, and newly created Nodes.

## Answer

Preserve-through bags stay **private** in `nextguide-gde` and follow the Node (or Guide), not the edit kind:

- **Title / RTF body edits** (including subset RTF rewrite): keep that Node’s non-Core / unknown attrs unchanged.
- **move_node / reparent:** bags move with the Node and its subtree.
- **remove_node:** bags for that Node and descendants are dropped.
- **insert_child:** new Node gets an empty/default bag (no invented non-Core attrs).
- **Guide/file-level** Preserve-through survives Core tree/body edits and is written on save.

UI never exposes these attrs in Editor v1.
