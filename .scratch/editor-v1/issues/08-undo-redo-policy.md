Type: grilling
Status: resolved

## Question

What **undo/redo** policy should Editor v1 adopt for tree edits and body edits in the first ship?

Decide whether undo is in scope at all, whether tree and RTF share one stack, and what the minimum bar is for “feels like an editor” without boiling the ocean.

## Answer

**Undo and redo are in Editor v1**, with **two stacks** (not one unified document history):

1. **Body:** GTK `TextBuffer` undo/redo while editing that Node’s body; changing selection seals/clears that view’s body stack as usual for per-buffer undo.
2. **Tree:** App-level stack for structure/title ops (insert/remove/move/rename), surviving selection changes.

- Modest depth cap (~50) per stack.
- Edit menu + platform-usual shortcuts; active stack follows focus (body text view vs tree).
