# 02: Undo and Redo on the toolbar

**What to build:** Undo and Redo sit on the toolbar as icon buttons. Each is grey when the focused pane’s stack is empty, and each tooltip names the command and its existing shortcut. Using either button leaves focus where it was, so body undo and tree undo stay on the pane that was already active. A successful undo or redo does not announce itself on the bottom line.

**Blocked by:** 01 File menu and Save on the toolbar

**Status:** claimed

- [ ] Undo and Redo are icon buttons on the toolbar, in that order, after Save
- [ ] Undo is grey when the focused pane’s undo stack is empty; Redo is grey when that pane’s redo stack is empty
- [ ] A greyed button keeps a tooltip with the command name and the existing shortcut
- [ ] Activating Undo or Redo does not take focus, and the stack that was active before the click is the one that runs
- [ ] A successful undo or redo publishes no success sentence
- [ ] Headless tests on the presentation API cover both buttons against each pane’s stacks
- [ ] Undo and Redo are no longer menu items; an Edit menu with nothing left in it is gone
