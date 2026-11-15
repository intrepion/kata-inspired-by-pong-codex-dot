# Pixels native court states: Mac (2026-11-13)

The Pixels source, core, scenario source and Pixels lockfile used here match the corresponding Git blobs at `ce015a506bbaab4851fbaec995803f61ff01e612`. Pixels source SHA-256: `26f3d4ae8bd18ee7fbfd431e15f83247904f7292a236905f1f54b586e208f788`. A cached offline release build produced executable SHA-256 `49a879c01da5be7abf8acc7d236a45d97032c17dc29e691621e0015c1e5cb0b2`. The separate app bundle and previous binary were retained under `/tmp/pong-pixels-court-native-mac-20261113-v1/`.

In the native Mac window, desktop automation observed Ready and 0–0, sent Return to start, and sent Space to pause. The title and court showed Paused and 1–1. After Return restarted play, the unattended match reached 1–7; the court showed “CPU WINS” and the title showed Over. Automated Return followed by Space produced a Paused 0–0 display. Scores and labels were visually consistent at those observed points.

This is an automated desktop interaction with an unattended match, not physical keyboard or paddle movement testing. A player win, other sizes, and release qualification were not checked. The separate earlier software-frame fixture and 14-test result are documented in `PIXELS-COURT-STATES-MAC-20261113.md`.
