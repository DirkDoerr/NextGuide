# 01: File menu and Save on the toolbar

**What to build:** The Guide’s file name and dirty mark stay in the operating-system title bar. Directly under it, a toolbar starts with the word File and a Save icon. File contains New, Open, Save As, Recent, and Quit, and those commands behave as they do today. Save is grey when the Guide is clean and available when it is dirty, including an untitled Guide. A Guide with no Nodes shows “Untitled Guide — add a Node to begin.” in the body area. A successful save does not announce itself on the bottom line. Edit, Format, and Tree stay reachable until their own tickets move them.

**Blocked by:** None (can start immediately).

**Status:** resolved

- [x] The title bar shows the file name, or Untitled, and keeps the existing dirty mark
- [x] The toolbar sits under the title bar and does not replace it
- [x] File offers New, Open, Save As, Recent, and Quit, with the existing dirty prompts, recent-Guide list, and shortcuts
- [x] Save is an icon on the toolbar and is not also an item in File
- [x] Save is grey when the Guide is clean, and available when it is dirty, including an untitled Guide
- [x] A greyed Save keeps a tooltip with the command name and the existing shortcut
- [x] “Untitled Guide — add a Node to begin.” appears in the body only when the Guide has no Nodes, and that body stays non-editable
- [x] Any other empty body stays blank
- [x] A successful save publishes no success sentence; a failed save still shows the existing failure sentence
- [x] Headless tests on the GTK-free presentation API cover Save’s enabled state and the empty hint
- [x] Edit, Format, and Tree commands remain reachable
- [x] The Dummy root stays hidden

## Answer

File and Save sit on a toolbar under the title bar (`0c4a566`). The title bar still shows the file name (or Untitled) and the dirty mark. File offers New, Open, Save As, Recent, and Quit; Save is a toolbar icon, grey when the Guide is clean and available when it is dirty, including an untitled Guide. A Guide with no Nodes shows “Untitled Guide — add a Node to begin.” in a non-editable body; any other empty body stays blank. A successful save publishes nothing; a failed save still shows the failure sentence. Edit, Format, and Tree stay as menus. Headless tests on the GTK-free presentation API cover Save’s enabled state and the empty hint.
