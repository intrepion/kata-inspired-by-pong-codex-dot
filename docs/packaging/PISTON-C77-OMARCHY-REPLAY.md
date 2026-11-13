# Piston contact-timing replay on Omarchy

2026-11-13, Omarchy ARM64; source `38584cd5eb895fab314745039c0f6403510965ca` includes contact-timing physics `c77bf3a`. A fresh offline locked release build and all seven Piston frontend tests passed. The local Linux portable pilot archive was extracted into a separate directory; its archive digest and all four extracted manifest entries verified. Archive size: 4,052,549 bytes; archive SHA-256: `9541985844268e337dc5751004c6703cb9ba192a763e856186d9ec2c510b6d60`; binary SHA-256: `a06c402efe98df158cd2d9f4446485b27fb404ac9316374594c1070dd8b984e1`.

With approved host-display access, the relocated binary completed `--scripted-test --scripted-captures` with exit 0. Its 13 telemetry records exactly match the revised contact-timing model reference through finished tick 1440. Six app-owned render-target PNGs were produced; tick 600 was inspected and showed the court, paddles and ball. These are application captures, not desktop/compositor screenshots. Archive, extraction, captures and telemetry remain under distinct `/tmp/pong-piston-c77-*` paths; historical artifacts were preserved.

This is a local package extraction and scripted rendering/replay check. Physical input, focus/resize, full-match interaction, clean-machine installation, complete dependency notices and public distribution remain unverified.
