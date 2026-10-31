# 20: Undo/redo (body + tree stacks)

**What to build:** Undo and redo work via **two stacks** — GTK body undo while the text view is focused, and an app-level stack for tree/title ops — with Edit menu and standard shortcuts, focus choosing the active stack, and a modest depth cap (~50).

**Blocked by:** 17 — Edit titles and bodies + dirty/save; 18 — Tree restructure commands

**Status:** resolved

- [x] Body undo/redo works for rich-text edits via GTK buffer undo
- [x] Tree/title ops undo/redo via the app stack across selection changes
- [x] Changing Node selection seals body undo for that view as specified
- [x] Edit menu / shortcuts target the focus-appropriate stack; depth is capped

## Answer

`EditorDoc` keeps an app-level tree/title history (`UNDO_DEPTH_LIMIT` / `TREE_UNDO_LIMIT` = 50) of Guide+dirty checkpoints, pushed on `set_title` / `insert_child` / `remove_node` / `move_node` (`discard_last_push` on failed ops); `set_body` does not push. `undo_tree` / `redo_tree` restore checkpoints. Relm4: Edit menu Undo/Redo + Ctrl/Cmd+Z / Shift+Z / Y; `FocusPane` picks GTK `TextBuffer` undo (same depth) vs tree stack (title edits and tree commands force Tree focus); body fills use `begin_irreversible_action` so selection change seals the body stack; tree undo/redo discards uncommitted body buffer edits before restore. Headless tests cover title/structure undo/redo, body non-push, depth cap, and failed-op non-push.
