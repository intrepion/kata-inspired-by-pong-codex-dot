# Fyrox HUD scale correction

2026-11-12, Mac ARM64, based on `993777b`. The court already scaled to the
window, but title font size and header height stayed at defaults. They now scale
with the same court factor (18-pixel base font, 65-pixel base header height).
No physics, input or timing changes.

[Before](evidence/fyrox-hud/before.png) / [after](evidence/fyrox-hud/after.png)
are native tick-600 captures at 1600×1200. Score and full instruction line are now
readable without clipping; both paddles, divider and ball remain visible. Two
existing adapter tests passed; cached release build passed. Native replay exited
zero in 13.828 seconds, thirteen states through finished tick 1440, six PNGs.

Binary SHA-256:
`21880f7e337191f1dd9611f885a57acd044a89280eb5645c753be48bd1d9ce81`.
This validates the observed high-resolution layout, not all screen sizes or
manual input. Windows/Linux and newly packaged-binary requalification remain.
Earlier benchmark/package reports retain their original hashes and screenshots.
No new performance ranking or distribution readiness is claimed.
