Status: done

# Editor window — toolbar and pane presentation

## Problem Statement

Editor v1 already opens, edits, and saves a Guide, but the window is hard to use. Everyday commands sit in text menus (File, Edit, Format, Tree). There is no icon toolbar. The menu strip is a second bar under the operating-system title bar. The tree and the body are cramped. After a click, the bottom line announces success as well as failure, so it is unclear when something actually went wrong.

## Solution

Give `nextguide` one finished window on Windows, macOS, and Linux. The operating system keeps the title bar (file name and dirty mark). Directly under it, one toolbar row shows the commands a Guide owner uses while writing and restructuring, with standard symbolic icons. File commands that are not Save stay in a File menu on that row. The tree and the body stay where they are, with roomier padding. The bottom line speaks only when an action was refused or failed. An empty Guide explains itself in the body area.

Editing behavior stays the closed Editor v1 contract. This slice changes presentation.

## User Stories

1. As a Guide owner, I want the operating system’s title bar to show the Guide’s file name, so that I can see which Guide I have open.
2. As a Guide owner, I want an untitled Guide’s title bar to read Untitled, so that a new Guide is obvious.
3. As a Guide owner, I want a dirty Guide’s title bar to keep the existing dirty mark, so that I can see unsaved work without a success message.
4. As a Guide owner, I want the toolbar to sit directly under that title bar, so that the tools are not a second title.
5. As a Guide owner, I want the same window arrangement on Windows, macOS, and Linux, so that the app has one look.
6. As a Guide owner, I want every toolbar command on a single row, so that I do not hunt through menus while writing.
7. As a Guide owner, I want the window wide enough that the toolbar neither wraps nor collapses into an overflow menu, so that every command stays visible.
8. As a Guide owner, I want gaps between command groups, so that file, history, formatting, type, and structure read as separate clusters.
9. As a Guide owner, I want the groups in this order: File, Save, undo/redo, bold/italic/underline, lists, font family and size, insert/delete, then move/indent, so that the row has a stable order.
10. As a Guide owner, I want the File control to be the word File, so that the menu is recognizable.
11. As a Guide owner, I want File to contain New, Open, Save As, Recent, and Quit, so that session commands stay together.
12. As a Guide owner, I want Save on the toolbar and not also in File, so that saving has one visible place.
13. As a Guide owner, I want New, Open, Save As, Recent, and Quit to keep their current behavior, so that dirty prompts, recent Guides, and launch rules do not change.
14. As a Guide owner, I want Save, Undo, Redo, Bold, Italic, Underline, bullet list, numbered list, Delete, Move Up, Move Down, Indent, and Outdent to be icon buttons, so that I can recognize them without opening a menu.
15. As a Guide owner, I want those icons to be the standard symbolic icons from the Adwaita theme the installers already ship, so that the toolbar matches the GTK runtime.
16. As a Guide owner, I want each icon button’s tooltip to be the command name, so that an unfamiliar symbol is still legible.
17. As a Guide owner, I want a tooltip to include the shortcut when that command already has one, so that the toolbar teaches the shortcuts Editor v1 already has.
18. As a Guide owner, I want tree commands’ tooltips to be the command name alone, so that the tooltip does not invent a shortcut.
19. As a Guide owner, I want Insert Child and Insert Sibling to show the standard add symbol plus the words Insert Child and Insert Sibling, so that those two actions are distinguishable.
20. As a Guide owner, I want font family and font size to be dropdowns on the toolbar, so that I can see and change type without a menu.
21. As a Guide owner, I want the family dropdown to offer Arial and System Sans, so that the closed font set stays the set I can pick.
22. As a Guide owner, I want the size dropdown to offer 9, 10, 11, 12, 14, 18, and 24, so that the closed size set stays the set I can pick.
23. As a Guide owner, I want a toolbar click to leave focus where it was, so that the text selection and the active undo stack do not change just because I used a tool.
24. As a Guide owner, I want Undo and Redo to keep using the pane that was focused before the click, so that body undo and tree undo stay separate.
25. As a Guide owner, I want Save greyed when the Guide is clean, so that I can see there is nothing to write.
26. As a Guide owner, I want Save available when the Guide is dirty, including an untitled Guide, so that the existing save path still runs.
27. As a Guide owner, I want Undo greyed when the focused pane’s undo stack is empty, so that an empty stack is visible before I click.
28. As a Guide owner, I want Redo greyed when the focused pane’s redo stack is empty, so that an empty stack is visible before I click.
29. As a Guide owner, I want Bold, Italic, Underline, and both font dropdowns greyed unless some body text is selected, so that selection-only formatting stays honest.
30. As a Guide owner, I want the list buttons available while a Node body is being edited, including when the caret has no selection, so that a list toggle can still apply to the caret’s line.
31. As a Guide owner, I want the list buttons greyed when no Node body is being edited, so that I cannot format a body I am not in.
32. As a Guide owner, I want Insert Child available whenever a Guide is open, including an untitled Guide with nothing selected, so that I can add a top-level Node and leave the empty Guide.
33. As a Guide owner, I want Insert Sibling, Delete, Move Up, Move Down, Indent, and Outdent greyed when no Node is selected, so that structure commands announce that they have no target.
34. As a Guide owner, I want those structure buttons to stay clickable when a Node is selected but the move itself is illegal, so that the existing explanation still appears.
35. As a Guide owner, I want a greyed button to keep its tooltip, so that I can still read the command.
36. As a Guide owner, I want Bold, Italic, or Underline to look pressed only when the whole selection has that style, so that the toolbar tells the truth about mixed text.
37. As a Guide owner, I want a list button to look pressed when every line in the selection is that kind of list, so that the toolbar matches the lines I am in.
38. As a Guide owner, I want a caret with no selection to count as its line for that list state, so that the button matches the line I am typing on.
39. As a Guide owner, I want a font dropdown to show the family or size only when the whole selection shares it, so that a mixed selection is not labeled as one value.
40. As a Guide owner, I want a mixed family or size to leave that dropdown blank until I choose, so that I see that the selection disagrees.
41. As a Guide owner, I want that choice to apply to the whole selection, so that formatting behavior stays what Editor v1 already does.
42. As a Guide owner, I want a greyed control not to look pressed, so that an unavailable command does not look active.
43. As a Guide owner, I want the bottom line to show the existing sentence when an action is refused or fails, so that I learn why a move, open, or save did not happen.
44. As a Guide owner, I want a successful save, open, rename, format, insert, undo, or redo to leave the bottom line unchanged by a success sentence, so that the line means something went wrong.
45. As a Guide owner, I want “Untitled Guide — add a Node to begin.” in the body area when the Guide has no Nodes, so that the empty Guide tells me what to do.
46. As a Guide owner, I want any other empty body to stay blank, so that an empty Node body is a writing surface and a Guide with Nodes but no selection stays quiet.
47. As a Guide owner, I want the tree on the left and the body on the right, still resizable, so that the Editor v1 layout remains.
48. As a Guide owner, I want the tree and the body to keep their current widgets, so that title editing and body editing do not get a new control.
49. As a Guide owner, I want roomier tree rows and body padding than today, so that the panes feel less cramped.
50. As a Guide owner, I want the tree selection to use the GTK theme’s selection color, drawn across the padded row, so that selection is obvious without a new palette.
51. As a Guide owner, I want the body’s default type to stay Arial 9, so that unstyled text still matches the closed font default.
52. As a Guide owner, I want the Dummy root never to be shown, so that Insert Child with nothing selected adds a top-level Node I can see.
53. As a NextGuide developer, I want GTK4 and Relm4 to remain the UI stack, so that ADR-0003 stays in force.
54. As a NextGuide developer, I want no libadwaita dependency, so that the window does not become a GNOME application.
55. As a NextGuide developer, I want one small stylesheet for padding and spacing, so that the look does not introduce a color palette.
56. As a NextGuide developer, I want toolbar presentation decided by a GTK-free function on `nextguide`, so that CI can test enablement, pressed state, dropdown values, status disposition, and the empty hint without a display.
57. As a NextGuide developer, I want the Relm4 window only to render that result and to report the snapshot the function needs, so that widget code does not reimplement the rules.
58. As a CI job, I want those presentation tests to run with the existing headless `nextguide` tests, so that the window slice does not require GTK in CI.
59. As a human tester, I want a manual look at the running window for icon choice, group order, spacing, title-bar placement, and focus staying put, so that shell details the headless seam cannot see still get a check.
60. As a Guide owner, I want existing shortcuts to keep working, so that this slice does not add or remove key bindings.

## Implementation Decisions

- Deliver the slice in the existing Editor v1 app: the GTK-free `nextguide` library gains a presentation API, and the Relm4 shell renders it. No new crate. `nextguide-gde` and `nextguide-rtf` stay the editing and font authorities; this slice does not change their contracts.
- The presentation API is pure and contains no GTK types. Callers pass a snapshot and an optional action outcome. The snapshot carries only facts: whether a Guide is open, whether it is dirty, whether it has any Nodes, which pane is focused, whether that pane’s undo and redo stacks are empty, whether a Node body is being edited, whether a Node is selected, whether any body text is selected, whether the whole selection shares bold, italic, and underline, the list kind of each line in the selection (a caret is its line), and the family and size when the whole selection shares them. The shell reads those facts out of the widgets and `EditorDoc`. The API does not inspect widgets.
- The API returns, for each toolbar command, whether it is enabled, whether it is shown pressed, and the dropdown’s displayed value (a shared family, a shared size, or blank). A control that is not enabled is not pressed. It returns whether the empty hint is shown. Given an action outcome, it returns the bottom-line sentence only for a refusal or a failure, and returns no sentence for a success. Refusal and failure sentences stay the ones Editor v1 already uses. Successes that must produce no sentence include save, open, rename, format toggle, font change, list toggle, insert, delete, move, undo, and redo, plus dismissing a file dialog.
- The shell lays the window out as: operating-system title bar (existing title string, including the dirty mark), one toolbar row, the existing resizable tree|body split, then the bottom line. The toolbar’s minimum content width is the window’s minimum width, so the row does not wrap or overflow. Groups are separated by gaps, in the order in the user stories. File is a menu button labeled File. Save is only on the toolbar. Icon buttons use Adwaita symbolic names already bundled with the GTK runtime. Insert Child and Insert Sibling use the standard add symbol plus their words. Family and size are dropdowns bound to the API’s displayed value and to the existing Arial / System Sans and 9–24 point sets.
- Toolbar controls do not take focus. Activating one applies the existing command to the selection already in place. Undo and Redo consult the pane focused before the click.
- The shell drops the header strip of File / Edit / Format / Tree popovers. Edit, Format, and Tree are not menus. Their commands live on the toolbar.
- Tree and body widgets stay the current tree view and text view. A stylesheet sets padding and spacing only, including roomier tree rows and body padding. Selection color is the theme’s selection, painted across the padded tree row. The body default remains Arial 9. The empty hint is that stylesheet-free string in the body area, and only when the API says the Guide has no Nodes. The body stays non-editable in that state, as it does today.
- Shortcuts are unchanged. Tooltips are the command name, and the existing shortcut when the command has one. Tree commands have no shortcut in the tooltip.
- Grey versus clickable follows the snapshot, not a second copy of the edit rules in the widget callbacks. In particular, Insert Child stays enabled for an open Guide with no selection (the child is a top-level Node under the Dummy root, which is never shown). Structure commands stay enabled when a Node is selected even if the eventual move will be refused. Those refusals still surface through the presentation API as status sentences.
- Respect ADR-0003: GTK4 + Relm4, dynamically linked, no web UI. Do not add libadwaita. Do not add a custom icon set or a color palette.

## Testing Decisions

- Good tests assert external behavior of the presentation API only: enabled or grey, pressed or not, dropdown value or blank, empty hint shown or not, and status sentence present only for a refusal or failure. They do not assert GTK widget types, icon names, pixel padding, or stylesheet text.
- **One seam:** the GTK-free presentation API on `nextguide`. Build snapshots and action outcomes in tests and read the result. Do not open a display.
- The shell is a renderer of that API plus the focus rule. Icon choice, group order, spacing, title-bar placement, and “a toolbar click does not take focus” are a manual check of the running window. That manual check is not a second automated seam, and CI does not gain GTK tests.
- Prior art: the headless `nextguide` session tests (dirty titles, tree undo, recent files, launch path) and the Editor v1 rule that library tests prove behavior while the widget tree stays thin. Format v1 Structural and Byte-stable gates stay green and are not the seam for this slice.
- Required coverage includes: Save against dirty and clean; undo and redo against the focused pane’s stacks; character controls and font dropdowns against an empty selection, a uniform selection, and a mixed selection; list buttons against a caret and against mixed lines; Insert Child with a Guide and no selection; structure controls with and without a selected Node; empty hint only when the Guide has no Nodes; success outcomes producing no status sentence; an existing refusal sentence still returned for an illegal structure action.

## Out of Scope

- Any change to Editor v1 editing behavior: selection-only bold, italic, underline, and font; list toggles on the caret line; Round-trip; Preserve-through; untouched RTF bytes; dirty prompts; one Guide per window; the two undo stacks; explicit save.
- Caret typing-style (arm a style, then type).
- New shortcuts, or a platform menu bar as a second command surface.
- Per-node Guide icons, full-text search, drag-and-drop reparent, print, custom themes, and a color palette.
- libadwaita, a custom icon set, replacing the tree or body widget, and a toolbar that wraps or overflows.
- Confirmation dialogs for delete or other commands.
- GTK tests in CI, a tagged installer, Windows `.gde` file association, and Linux installers.
- A human open-in-The-Guide pass as a gate for this slice.

## Further Notes

- Behavior contract remains the Editor v1 spec at `.scratch/editor-v1/spec.md`. This spec is presentation on top of that contract. Domain language is `GLOSSARY.md`. Stack constraint is ADR-0003.
- The presentation API is the seam confirmed for this spec. Implement the library API and the shell together; the manual window check follows that, and is not a CI job.
