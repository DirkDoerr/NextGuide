# 05: Roomier panes and one toolbar row

**What to build:** The window is one toolbar row under the system title bar, then the tree and the body, then the bottom line. The row’s groups are File, Save, undo/redo, bold/italic/underline, lists, family and size, insert/delete, and move/indent, with gaps between groups. The window is wide enough that the row does not wrap or collapse. Tree rows and the body are roomier than today. The tree selection uses the GTK theme’s selection color across the padded row. The body’s default type stays Arial 9. No leftover Edit, Format, or Tree menu remains.

**Blocked by:** 02 Undo and Redo on the toolbar, 03 Formatting controls on the toolbar, 04 Structure controls on the toolbar

**Status:** ready-for-agent

- [ ] The header of separate Edit, Format, and Tree menus is gone
- [ ] The toolbar is a single row in the agreed group order, with gaps between groups
- [ ] The window’s minimum width fits that row, with no wrap and no overflow menu
- [ ] Icons are the standard Adwaita symbolic icons, with no custom icon set
- [ ] Tree rows and body padding are roomier than today, via spacing and padding only
- [ ] The tree selection uses the theme’s selection color, drawn across the padded row
- [ ] The body’s default type stays Arial 9
- [ ] The tree stays on the left and the body on the right, still resizable, in the same widgets
- [ ] A manual check of the running window confirms icon choice, group order, spacing, title-bar placement, and that a toolbar click does not take focus
- [ ] The window does not gain libadwaita, a color palette, or GTK tests in CI
