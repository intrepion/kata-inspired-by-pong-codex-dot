# Windows Softbuffer contact-timing recheck

On 2026-11-12, Softbuffer was freshly built in an isolated Windows MSVC debug target from `4f4da1dc2fa82025786383dc1ea0ca38ba23ff7a` (the adapter, shared core and scenario match `c77bf3a`). The executable SHA-256 was `d7db34913f2421a961a91945c4cd2d326e7c5f1b62c408f94ca9234019206fd6`.

The app's scripted replay exited 0 at tick 1440, emitted 13 records and six app-owned PNGs, and produced empty stderr. Its [telemetry](evidence/windows-softbuffer-contact-timing-20261112.jsonl) matched the revised model reference for every discrete field and had maximum absolute numeric difference zero (comparison allowance 0.001). The PNGs were generated but not visually reviewed. Scripted input and app-owned buffers do not establish OS input or desktop presentation.

Separately, the freshly built normal game was opened with supported Windows automation. Space started play; ball motion and a score of 1:0 were visible; Space paused; R restarted to 0:0 Playing; Escape closed the window. A brief W press produced no discernible paddle displacement, so sustained movement and release remain unverified. Full match, focus/resize, physical keyboard, package and clean-machine qualification remain open. This smoke check is not a collision-specific acceptance test.
