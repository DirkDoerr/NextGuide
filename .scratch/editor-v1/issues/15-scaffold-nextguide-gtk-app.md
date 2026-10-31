# 15: Scaffold `nextguide` Relm4/GTK app

**What to build:** A Relm4/GTK desktop binary named **`nextguide`** that launches a window and depends on `nextguide-gde` in-process (stub UI is enough), so later tickets have an app crate to land in. Thin smoke if headless GTK is practical.

**Blocked by:** 12 — Rename Round-trip CLI to `nextguide-cli`

**Status:** ready-for-agent

- [ ] Workspace builds binary `nextguide` (GTK4 + Relm4) linking `nextguide-gde`
- [ ] App starts a window on a developer machine with GTK available
- [ ] Thin smoke test exists or is explicitly deferred with a documented manual check if CI cannot run GTK yet
