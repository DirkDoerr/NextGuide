Type: grilling
Status: open
Blocked by: 01

## Question

How should the Format v1 Rust workspace be laid out?

Decide crate boundaries (e.g. library crate for the Core-model codec vs binary crate for the Round-trip CLI), repository paths, and how that layout supports shipping one executable per OS/arch while keeping the library reusable for a later UI — under the minimal-dependencies rule.
