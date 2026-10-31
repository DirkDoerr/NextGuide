# 19: Document session (New / Open / Save As / prompts / recent / CLI path)

**What to build:** One Guide per window with New, Open, Save As, dirty Save/Discard/Cancel prompts on replace/quit, untitled until Save As, recent files (5), and `nextguide [path.gde]` / bare launch behaviour per the Editor v1 spec.

**Blocked by:** 17 — Edit titles and bodies + dirty/save

**Status:** resolved

- [x] New / Open / Save As / quit honour dirty prompts
- [x] Window title shows Untitled or filename + dirty mark
- [x] Recent files lists up to 5 paths and opens them
- [x] `nextguide path.gde` opens that Guide; bare `nextguide` starts untitled New

## Answer

`EditorDoc::new` / `save_as`; `DirtyPromptChoice` + `needs_dirty_prompt`; `RecentFiles` (MRU ≤5, `~/.config/nextguide/recent`); `launch_path_from_args`. Relm4: New/Open/Save/Save As/Recent, dirty Save/Discard/Cancel on replace/quit, bare launch = untitled New.
