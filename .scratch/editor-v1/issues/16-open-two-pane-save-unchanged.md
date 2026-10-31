# 16: Open Guide — two-pane tree + body + save unchanged

**What to build:** From `nextguide`, open a `.gde`, show a resizable two-pane UI (Dummy root hidden; titles in tree; single selection; body via `nextguide-rtf` → GTK tags). Saving without edits preserves Structural Round-trip honesty and untouched RTF bytes for bodies the user did not change.

**Blocked by:** 13 — Core-model mutation API; 14 — `nextguide-rtf` library; 15 — Scaffold `nextguide` Relm4/GTK app

**Status:** resolved

- [x] Open loads a Guide into the two-pane UI with Dummy root hidden and correct Node-path mapping
- [x] Selecting a Node shows its body (soft-fail fallback if RTF unparseable; original bytes retained)
- [x] Save without edits keeps Format v1 Structural honesty / untouched bodies
- [x] Split is user-resizable with a sensible default

## Answer

`EditorDoc` + `visible_tree` / `body_view` helpers (GTK-free) map Dummy-hidden rows to `NodePath` and display bodies via `nextguide-rtf` (soft-fail fallback; Guide RTF untouched). Relm4 UI: resizable `Paned` (~⅓/⅔), TreeView + tagged TextView (read-only), Open/Save, `nextguide path.gde` CLI open. Headless tests cover tree mapping, soft-fail, and save-unchanged Structural Round-trip / RTF byte survival.
