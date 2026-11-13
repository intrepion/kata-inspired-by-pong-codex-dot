# Pixels in-court labels and action prompts — macOS, 2026-11-13

Pixels now labels the score sides YOU and CPU and displays ENTER TO START, SPACE TO RESUME, or ENTER TO RESTART for the applicable non-playing state. The instruction has a dark backing to keep the net and ball from obscuring it, and is absent in play. Input and physics are unchanged.

All 16 offline locked release-profile tests passed on Mac, including framebuffer checks for side labels, required glyphs and state-dependent hints. In the rebuilt native window, the score labels and start/resume prompts were legible. An unattended match ended CPU WINS at 3–7 with ENTER TO RESTART visible. Supported desktop automation sent Return to restart at 0–0, clearing the prompt; Escape closed the app. These observations do not establish physical-input behavior.

The executable SHA-256 is `8a83b9fe817a5756a83a0e000491e26b50dde763dfac1ca4b6454a7f92e5f988`; edited adapter SHA-256 is `e7a0399d394a9ec3afde163882783fd83ca8e2528e55988cdd54947d6771349f`. The isolated mixed tree used Pixels adapter base blob `d4fbae5aa5bd454190fc4b100f518b8c07b72997` from checkpoint `795a5315b44bf7e2f3a53e2649d60ff1f7a50160`; other inputs were not all compared. The prior source and executable and candidate app remain under `/tmp/pong-pixels-court-help-mac-20261113-v1`.

Player-win rendering, physical input, other sizes and platforms, and release qualification remain unverified. The [Mac Pixels and wgpu license screen](PIXELS-WGPU-MAC-LICENSE-SCREEN.md) is a manifest screen rather than complete clearance; native/system components, notices and distribution checks remain open.
