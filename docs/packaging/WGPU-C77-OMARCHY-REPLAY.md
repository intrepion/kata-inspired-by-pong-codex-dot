# Raw wgpu contact-timing replay on Omarchy

2026-11-13, Omarchy ARM64; source `663aa403e04a571cb1b43d9731fcb83090630436` includes contact-timing physics `c77bf3a`. A fresh offline locked release build and all four raw-wgpu frontend tests passed. The local Linux portable pilot archive was extracted into a separate directory; its archive digest and all four extracted manifest entries verified. Archive size: 3,977,452 bytes; archive SHA-256: `935ef42f66bfa989daad259037f81795582ae18c4e2e05ca4c111bf03e904dc3`; binary SHA-256: `87069bb498b10af86e08e78da716a7909c96ee291b14cc18aa049917327a52e1`.

With approved host-display access, the relocated binary completed `--scripted-test --scripted-captures` with exit 0. Its 13 telemetry records exactly match the revised contact-timing model reference through finished tick 1440. Six app-owned GPU surface-readback PNGs were produced; tick 600 was inspected and showed the court, paddles and ball. These are surface readbacks, not desktop/compositor screenshots. Archive, extraction, captures and telemetry remain under separate `/tmp/pong-wgpu-c77-*` paths; historical artifacts were preserved.

This is a local package extraction and scripted rendering/replay check. Physical input, focus/resize, full-match interaction, clean-machine installation, complete dependency notices and public distribution remain unverified.
