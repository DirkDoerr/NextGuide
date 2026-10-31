# Rust for Format v1

Format v1 needs a greenfield Core-model `.gde` library plus a CLI, shipped as easy-to-install executables on Windows, macOS, and Linux, with few dependencies, and with optional later in-process reuse on Android/iOS. We chose **Rust** and a hard **minimal-dependencies** rule: one (or few) crates we own, prefer the standard library for binary I/O, add third-party crates only when the cost of not doing so is clear. Go was the better fit for “desktop CLI tooling only”; Rust won because the README’s optional mobile targets make a dead-end desktop-only codec an expensive mistake if this library is meant to stay the format layer.

## Considered Options

- **Go** — excellent static/simple desktop binaries and a lean stdlib culture; weaker long-term embedding story for Android/iOS UI.
- **Rust** (chosen) — same desktop binary ship shape when deps stay minimal; natural later as a native library inside mobile/desktop apps.
- **Node / JVM / .NET** — rejected for Format v1 because easy install and few runtime dependencies matter more than ecosystem familiarity right now.
