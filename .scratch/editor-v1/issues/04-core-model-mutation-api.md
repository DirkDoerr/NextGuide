Type: grilling
Status: resolved

## Question

What **mutation / edit API** should sit on (or beside) Format v1’s Core-model library so Editor v1 can create, delete, rename, reorder, and reparent Nodes, and update titles and RTF bodies, without breaking Preserve-through or Round-trip honesty?

Format v1 deliberately exposed walk/inspect only. Decide whether Editor v1 extends `nextguide-gde`, adds a sibling crate, or keeps a separate editor model that only saves through the codec — and what the public edit surface looks like.

## Answer

**Extend `nextguide-gde`** with an explicit Core-model mutation API (same crate owns Preserve-through bags; keep them private).

- **Addressing:** public `NodePath` — 0-based child-index sequence from the Dummy root (`[]` = Dummy root).
- **Ops:** `Guide::new_empty()`; `set_title` / `set_rtf_body`; `insert_child(parent, index)` (new Node = empty title, empty RTF, no children, default Preserve-through); `remove_node` (subtree; refuse `[]`); `move_node(from, to_parent, index)` with same-parent index adjustment; index `len` = append.
- **Dummy root:** cannot delete or reparent; title/RTF setters allowed on `[]` for honesty; Editor v1 UI hides the Dummy root.
- **Errors:** extend typed `Error` with a distinct invalid-path/invalid-edit variant (not overloaded `Corrupt`); ops fail closed and apply atomically.
- Preserve-through behaviour under edits is deferred to [Preserve-through under edits](09-preserve-through-under-edits.md).
