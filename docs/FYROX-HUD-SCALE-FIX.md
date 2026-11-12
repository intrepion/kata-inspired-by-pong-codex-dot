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

## Windows HUD follow-up

Rebuilt the corrected Fyrox adapter after the original Windows timing pilot. New binary SHA-256 `e0143624378d61c956f525bef37051815281812ca9e87c6d41b2561c50ae5a48`. Native scripted replay exited 0 in 13.344s, thirteen states and six PNGs. Inspected 800x600 tick600: score and complete control line remain readable without clipping, court and paddles remain visible. This lower-resolution check does not substitute for arbitrary-DPI acceptance. Original timing results retain the old binary hash; no new performance claim.

## Omarchy ARM64 local package follow-up

Fresh optimized build with this HUD correction and two adapter tests pass. A new portable archive was extracted separately and all four manifest checksums verified. The extracted native process exits 0 after 13 replay states through tick 1440 and six application-owned PNG captures. Tick 600 inspection confirms readable score, state and control text; aspect-ratio letterboxing remains. Removing the five package-manifest files preserves an adjacent sentinel, but **clean uninstall is incomplete**: the engine generated `data/resources.registry` (2 bytes) inside the extracted folder. Empty-directory removal correctly refuses to delete it. This generated state is retained rather than recursively removed; package lifecycle documentation needs to account for it.

[Redacted package evidence](packaging/linux-fyrox-hud-package-20261112.json). Previous benchmark/package hashes remain historical. No new timing, real-keyboard, compositor, cross-distro or redistribution clearance is implied. Archive/captures remain local.
