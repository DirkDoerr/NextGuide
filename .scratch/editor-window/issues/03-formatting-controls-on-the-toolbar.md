# 03: Formatting controls on the toolbar

**What to build:** Bold, italic, underline, bullet list, numbered list, font family, and font size are on the toolbar. Character controls and the font dropdowns are grey unless some body text is selected. A character button looks pressed only when the whole selection has that style. A dropdown shows Arial, System Sans, or a point size (9, 10, 11, 12, 14, 18, 24) only when the whole selection shares it, and stays blank when the selection is mixed. Choosing a value still applies to the whole selection. List buttons stay available while a Node body is being edited, including at a bare caret, and look pressed when every line in the selection is that kind of list. A caret counts as its line. Formatting does not announce success.

**Blocked by:** 01 File menu and Save on the toolbar

**Status:** resolved

- [x] Bold, italic, underline, bullet list, and numbered list are icon buttons; family and size are dropdowns
- [x] The family dropdown offers Arial and System Sans; the size dropdown offers 9, 10, 11, 12, 14, 18, and 24
- [x] Bold, italic, underline, and both dropdowns are grey unless some body text is selected, and a greyed control does not look pressed
- [x] A character button looks pressed only when the whole selection has that style
- [x] A dropdown shows a value only when the whole selection shares it, and is blank when the selection is mixed
- [x] A choice from a dropdown applies to the whole selection, as Editor v1 already does
- [x] List buttons are available while a Node body is being edited, including at a caret, and grey only when no Node body is being edited
- [x] A list button looks pressed when every line in the selection is that kind of list; a caret counts as its line
- [x] Icon-button tooltips carry the command name and the existing shortcut
- [x] Activating a formatting control does not take focus
- [x] A successful format change publishes no success sentence
- [x] Headless tests cover an empty selection, a uniform selection, a mixed selection, a caret, and mixed lines
- [x] These commands are no longer in a Format menu; a Format menu with nothing left in it is gone

## Answer

Bold, italic, underline, lists, and the font dropdowns sit on the toolbar and follow the presentation API (`47a5363`). Character controls and the dropdowns stay grey unless some body text is selected, and a grey control does not look pressed. A character button looks pressed only when the whole selection has that style. A dropdown shows Arial, System Sans, or a point size (9, 10, 11, 12, 14, 18, 24) only when the whole selection shares it, stays blank when the selection is mixed, and still applies a choice to the whole selection. List buttons stay available while a Node body is being edited, including at a caret, and look pressed when every line in the selection is that kind of list. Tooltips carry the command name and shortcut, and activating a control does not take focus. A successful format change publishes no success sentence. The Format menu is gone. Headless tests cover an empty selection, a uniform selection, a mixed selection, a caret, and mixed lines.
