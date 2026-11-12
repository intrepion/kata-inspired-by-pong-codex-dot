# SDL3 macOS ARM64 local package pilot

2026-11-12 UTC. App-owned `experiments/sdl3/packaging/package-macos.cjs` produced a minimal ARM64 `.app` and portable archive using the same release binary measured in the [Mac benchmark](../benchmarks/SDL3-MAC-PILOT.md). [Checksum manifest](sdl3-mac-package-manifest-20261112.json). No public binary upload or system install.

Passed: Node syntax, plist validation, archive creation and extraction outside the checkout. `otool -L` reported system frameworks/libraries. Using the supported native-app GUI tool to launch the exact extracted bundle, Ready rendered; Space began play; held W/S moved the paddle in opposite directions; scoring reached0:3; Space paused with the pause indicator visible; resume and R restart returned to0:0 Playing. Escape exited, confirmed by no eligible process. This is bounded native interaction, not a full match/acceptance pass.

The exact staged `.app` was then removed after matching executable/source and archive SHA-256 checks. The original release binary, archive and neighbor sentinel remained unchanged. Build artifacts were preserved. This qualifies a local removable-bundle staging exercise, not a clean-machine installer, upgrade or `/Applications` lifecycle.

Not distribution-ready: complete third-party notices, native-source license review, Developer ID signing/notarization, downloaded/quarantined Gatekeeper behavior, clean-machine checks and Intel macOS remain pending. No security settings or signatures were changed.

Both Mac package creators also passed the6-case [guard regression suite](MACOS-GUARD-TESTS.md).
