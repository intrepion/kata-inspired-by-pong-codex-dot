# SDL3 match-end restart instruction — macOS, 2026-11-13

Mac SDL3 cross-check of the shared restart instruction at `4da734af7ce93cecefe01a6f28ac6bb43847d9b1`. The cached offline locked release build succeeded and all 12 SDL3 adapter tests passed.

The native window started with automated Space. An unattended match ended CPU WINS at 3–7; R TO RESTART was legible beneath the winner. Automated R restarted at 0–0 and cleared the instruction, and Escape closed the window. These are supported desktop automation and visual observations, not physical keyboard input.

The executable SHA-256 is `600eedf209d08267f0b0e6a087d4f3024e3e2e655ad14cfff5766f95b94a0ad9`. The isolated mixed source tree used SDL3 adapter blob `d7c995ac1f91f28dae8e74c02a69f9a58ddc0109`, shared renderer blob `5c3d07376e7089a0cd84a6cb5ce9836a94f07c1e`, and core lib.rs blob `8a9a1f2ecdcbfefde4d67e3f4df0d5f1d2930dd3`, matching that checkpoint; other inputs were not all compared. The previous executable and candidate app remain in `/tmp/pong-sdl-restart-hint-mac-20261113-v1`.

Player-win rendering, other sizes and platforms, physical input and release qualification remain unverified. The [Mac SDL3 HIDAPI audit](SDL3-MAC-HIDAPI-LICENSE.md) identifies an optional libusb loading path; it did not establish that libusb was loaded in its two historical snapshots. That license concern, other system components, notices and clean-machine checks remain open.
