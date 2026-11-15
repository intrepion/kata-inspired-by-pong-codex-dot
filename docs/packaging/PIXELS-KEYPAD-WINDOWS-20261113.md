# Pixels keypad Enter: Windows check, 2026-11-13

On Windows, checkpoint `8f724afb69efd4a86a5453c11d22019a4e1fbd36` passed the locked, offline Pixels debug test suite: 15 passed, including the keypad Enter input regression. The tests inject winit key codes; they do not establish delivery from a native or physical keypad. The adapter source Git blob was `d4fbae5aa5bd454190fc4b100f518b8c07b72997`.

The locked, offline debug build succeeded. Its executable SHA-256 was `9775eb910b3aec673b63b9f58804b16e27865624efc30d86fb86a51c8a327beb`. An app-owned scripted replay exited successfully and all 13 telemetry records matched `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. It generated six captures. The inspected tick 480 capture showed the paused court, paddles, ball and 0–1 score; its SHA-256 was `70a07dcc3beabcdf24451b0e27e564b3a609a1f6cb9d5d7442b887ca00d79890`. The local replay logs and captures are retained under `pixels-keypad-windows-20261113` alongside the preserved earlier executable and test binary.

The replay does not exercise keypad input. Native keyboard delivery, physical keypad and accessibility input, complete interactive gameplay, clean-machine installation, dependency notice review and release qualification remain unverified. This check does not qualify raw wgpu on Windows.
