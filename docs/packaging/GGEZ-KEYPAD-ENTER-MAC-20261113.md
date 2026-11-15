# ggez keypad Enter — macOS, 2026-11-13

The ggez adapter now accepts `NumpadEnter` wherever Enter starts or resumes play or restarts a finished match. Enter during active play retains its existing no-op behavior; R remains the active restart. Repeat and unfocused input remain ignored.

All 10 offline locked release-profile tests passed on Mac. The new synthetic key-code regression exercises Ready, repeated and unfocused input, active play, pause/resume and match-end restart; the existing match-end held-movement test now includes keypad Enter.

In a rebuilt native window, the supported desktop tool's `KP_Enter` command started play from Ready and resumed a match paused with Space; the on-court state text cleared and the score subsequently changed. This is automated input, and the tool's mapping to a physical keypad event was not independently observed. Physical keypad input, other platforms and release qualification remain open.

The candidate executable SHA-256 is `e0c80a89d4263a38bf529298a4748e41ef59bdcedb5738917a3b7c8e83a1e40e`; edited adapter SHA-256 is `497f0c58f2ed59354ddcd068af34ce7c30495552c915a09cb4bd54bd00b52763`. The mixed isolated tree used ggez adapter base blob `a536df84c04622fd42c567067fda6900f80dae4e` from checkpoint `ece380529d64ce142243f2750e3c1878d4c2aa89`; other build inputs were not all compared. The previous adapter and executable and the candidate app are retained in `/tmp/pong-ggez-keypad-mac-20261113-v1`.
