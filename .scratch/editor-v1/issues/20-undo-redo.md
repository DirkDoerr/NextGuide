# 20: Undo/redo (body + tree stacks)

**What to build:** Undo and redo work via **two stacks** — GTK body undo while the text view is focused, and an app-level stack for tree/title ops — with Edit menu and standard shortcuts, focus choosing the active stack, and a modest depth cap (~50).

**Blocked by:** 17 — Edit titles and bodies + dirty/save; 18 — Tree restructure commands

**Status:** ready-for-agent

- [ ] Body undo/redo works for rich-text edits via GTK buffer undo
- [ ] Tree/title ops undo/redo via the app stack across selection changes
- [ ] Changing Node selection seals body undo for that view as specified
- [ ] Edit menu / shortcuts target the focus-appropriate stack; depth is capped
