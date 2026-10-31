Type: grilling
Status: open

## Question

What exact equality bar should Format v1 fixtures use to prove Round-trip for the Core model?

Candidates include: byte-identical rewrite of supported files; structural equality of the Core model after load→save; canonicalised `.gde` bytes; or a hybrid (e.g. structural for edited Guides, byte-stable for untouched fixtures). Pick the bar the CLI and tests will enforce, including what may differ (timestamps, ordering, non-Core attributes if preserved).
