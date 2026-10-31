# 21: Win/macOS release bundles (+ `nextguide-cli`)

**What to build:** First-wave **Windows and macOS** release artifacts that ship `nextguide` with a **bundled GTK runtime** and `nextguide-cli` in the same release; manual updates; signing optional (unsigned OK). Linux installers remain deferred.

**Blocked by:** 19 — Document session; 20 — Undo/redo

**Status:** ready-for-agent

- [ ] Documented/automated path to produce Windows installer (MSI or NSIS) and macOS `.app` in a DMG including GTK
- [ ] `nextguide-cli` is included in the same release artifacts
- [ ] README (or release notes) describe manual updates and optional signing
- [ ] Linux packaging explicitly left for a later wave
