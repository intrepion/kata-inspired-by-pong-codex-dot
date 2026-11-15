# bracket-lib held-input Mac relocated package check

2026-11-13, Mac ARM64. This is a local package check, not release qualification.

The existing Release pilot archive was built from source commit `198f9a46c560f71eeafc782fd9db10318308cd28`, including the held-input pause fix. Prior build and eleven release tests are documented in [the pause follow-up](../BRACKET-PAUSE-HELD-INPUT-FIX.md); they were not rerun for this check. At checkpoint `e270880b147c8f373071ca83db969ec9772ba7d6`, the bracket-lib main source and manifest, Cargo.lock, core and scenario library sources, and smoke scenario matched that historical commit. This is a comparison of those named files, not a fresh build at the checkpoint.

- Archive `pong-bracket-lib-macos-arm64-pilot.tar.gz`: SHA-256 `4a5b9655ab189550084d65bbc4b06c34557152713b18ed11c3f24149fbba0493`.
- Extracted executable SHA-256 `2518e0228d105b89f392c3e50efd09774d6591a745b5b6c4e3e83c36d62b90e7`, matching the package manifest. The extracted Info.plist passed `plutil -lint`.
- Extracted into `/tmp/pong-bracket-pause-relocated-20261113-v1`. Its executable ran `--scripted-test --scripted-captures .../captures` to exit 0; all 13 JSONL records exactly matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six app-owned PNG capture files were emitted. The captures were not visually inspected in this check.
- Separately, supported desktop automation opened the relocated app, pressed Space and observed Playing at 0–0, pressed Space and observed Paused at 0–1, then pressed R and observed Playing at 0–0. Q closed the app. These are sequential automated key actions and observed window states, not physical input or a held-through-pause test.

The archive and extracted evidence remain local. Clean-machine installation, signing/notarization, dependency notice completeness, accessibility, physical input, held-across-pause behavior and distribution remain open. No package removal was performed.
