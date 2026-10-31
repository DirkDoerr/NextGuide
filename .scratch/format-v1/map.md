# Format v1 wayfinder map

Type: wayfinder:map

## Destination

A buildable product spec for **Format v1**: a greenfield library plus CLI that Round-trips The Guide **v2** `.gde` files for the **Core model** (tree, titles, opaque RTF bodies), with fixture-based proof. No editor UI.

## Notes

- Domain language: `GLOSSARY.md`
- Tracker: local markdown under `.scratch/format-v1/` (see `docs/agents/issue-tracker.md`)
- Codec posture: **greenfield** from primary sources — do not ship Format v1 by adapting `libguide` / forks as the implementation
- Implementation language: **Rust**, minimal-dependencies rule (see Decisions so far + ADR-0002)
- Skills: `grilling` + `domain-modeling` for grilling tickets; `research` for research tickets; `prototype` only if a ticket is labelled that way

## Decisions so far

- [Inventory primary sources for The Guide v2 .gde format](issues/04-primary-sources-inventory.md): No separate format spec — v2 `.gde` is defined by Apache-2.0 The Guide 2.0 `libguide` (`guide.c`/`guide.h`); inventory at `docs/research/gde-v2-primary-sources.md`
- [Choose Format v1 implementation language](issues/01-implementation-language.md): Rust with a hard minimal-dependencies rule (desktop binaries now; same codec linkable later for optional mobile)

## Not yet specified

- Whether third-party format descriptions (e.g. Kaitai) may be used strictly as *reference* while the codec stays greenfield
- Error model for corrupt, truncated, or non-v2 `.gde` inputs
- How Format v1 later feeds a desktop/mobile editor (beyond this destination)

## Out of scope

- Editor UI and desktop / mobile apps (README platforms beyond the format layer)
- The Guide **v1** `.gde` generation (read or write)
- Full The Guide 2.0 attribute parity beyond the Core model (icons, colors, checkboxes, hyperlinks, remembered expansion, etc.)
- Adapting `libguide` or its forks as the Format v1 implementation
