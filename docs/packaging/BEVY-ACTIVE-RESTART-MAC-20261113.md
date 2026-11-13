# Bevy active restart held movement

2026-11-13, macOS ARM64. An isolated Bevy adapter regression reproduced that a movement key held while R restarts a running match remained active: expected zero axis, observed -1. This was a model-level test, not physical keyboard delivery. The pre-fix source came from checkpoint `f111201563ce12c0beb8d0db9762cbd7bad743f8`.

The R restart now uses a helper that restarts the game and marks the movement gate for a fresh playing sample. Any of the four movement keys held through restart remains suppressed until released, and a fresh press works. A regression covering all four bindings failed before the fix and the full offline locked release-profile Bevy suite passed 6/6 afterward. A cached Mac release build also succeeded. Changed main.rs SHA-256 `4f35a5519e99d1edb8a8581c4b613a57e79907cf4d4ad0d436045c1808d4301a`; rebuilt executable SHA-256 `2f642c7ab2b55a3ae97ad0dbece4826b62188a947ed11cf621046cec7e73c34b`.

These checks do not establish graphical replay, operating-system input delivery, physical keys, Windows behavior or release qualification. In particular, this change does not resolve or re-evaluate the separate Windows Bevy replay crash. Isolated source and target remain under `/tmp/pong-mac-input-recheck-20261113-v1` and `/tmp/pong-bevy-nonplaying-mac-target-20261113-v1`; prior artifacts were retained.
