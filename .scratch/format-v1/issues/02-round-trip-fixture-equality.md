Type: grilling
Status: resolved

## Question

What exact equality bar should Format v1 fixtures use to prove Round-trip for the Core model?

Candidates include: byte-identical rewrite of supported files; structural equality of the Core model after load→save; canonicalised `.gde` bytes; or a hybrid (e.g. structural for edited Guides, byte-stable for untouched fixtures). Pick the bar the CLI and tests will enforce, including what may differ (timestamps, ordering, non-Core attributes if preserved).

## Answer

**Hybrid:**

1. **CI gate (all fixtures):** `load → save → load` must preserve the **Core model** structurally — tree topology, node titles, and opaque RTF body bytes. On-disk `.gde` bytes may change.
2. **Additional gate:** Guides produced by Format v1’s **canonical writer** (and owned “canonical golden” fixtures) must **byte-stable** Round-trip (`save` of a load equals the prior bytes under that writer).

Non-Core attribute handling is deferred to [Non-Core attributes policy](03-non-core-attributes-policy.md); it may affect whether The Guide–written files can ever be byte-identical, but must not weaken the Core-model structural gate.
