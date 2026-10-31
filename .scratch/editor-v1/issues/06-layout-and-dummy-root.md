Type: grilling
Status: resolved

## Question

What **primary layout** should Editor v1 use to deliver The Guide’s functionality (not its Windows chrome), and how exactly should the **Dummy root** be hidden in the UI while remaining correct on disk via Format v1?

## Answer

**Two-pane** desktop layout: resizable tree (left, ~⅓ default) + body rich-text (right, ~⅔); menubar/toolbar for commands — functionality of The Guide, not a Windows UI clone.

- **Titles** edited in the tree; body pane is RTF only.
- **Dummy root** never shown or selectable; tree top level = its children; UI maps rows to library **Node paths** that still include the Dummy root (first row ↔ `[0]`).
- **Single** Node selection for body editing; multi-select out of Editor v1.
- **Empty Guide:** empty tree + empty/disabled body until a Node is added.
- Drag-drop vs menu reparent left in map fog.
