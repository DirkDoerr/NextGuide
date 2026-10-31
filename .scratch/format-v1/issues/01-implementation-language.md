Type: grilling
Status: resolved

## Question

What implementation language and ecosystem should Format v1 use for the library and CLI?

No hard constraint was set at charting time. Choose with an eye to: greenfield binary I/O, testable fixtures, a small CLI, and eventual reuse by a later UI (without requiring that UI in this effort).

## Answer

**Rust**, with a hard **minimal-dependencies** rule.

Constraints that drove the choice: ship a real **executable**, **easy to install** (drop a binary per OS/arch), **reduce dependencies**, and stay credible for README targets — Win/macOS/Linux CLIs now, with the same Core-model codec later linkable in-process for optional Android/iOS (and desktop) UI. Go was the stronger “desktop tooling only” fit; Rust was chosen so Format v1 is not a dead-end when mobile becomes real.
