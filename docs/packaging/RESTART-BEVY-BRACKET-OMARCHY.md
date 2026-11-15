# Active restart: Bevy and bracket-lib — Omarchy, 2026-11-13

Both adapters retained held movement when R restarted an active match. Mac independently published the Bevy fix at `698730e`; its polling gate resets on restart so already-held keys are suppressed on the next sample. Bracket-lib now marks its currently held movement bindings suppressed before restarting. Release followed by a fresh press restores movement. A locally constructed regression in each adapter failed before fixing; Bevy was then cross-checked using the independently published regression.

Full locked offline suites passed: `cargo test --offline --locked -p pong-bevy` (6 tests) and `cargo test --offline --locked --manifest-path experiments/bracket-lib/Cargo.toml` (14 tests). These are input-gate/event-handler tests on Omarchy; they do not exercise physical input, actual windows, rebuilt release packages, or other platforms. In particular they do not resolve the separately documented Windows Bevy replay crash. Prior artifacts remain historical.

Source Git blobs: Bevy `7494de30094aed31d01240f4f80d62f82e491de7`, bracket-lib `807249733a9c4c5bbeb6c6c3e67a9ba44e3c0e15`.
