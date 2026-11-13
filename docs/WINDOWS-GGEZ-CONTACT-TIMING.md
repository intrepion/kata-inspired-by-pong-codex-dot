# Windows ggez contact-timing recheck

On 2026-11-13, ggez was built in Windows MSVC debug mode from `0870089d5b7d747ae42addc5a94b24cab3384a57` with locked offline dependencies and the existing compatible cache. Both adapter tests passed. Executable SHA-256: `ff9907b02ae32c2b7e4f403f477b12ecf18419c5c3dc6cb48e32befbc31b4f88`.

The scripted replay exited 0 at tick 1440 with empty stderr, 13 [telemetry records](evidence/windows-ggez-contact-timing-20261113.jsonl), and six app-owned PNGs. Telemetry was byte-identical to the contact-timing model reference. The paused tick 480 app-owned render capture was inspected and showed the court, score, paddles, ball and pause text. This capture is not a desktop screenshot or proof of operating-system input.

Separately, the fresh normal game was opened with supported Windows desktop automation. Enter started play, ball movement and a 1:0 score were visible, Space showed Paused, R reset the score to 0:0, and the title-bar Close action was followed by the window disappearing. This is a bounded automated native smoke check, not physical keyboard or sustained movement/release, collision-specific acceptance, full-match, focus/resize, package or clean-machine qualification. Prior binaries and captures remain historical.
