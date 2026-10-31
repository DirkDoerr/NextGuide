# 19: Document session (New / Open / Save As / prompts / recent / CLI path)

**What to build:** One Guide per window with New, Open, Save As, dirty Save/Discard/Cancel prompts on replace/quit, untitled until Save As, recent files (5), and `nextguide [path.gde]` / bare launch behaviour per the Editor v1 spec.

**Blocked by:** 17 — Edit titles and bodies + dirty/save

**Status:** ready-for-agent

- [ ] New / Open / Save As / quit honour dirty prompts
- [ ] Window title shows Untitled or filename + dirty mark
- [ ] Recent files lists up to 5 paths and opens them
- [ ] `nextguide path.gde` opens that Guide; bare `nextguide` starts untitled New
