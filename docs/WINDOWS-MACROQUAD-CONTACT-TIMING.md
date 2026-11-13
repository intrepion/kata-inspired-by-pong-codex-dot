# Windows Macroquad contact-timing recheck

On 2026-11-13, Macroquad was built from `38584cd5eb895fab314745039c0f6403510965ca` with locked offline dependencies in a fresh Windows MSVC debug target. The adapter test command passed with zero tests; it is a build check, not an adapter behavior test. The executable SHA-256 was `16df3e6edb9681afc513a84db5f8110c6f1492fb8776fd08016c7710caeb5e3a`.

The scripted replay exited 0 at tick 1440 with empty stderr, 13 [telemetry records](evidence/windows-macroquad-contact-timing-20261113.jsonl), and six app-owned PNGs. Telemetry was byte-identical to the contact-timing model reference. The paused tick 480 capture was inspected and shows the court, score, paddles, ball and paused status. This is an app-owned capture, not a desktop screenshot or proof of OS input.

No normal interactive test was performed. Native and physical input, full match, focus/resize, packaging and release qualification remain open for this revision. Earlier binaries and captures remain historical.
