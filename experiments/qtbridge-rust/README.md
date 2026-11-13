# Pong / Qt Bridge for Rust

Actual [Qt Bridge for Rust](https://github.com/qt/qtbridge-rust), pinned to upstream commit `cdd681c4efd0ccb9e670c407bf5b9a0e4962ecb3` (0.4.1). This is not a substitute CXX-Qt application. Qt Bridge itself currently uses CXX-Qt internally. Rust owns the shared `pong-core` simulation; Qt Quick/QML draws the court and sends keyboard state to Rust slots.

## Run

Requires Rust >=1.88, a C++ toolchain and Qt >=6.10 with Qt Core/GUI/QML/Quick and matching private development headers. `qmake` must be discoverable (or specify `QMAKE=/path/to/qmake6`). Do not install a different Qt version globally just for this experiment.

```sh
QMAKE=/usr/bin/qmake6 cargo run --manifest-path experiments/qtbridge-rust/Cargo.toml
```

On Windows, use Qt's matching MSVC kit and Rust MSVC toolchain. macOS requires its Qt kit and C++ toolchain. The qmake path above is Linux-specific.

Enter/R starts or restarts; W/S or arrows move; Space starts or pauses/resumes; Escape pauses. Focus loss clears input and pauses. Court scales with the window. A Qt timer requests snapshots, but Rust uses monotonic elapsed time and fixed 120Hz steps with 100ms bounded catch-up, not timer-count physics.

## Tests without Qt

```sh
cargo test --manifest-path experiments/qtbridge-rust/Cargo.toml --no-default-features
```

Nine controller/gameplay tests cover agreement with the shared core, snapshot schema consumed by QML, movement bounds, bounce behavior, scoring/game-over, pause/resume/restart, invalid time, bounded catch-up and irregular frame partitioning. Optional Qt dependencies still need initial Cargo resolution; `--no-default-features` is not a promise of an offline first build.

## Licensing / distribution

The Qt Bridge upstream library is **LGPL-3.0-only OR Qt Commercial**, not MIT. Qt modules have their own LGPL/GPL/commercial choices; check the precise modules and versions shipped. The repository's application license does not replace dependency licenses.

For LGPL distribution, retain notices and license texts, provide required corresponding library source and modification notices, and permit replacement/relinking as required. Rust libraries are normally statically linked, so simply using Qt shared libraries does NOT by itself settle Qt Bridge's LGPL obligations: provide a compliant relinking path/artifacts or obtain an appropriate commercial license. Do not distribute signed/locked-down binaries until these obligations are reviewed. A Qt commercial license must actually cover the components and use; it is not implied by using this example. This experiment adds source, not a packaged redistribution or legal advice.

## Platform / maturity limits

Upstream currently lists Linux x86_64 and Windows x64; macOS ARM64 is experimental. Omarchy ARM64 is outside that stated matrix, even if local validation succeeds. Qt Bridge 0.4.1 is early-stage; this pinned implementation is a comparison experiment, not a support guarantee. QML is embedded, but Qt runtime libraries/plugins are not bundled. Browser/WASM/mobile targets are not provided.

Validation results are in VALIDATION.md. Build, launch and interactive gameplay are reported separately.
