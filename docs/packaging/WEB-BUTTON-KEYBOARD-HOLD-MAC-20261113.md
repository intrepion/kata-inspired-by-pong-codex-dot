# Mac shared web movement-button keyboard regression

2026-11-13. The shared controller source at checkpoint `060f2abc3fb133311533b01aca9b13152101adea` was fetched into the isolated source overlay described in [the Mac input report](NONPLAYING-INPUT-MAC-20261113.md). Source SHA-256: `c55c301fd53c1a1391dbad72f44b3b517818b212c1bc2c71a5af87a2f80f162f`. The offline locked release-profile `pong-web-ui-common` test suite passed 17/17 on macOS ARM64, including `movement_button_keys_are_independent_and_clear_on_blur_pause_and_restart`, and the previous held-through-match-end test.

This is a controller/model test, not browser event delivery or physical keyboard evidence. The wasm32 target is installed but `wasm-bindgen` is not available on PATH, so no new matching browser artifacts were built or run on this Mac. No desktop wrapper, device or release qualification is claimed. Existing source and build artifacts were preserved.
