Type: grilling
Status: resolved

## Question

May third-party format descriptions (e.g. Kaitai `.kty` from the non-Windows `libguide` fork) be used strictly as *reference* while Format v1’s codec stays greenfield?

Decide allowed vs disallowed use (read for cross-check, generate code, vendor files in-repo) without violating ADR-0001.

## Answer

**Reference only (aligns with ADR-0001):** humans/agents may **read** Kaitai (and similar) descriptions to cross-check understanding of The Guide 2.0 `guide.c`. **Disallowed:** codegen from `.kty`, vendoring them as the codec, or treating them as normative over primary sources. No requirement to copy them into this repo.
