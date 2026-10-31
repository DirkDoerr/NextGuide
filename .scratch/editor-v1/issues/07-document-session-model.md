Type: grilling
Status: resolved

## Question

What **document session model** should Editor v1 use for open / save / save-as / new Guide, dirty state, and “one Guide vs many”?

Standing charting prefs already require those workflows; this ticket locks the UX/session rules (single window vs tabs, prompts on close, path handling) so the spec can be precise.

## Answer

**Single Guide per window** (no tabs/MDI). Opening another Guide replaces the current after dirty prompts; users can run multiple OS windows if needed.

- **Dirty** on any Core-model/body edit; **Save** / **Save As**; **New** / **Open** / quit → Save / Discard / Cancel when dirty.
- **Untitled** until Save As; window title shows name + dirty indicator.
- **Explicit save only** — no autosave in Editor v1.
- **`nextguide [path.gde]`** opens that file; bare launch → untitled New Guide.
- **Recent files** menu: last **5** paths.
