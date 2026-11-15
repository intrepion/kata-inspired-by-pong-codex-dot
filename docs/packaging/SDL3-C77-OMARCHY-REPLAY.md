# SDL3 contact-timing replay on Omarchy

2026-11-12, Omarchy ARM64; source `49f2c2f3f55c36199b7ba28f40e565fd2454e321` includes contact-timing physics `c77bf3a`. A fresh offline locked bundled-SDL3 release build and both SDL3 frontend tests passed. The shared low-level controller's three tests and the SDL3 portable-packager test also passed.

The existing local portable helper created a 2,000,276-byte archive; the archive digest and all four extracted manifest entries verified. Archive SHA-256: `1f70d474237379d993f1455b84872522d66cb948409af794eb9387d75ed5aa95`; binary SHA-256: `ffc91cbe99bee7b75e44558a7b03f40a6bb4d0ffb85da0558279d8ced39e1af3`.

With approved host-display access, the relocated binary completed `--scripted-test --scripted-captures` with exit 0. Its 13 telemetry records exactly match the revised contact-timing model reference through finished tick 1440. Six app-owned SDL renderer PNGs were produced; tick 600 was inspected and showed the score, court, paddles and ball. These are renderer captures, not desktop/compositor screenshots. Archive, extraction, captures and telemetry remain under distinct `/tmp/pong-sdl3-c77-*` paths; previous artifacts were preserved.

This is a local package extraction and scripted rendering/replay check. Physical input, focus/resize, full-match interaction, clean-machine installation, complete dependency notices and public distribution remain unverified.
