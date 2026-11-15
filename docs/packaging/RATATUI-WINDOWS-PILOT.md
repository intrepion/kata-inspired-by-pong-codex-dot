# Ratatui Windows portable cell-replay pilot

Fresh locked release build36.28s, five release tests passed (input clear/release/opposites, fallback stop, small/normal render, exported cell symbols/styles). Local new-only unsigned ZIP/extraction into a path containing spaces; extracted SHA-256 matched source. [Manifest](ratatui-windows-manifest-20261112.json). Archive contains executable, project LICENSE and incomplete-pilot warning. Source/build/staging artifacts retained, no system changes.

Extracted executable run from its own directory with `--scripted-test --scripted-cells NEW_DIRECTORY` exited0 in0.471s. Thirteen states through finished tick1440, six JSON cell buffers and six text exports; each JSON contains3000 cells (100x30) with symbols, foreground/background and modifiers. Tick600 exported header contains score0:1, Playing and control instructions.

This runs Ratatui TestBackend with deterministic scripted inputs, not a visible Windows terminal. It is NOT a screenshot, real console keyboard/focus/raw-mode restoration test, or performance benchmark. Terminal encoding/rendering and actual native-input handling remain unqualified by this batch.

Installer/clean-machine dependencies, third-party notices, signing/download security handling, upgrade/uninstall are separate pending gates. No user-data deletion or security-warning bypass occurred.
