# Windows Piston contact-timing recheck

On 2026-11-13, Piston was built from `971319ea96bb509248e2bb5b2905db0a777c1f10` with locked offline dependencies in Windows MSVC debug mode, using a compatible existing dependency cache. All seven adapter/clock tests passed. Executable SHA-256: `28916cbdd09b97d4f7b6ffb3f89298b610d6342642d8e9924997eaa43c3204d6`.

The scripted replay exited 0 at tick 1440 with empty stderr, 13 [telemetry records](evidence/windows-piston-contact-timing-20261113.jsonl), and six app-owned PNGs. Telemetry was byte-identical to the contact-timing model reference. The paused tick 480 capture was inspected and shows the court, paddles and ball. This is an app-owned render target, not a desktop screenshot or proof of OS input.

No normal interactive test was performed in this follow-up. Native and physical input, full match, focus/resize, packaging and release qualification remain open for this revision. Prior binaries and captures remain historical.
