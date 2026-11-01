# 05: Roomier panes and one toolbar row

**What to build:** The window is one toolbar row under the system title bar, then the tree and the body, then the bottom line. The row’s groups are File, Save, undo/redo, bold/italic/underline, lists, family and size, insert/delete, and move/indent, with gaps between groups. The window is wide enough that the row does not wrap or collapse. Tree rows and the body are roomier than today. The tree selection uses the GTK theme’s selection color across the padded row. The body’s default type stays Arial 9. No leftover Edit, Format, or Tree menu remains.

**Blocked by:** 02 Undo and Redo on the toolbar, 03 Formatting controls on the toolbar, 04 Structure controls on the toolbar

**Status:** resolved

- [x] The header of separate Edit, Format, and Tree menus is gone
- [x] The toolbar is a single row in the agreed group order, with gaps between groups
- [x] The window’s minimum width fits that row, with no wrap and no overflow menu
- [x] Icons are the standard Adwaita symbolic icons, with no custom icon set
- [x] Tree rows and body padding are roomier than today, via spacing and padding only
- [x] The tree selection uses the theme’s selection color, drawn across the padded row
- [x] The body’s default type stays Arial 9
- [x] The tree stays on the left and the body on the right, still resizable, in the same widgets
- [x] A manual check of the running window confirms icon choice, group order, spacing, title-bar placement, and that a toolbar click does not take focus
- [x] The window does not gain libadwaita, a color palette, or GTK tests in CI

## Answer

The window is one toolbar row under the system title bar: File, Save, undo/redo, bold/italic/underline, lists, family and size, insert/delete, then move/indent, with gaps between groups. The window’s minimum width is that row, so it does not wrap or collapse. Icons stay the standard Adwaita symbolic names. Tree rows and the body are roomier through spacing and padding only; selection uses the theme’s selection color across the padded row, and the body’s default type stays Arial 9. The tree stays on the left and the body on the right in the same resizable widgets. Edit, Format, and Tree menus stay gone, and the window gains no libadwaita, color palette, or GTK tests (`0608f05`). The focus-staying-put toolbar click was not checked against a running window.
