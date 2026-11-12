# Softbuffer contact-timing replay on Omarchy

2026-11-12, Omarchy ARM64; source `cf3bff1d2958c7f157db9754a59a2b478fab9579` includes contact-timing physics `c77bf3a`. A fresh offline locked release build and all four Softbuffer frontend tests passed. The shared Linux portable-packager test passed.

The local portable pilot archive was extracted into a separate directory; the archive digest and all four extracted manifest entries verified. Archive size: 1,688,996 bytes; archive SHA-256: `96be6f5e80b163b2db01821fd319ec9887b8285706c3c38efb5549ede9f9044e`; binary SHA-256: `006b1d240a4a48fbbeba18a6a44b8ce84b3cc4025c41f6b93d29881a0443995e`.

With approved host-display access, the relocated binary completed `--scripted-test --scripted-captures` with exit 0. Its 13 telemetry records exactly match the revised contact-timing model reference through finished tick 1440. Six app-owned CPU presentation-buffer PNGs were produced; tick 600 was inspected and showed score, court, paddles and ball. These are buffer captures, not desktop/compositor screenshots. Archive, extraction, captures and telemetry remain under separate `/tmp/pong-softbuffer-c77-*` paths; historical artifacts were preserved.

This is a local package extraction and scripted rendering/replay check. Physical input, focus/resize, full-match interaction, clean-machine installation, complete dependency notices and public distribution remain unverified.
