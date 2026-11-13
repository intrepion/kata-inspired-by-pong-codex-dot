# Windows nannou contact-timing recheck

On 2026-11-13, nannou was built from `c9b957137c423ee03ce23d351a0cf8107a1296cf` with locked offline dependencies in Windows MSVC debug mode, using the existing dependency cache. Both adapter tests passed. Executable SHA-256: `2be4eaf75f7c9a211066a16964e04b6df28331fc8f3f13b5d9cff98739bd30d5`.

The scripted replay exited 0 at tick 1440 and produced 13 [telemetry records](evidence/windows-nannou-contact-timing-20261113.jsonl) and six app-owned PNGs. Telemetry matched the contact-timing model reference line-for-line. The tick 480 capture was inspected and shows the court, score, paddles and ball. This app-owned image is not a desktop screenshot or proof of OS input.

Stderr included a Vulkan loader error for a missing Epic Online Services overlay layer manifest; the application continued using Intel UHD Graphics 630 Vulkan and completed. A requested Mailbox presentation mode also fell back to Immediate. No system settings were changed. No normal interactive test was performed in this follow-up. Native and physical input, full match, focus/resize, packaging and release qualification remain open for this revision. Prior binaries and captures remain historical.

## Pause-indicator recheck

At source `22efeab0b1b37c1fc1c784d6734ddcdb4113e42f`, including `c457f7a`, the prior binary was preserved and nannou was rebuilt with locked offline Windows MSVC dependencies. Both adapter tests passed. New executable SHA-256: `6f95e93f54174ed93b8e9be25f166d3e3e8cb4db5b413c976e41148f38815141`.

The fresh scripted replay exited 0, produced six app-owned PNGs, and its 13 [telemetry records](evidence/windows-nannou-pause-20261113.jsonl) matched the model reference line-for-line. The inspected tick 480 frame displayed `PAUSED • SPACE TO RESUME`; the tick 600 frame no longer displayed it. The same Vulkan overlay-manifest error and Mailbox-to-Immediate fallback appeared in stderr; the replay continued on Intel UHD Graphics 630. These images are app-owned, not desktop screenshots.

Separately, supported Windows desktop automation opened the fresh normal game: Enter started play, Space showed the pause label, Space resumed with the label gone, and title-bar Close was followed by the window disappearing. This is a bounded automated native check; sustained or physical keyboard input, full match, focus/resize, package and clean-machine qualification remain open.
