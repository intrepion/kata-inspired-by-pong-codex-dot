# Windows nannou contact-timing recheck

On 2026-11-13, nannou was built from `c9b957137c423ee03ce23d351a0cf8107a1296cf` with locked offline dependencies in Windows MSVC debug mode, using the existing dependency cache. Both adapter tests passed. Executable SHA-256: `2be4eaf75f7c9a211066a16964e04b6df28331fc8f3f13b5d9cff98739bd30d5`.

The scripted replay exited 0 at tick 1440 and produced 13 [telemetry records](evidence/windows-nannou-contact-timing-20261113.jsonl) and six app-owned PNGs. Telemetry matched the contact-timing model reference line-for-line. The tick 480 capture was inspected and shows the court, score, paddles and ball. This app-owned image is not a desktop screenshot or proof of OS input.

Stderr included a Vulkan loader error for a missing Epic Online Services overlay layer manifest; the application continued using Intel UHD Graphics 630 Vulkan and completed. A requested Mailbox presentation mode also fell back to Immediate. No system settings were changed. No normal interactive test was performed in this follow-up. Native and physical input, full match, focus/resize, packaging and release qualification remain open for this revision. Prior binaries and captures remain historical.
