# Qt Bridge checkpoint: 2026-11-13

- Omarchy ARM64, existing Qt 6.12.0, official Qt Bridge git revision pinned in Cargo.toml.
- Nine Qt-independent controller/gameplay tests PASS (`--no-default-features`).
- Full default-feature native build PASS (4m43s, one job, dev debug disabled).
- Native host launch sustained 10 seconds, no QML errors or stderr; terminated by deliberate timeout (124).
- Interactive gameplay/visual rendering NOT verified: native Computer Use inventory has no apps. Browser access is not native Qt access.
- Windows and macOS builds NOT attempted for this frontend. Upstream Linux support list is x86_64, so this ARM64 success is local evidence, not official support.
- No Qt installation, user checkout overwrite or binary distribution. Validation folder: `~/pong-qtbridge-validation`. Target may be recreated via documented Cargo commands.

Next manual check: start, hold W/S or arrows, verify score changes, Space freezes/resumes, R resets, switch focus pauses, resize preserves court proportions. Close the window to exit. Native executable testing does not involve file:// browser URLs.

The six added gameplay tests exercise snapshot schema, paddle bounds, pause/resume, paddle/wall collision, scoring through game-over/restart and irregular frame partitioning. These are deterministic controller tests, not substitutes for visual inspection or actual keyboard delivery. No desktop-access workaround was used.

## Toolchain availability, read-only check 2026-11-13

| Machine | Rust / C++ | Qt SDK | Result |
|---|---|---|---|
| Omarchy ARM64 | Existing Rust/C++ | Qt 6.12 Core + Quick/private development support | Full build, 9 tests with and without Qt features, native startup pass; interactive visual QA unavailable |
| Mac ARM64 | Rust 1.99.1; Apple clang 21.0.0 / selected Xcode | No qmake/Qt SDK or Quick headers found in PATH/standard Qt/Homebrew locations | Not built; needs Qt >=6.10 SDK + private headers; upstream macOS experimental |
| Windows x64 | Rust MSVC 1.99.1 installed (default GNU 1.99.0); VS2022 BuildTools MSVC 14.44.35207 | No qmake/Qt SDK or Quick headers found in PATH/standard Qt/MSYS2/vcpkg locations | Not built; needs Qt >=6.10 matching MSVC kit + private headers |

No SDK was installed. The checks are discovery of available toolchains, not proof that a custom unlisted Qt installation cannot exist. Visual evidence remains unavailable rather than substituting shell-driven desktop capture/input. The full Qt-linked test suite also passed; the nine tests exercise the controller, not the rendered UI.
