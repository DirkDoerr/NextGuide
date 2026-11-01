# 03: Formatting controls on the toolbar

**What to build:** Bold, italic, underline, bullet list, numbered list, font family, and font size are on the toolbar. Character controls and the font dropdowns are grey unless some body text is selected. A character button looks pressed only when the whole selection has that style. A dropdown shows Arial, System Sans, or a point size (9, 10, 11, 12, 14, 18, 24) only when the whole selection shares it, and stays blank when the selection is mixed. Choosing a value still applies to the whole selection. List buttons stay available while a Node body is being edited, including at a bare caret, and look pressed when every line in the selection is that kind of list. A caret counts as its line. Formatting does not announce success.

**Blocked by:** 01 File menu and Save on the toolbar

**Status:** claimed

- [ ] Bold, italic, underline, bullet list, and numbered list are icon buttons; family and size are dropdowns
- [ ] The family dropdown offers Arial and System Sans; the size dropdown offers 9, 10, 11, 12, 14, 18, and 24
- [ ] Bold, italic, underline, and both dropdowns are grey unless some body text is selected, and a greyed control does not look pressed
- [ ] A character button looks pressed only when the whole selection has that style
- [ ] A dropdown shows a value only when the whole selection shares it, and is blank when the selection is mixed
- [ ] A choice from a dropdown applies to the whole selection, as Editor v1 already does
- [ ] List buttons are available while a Node body is being edited, including at a caret, and grey only when no Node body is being edited
- [ ] A list button looks pressed when every line in the selection is that kind of list; a caret counts as its line
- [ ] Icon-button tooltips carry the command name and the existing shortcut
- [ ] Activating a formatting control does not take focus
- [ ] A successful format change publishes no success sentence
- [ ] Headless tests cover an empty selection, a uniform selection, a mixed selection, a caret, and mixed lines
- [ ] These commands are no longer in a Format menu; a Format menu with nothing left in it is gone
