# 12: Rename Round-trip CLI to `nextguide-cli`

**What to build:** The Format v1 Round-trip tool ships as **`nextguide-cli`** (dump/check unchanged in behaviour), so the **`nextguide`** binary name is free for the Editor v1 app. Workspace, CI, README, and tests refer to the new name.

**Blocked by:** None (can start immediately)

**Status:** ready-for-agent

- [ ] CLI crate/binary is named `nextguide-cli`; dump/check still work with the same exit-code contract
- [ ] Existing CI and docs/commands that invoked `nextguide` for Round-trip use `nextguide-cli`
- [ ] Format v1 Structural/Byte-stable gates still pass
