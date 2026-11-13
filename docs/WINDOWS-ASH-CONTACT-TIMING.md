# Windows Vulkan/ash contact-timing recheck

On 2026-11-13, Vulkan/ash was built from `69c3a2e1474ca4cf30c7e539bbf65d35d6c98ff3` with locked offline dependencies in Windows MSVC debug mode, using the existing cache. All six adapter/model tests passed. Executable SHA-256: `a42020f1638e3993bb4239c3a7e646b8157ef5fa8e7c98572d265874b8095115`.

The scripted replay exited 0 at tick 1440 and produced 13 [telemetry records](evidence/windows-ash-contact-timing-20261113.jsonl) and six app-owned PNGs. Telemetry matched the contact-timing model reference line-for-line. Stderr identified the Vulkan device as Intel UHD Graphics 630. The paused tick 480 capture was inspected and showed score, court, paddles, ball and pause marker. This app-owned image is not a desktop screenshot or proof of OS input.

Separately, supported Windows desktop automation opened the normal game. Space changed Ready to Playing; later Space showed Paused with a moved ball; R returned to 0:0 Playing with centered ball; Escape closed the window. This is a bounded automated native smoke check, not sustained movement/release, physical keyboard, full match, focus/resize, package or clean-machine qualification. Earlier binaries and captures remain historical.
