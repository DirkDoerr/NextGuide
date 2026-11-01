# 04: Structure controls on the toolbar

**What to build:** Insert Child, Insert Sibling, Delete, Move Up, Move Down, Indent, and Outdent are on the toolbar. Insert Child and Insert Sibling show the standard add symbol plus those words. Insert Child is available whenever a Guide is open, including when nothing is selected, and then adds a top-level Node. The other structure buttons are grey when no Node is selected, and stay clickable when a Node is selected but the move is illegal. An illegal move still shows the existing explanation on the bottom line. A successful structure edit does not announce itself.

**Blocked by:** 01 File menu and Save on the toolbar

**Status:** resolved

- [x] Insert Child and Insert Sibling show the standard add symbol and the words Insert Child and Insert Sibling
- [x] Delete, Move Up, Move Down, Indent, and Outdent are icon buttons
- [x] Insert Child is available for an open Guide with nothing selected, and adds a top-level Node under the hidden Dummy root
- [x] Insert Sibling, Delete, Move Up, Move Down, Indent, and Outdent are grey when no Node is selected
- [x] Those buttons stay clickable when a Node is selected but the move is illegal, and the existing refusal sentence appears on the bottom line
- [x] A greyed button keeps a tooltip with the command name and no invented shortcut
- [x] Activating a structure control does not take focus
- [x] A successful insert, delete, or move publishes no success sentence
- [x] Headless tests cover Insert Child with no selection, structure controls with and without a selected Node, and an illegal move still returning its refusal sentence
- [x] These commands are no longer in a Tree menu; a Tree menu with nothing left in it is gone
- [x] The Dummy root stays hidden

## Answer

Insert Child, Insert Sibling, Delete, Move Up, Move Down, Indent, and Outdent sit on the toolbar (`3d645ab`). Insert Child and Insert Sibling use the standard add symbol plus those words; the other five are icon buttons. Insert Child stays available for an open Guide with nothing selected and adds a top-level Node under the hidden Dummy root. The other structure buttons are grey when no Node is selected, keep a tooltip with the command name and no shortcut, and stay clickable when a Node is selected but the move is illegal so the existing refusal sentence still appears on the bottom line. Activating a structure control does not take focus. A successful insert, delete, or move publishes no success sentence. The Tree menu is gone. Headless tests cover Insert Child with no selection, structure controls with and without a selected Node, and an illegal move still returning its refusal sentence.
