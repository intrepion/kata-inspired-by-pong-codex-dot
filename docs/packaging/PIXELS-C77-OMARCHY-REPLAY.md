# Pixels contact-timing replay on Omarchy

2026-11-13, Omarchy ARM64; source `7495366` includes contact-timing physics `c77bf3a`. A fresh offline locked release build and all five Pixels frontend tests passed. The local Linux portable pilot archive was extracted into a separate directory; its archive digest and all four extracted manifest entries verified. Archive size: 4,005,003 bytes; archive SHA-256: `522611c3fdf54af4525d5ae542e3975d97b9dab68613b34d7ff9f65b31e0937d`; binary SHA-256: `3d59271754e7d95b3a7b347b2e6d6b4236422b1214bf32bccd689f18ad063ac9`.

With approved host-display access, the relocated binary completed `--scripted-test --scripted-captures` with exit 0. Its 13 telemetry records exactly match the revised contact-timing model reference through finished tick 1440. Six app-owned source-buffer PNGs were produced; tick 600 was inspected and showed the court, paddles and ball. These are source-buffer captures, not displayed-window or desktop/compositor screenshots. Archive, extraction, captures and telemetry remain under separate `/tmp/pong-pixels-c77-*` paths; historical artifacts were preserved.

This is a local package extraction and scripted rendering/replay check. Physical input, focus/resize, full-match interaction, clean-machine installation, complete dependency notices and public distribution remain unverified.
