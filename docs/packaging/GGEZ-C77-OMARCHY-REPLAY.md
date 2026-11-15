# ggez contact-timing replay on Omarchy

2026-11-13, Omarchy ARM64; source `24fbcd915eca37963ae32e094aacf2a73a96cd37` includes contact-timing physics `c77bf3a`. A fresh offline locked release build and both ggez frontend tests passed. The local Linux portable pilot archive was extracted into a separate directory; its archive digest and all four extracted manifest entries verified. Archive size: 4,985,303 bytes; archive SHA-256: `cd70be788230e75545cfcdfa094779a3de8cd65158236bb717871963a8a7635a`; binary SHA-256: `dc4849c03305076a79988adcd8129a53785c9540694b1b7cf7ead6d2e8327fed`.

With approved host-display access, the relocated binary completed `--scripted-test --scripted-captures` with exit 0. Its 13 telemetry records exactly match the revised contact-timing model reference through finished tick 1440. Six app-owned render-texture PNGs were produced; tick 600 was inspected and showed the score, court, paddles and ball. These are application captures, not desktop/compositor screenshots. Archive, extraction, captures and telemetry remain under separate `/tmp/pong-ggez-c77-*` paths; historical artifacts were preserved.

This is a local package extraction and scripted rendering/replay check. Physical input, focus/resize, full-match interaction, clean-machine installation, complete dependency notices and public distribution remain unverified.
