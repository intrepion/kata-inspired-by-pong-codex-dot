# Ratatui Mac local debug archive pilot

On 2026-11-13, the existing macOS ARM64 debug executable from source `e0cd5d23092d37df59bfa4ab352b5236f03d0f22` was staged with the repository MIT LICENSE and historical Ratatui README, compressed as a local tar.gz, and extracted to a separate directory. The extracted binary SHA-256 `97ca210aa2923ea22327d27686813b16f3f92a15d2c4e0ad7eec5816e417cdae` matches the previously tested binary; archive SHA-256 `28746036cdb0876e76eac38f9f91758d0a02b03940644d956ae85a398c17da3d`. The extracted LICENSE and README matched their staged originals. The binary is an arm64 Mach-O executable.

From the extracted directory, `--scripted-test --scripted-cells` exited successfully; its 13 JSON telemetry lines matched the earlier Mac replay byte-for-byte, and it produced six text/JSON TestBackend cell pairs. The tick-480 text contains Paused. This is app-owned cell/model replay evidence, not a terminal screenshot or real key delivery.

At checkpoint `a2e825d8a498fd0da49969fb7fcc99a830e09afc`, the Ratatui frontend source, its manifest and lockfile, shared core and scenario source, and the smoke scenario were checked against the historical build revision and had identical Git blob IDs. This is not a new build, nor proof that every build input or runtime dependency is unchanged. The staged README is historical and may contain superseded cross-platform statements.

The local archive, staging, extraction and replay outputs remain under `/tmp/pong-ratatui-mac-package-20261113-v1`; no cleanup or uninstall was attempted. This debug archive includes the first-party MIT license only: dependency notice completeness, signing, clean-machine execution, native and physical input, installer behavior, and public distribution remain unverified.
