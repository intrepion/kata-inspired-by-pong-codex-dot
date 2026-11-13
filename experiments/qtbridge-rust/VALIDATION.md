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

## Actual Qt/QML integration checkpoint

Omarchy Qt6.12: the dedicated production-QML/Backend integration renderer compiled and passed actual slot calls, timed axis movement, stable pause, resume/opposite movement, restart/reset and reduced-window scaling assertions. A deliberately invalid snapshot-schema assertion prevented render completion (negative control); the unchanged production contract produced a fresh valid 900x500 PNG. The image was visually inspected: background, center net, both paddles and ball present at expected positions. Evidence: `evidence/offscreen-court.png`.

This used `QT_QPA_PLATFORM=offscreen` and `QT_QUICK_BACKEND=software` only for the child process. It is synthetic application-owned test output, not desktop capture or interactive gameplay QA. No SDK installation or desktop-access workaround. Physical key/focus delivery and real GPU presentation remain unverified.
