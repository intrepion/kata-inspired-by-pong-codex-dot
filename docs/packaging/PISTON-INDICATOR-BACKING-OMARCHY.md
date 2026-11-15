# Piston state indicator backing — Omarchy, 2026-11-13

Piston's Ready/play symbol, pause bars and winner text previously shared the court with the net and ball. An opaque backing now covers those objects beneath the indicator, so they cannot visually merge with it. Playing has no backing. The source blob tested was `3ff785fcaa2a5986c7b02243e4d2dad1b9653783`, based on checkpoint `64f7f16e01e3a13f6c63463e2c59993b80c5547d`.

The offline locked debug build succeeded (binary SHA-256 `fdbd2574c7a8d42e1c7a9614cd875d34f3aa6d3d1e8b3a561587855b4b189c6a`) and all 16 tests passed. The added geometry test checks the backing contains every indicator cell for Ready, Paused and both winners, and is absent while playing.

With host-display access, the binary completed app-owned `--scripted-test --scripted-captures` with empty stderr and 13 records byte-for-byte equal to `docs/evidence/rust-core-contact-timing-model-20261112.jsonl`. Six application framebuffer captures were produced. The tick-480 paused image was visually inspected: [capture](../evidence/piston-paused-backing-omarchy-20261113.png), SHA-256 `2d9f10312c2132f69a2c615e7d760ee75a43bcfe636d873efc43e8816ed39d46`. Logs and captures remain in `/tmp/pong-piston-backing-20261113-a`.

This is scripted application rendering, not a compositor screenshot or physical-input test. The replay does not reach game over; winner and Ready backing geometry is unit-tested but not visually verified. Native interaction, other platforms, clean-machine installation, notices and release readiness remain unqualified. Earlier evidence is preserved.
