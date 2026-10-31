# Greenfield `.gde` codec for Format v1

Format v1 needs a library that loads and saves The Guide v2 `.gde` files for the Core model. We decided to write that codec **greenfield from primary sources**, not by adapting `libguide` or forks (e.g. `onderweg/the-guide-non-windows`) as the implementation. Reuse would couple us to foreign Windows-era C, unclear licensing/portability baggage, and an API shaped for The Guide rather than NextGuide; a greenfield Core-model codec stays small, MIT-aligned, and owned here. Third-party format notes (Kaitai, etc.) may still be consulted as secondary pointers — they are not the implementation and not the source of truth.

## Considered Options

- **Adapt `libguide` / non-Windows fork** — faster first parse; inherits architecture-specific file quirks, C/Windows history, and someone else's abstraction boundary.
- **Greenfield from primary sources** (chosen) — slower start; clear ownership, Core-model-sized surface, Round-trip behaviour we can specify and test with fixtures.
