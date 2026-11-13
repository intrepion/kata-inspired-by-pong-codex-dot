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
