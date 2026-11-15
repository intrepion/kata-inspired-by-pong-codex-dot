# Macroquad contact-timing replay on Omarchy

2026-11-13, Omarchy ARM64; source `617160ceb4573ff06f7339c7dbc98787d54f55df` includes contact-timing physics `c77bf3a`. A fresh offline locked Macroquad release build passed; its own test target contains zero tests. Shared Rust core passed 19 model tests and the scenario suite passed eight tests.

The local Macroquad portable pilot archive was extracted into a separate directory; its archive digest and all four extracted manifest entries verified. Archive size: 651,414 bytes; archive SHA-256: `216100dc27ca886b82ced042b97644e803580de42c22815e45957f2f8db9da94`; binary SHA-256: `e916a0ec2665724803d0c503baa5d2a674e028d67ea0dda80f0e8242f09aaba4`.

With approved host-display access, the relocated binary completed `--scripted-test --scripted-captures` with exit 0. Its 13 telemetry records exactly match the revised contact-timing model reference through finished tick 1440. Six app-owned frame PNGs were produced; tick 600 was inspected and showed the score, court, paddles and ball. These are application captures, not desktop/compositor screenshots. Archive, extraction, captures and telemetry remain under distinct `/tmp/pong-macroquad-c77-*` paths; historical artifacts were preserved.

This is a local package extraction and scripted rendering/replay check. Physical input, focus/resize, full-match interaction, clean-machine installation, complete dependency notices and public distribution remain unverified.
