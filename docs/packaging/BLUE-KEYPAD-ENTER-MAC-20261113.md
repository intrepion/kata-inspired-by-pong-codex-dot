# Blue Engine keypad Enter — macOS, 2026-11-13

Blue Engine now accepts `NumpadEnter` for the existing Enter action: start, resume, or restart a finished match. The main and keypad Enter bindings are separately focus-gated; R remains the active restart.

All 12 offline locked release-profile tests passed on Mac, including synthetic focus-return tests for the five action bindings and independent main/keypad suppression. The build emitted a future-incompatibility warning for `block 0.1.6`. The first test was stopped after it began compiling dependencies into an uncached target; the successful test and build used an existing release cache. Partial artifacts were retained.

In the rebuilt native window, the supported desktop tool's `KP_Enter` command started from Ready, and a second command resumed after Space paused at 2–2. The title showed Playing and the score subsequently advanced. The tool's mapping to physical keypad input was not independently observed; physical keyboard delivery, focus-return timing, other platforms and release qualification remain open.

The candidate executable SHA-256 is `c4a1a403b30a8d6b53e422fec4d2fc68f75ac03657f1bd88f7cba91a2296f584`; edited adapter SHA-256 is `494f240845515abfb112aded1be592dba360851102e25501235e69822d7ce3fb`. The isolated mixed tree used adapter base blob `aa64770f5776007336bd0827f42715354d15be63` from checkpoint `d26a84504b6e8783b6c556db1d65ff14b63cd721`; other inputs were not all compared. The previous source and executable and candidate app remain in `/tmp/pong-blue-keypad-mac-20261113-v1`.
