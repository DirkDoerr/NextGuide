# 16: Open Guide — two-pane tree + body + save unchanged

**What to build:** From `nextguide`, open a `.gde`, show a resizable two-pane UI (Dummy root hidden; titles in tree; single selection; body via `nextguide-rtf` → GTK tags). Saving without edits preserves Structural Round-trip honesty and untouched RTF bytes for bodies the user did not change.

**Blocked by:** 13 — Core-model mutation API; 14 — `nextguide-rtf` library; 15 — Scaffold `nextguide` Relm4/GTK app

**Status:** ready-for-agent

- [ ] Open loads a Guide into the two-pane UI with Dummy root hidden and correct Node-path mapping
- [ ] Selecting a Node shows its body (soft-fail fallback if RTF unparseable; original bytes retained)
- [ ] Save without edits keeps Format v1 Structural honesty / untouched bodies
- [ ] Split is user-resizable with a sensible default
