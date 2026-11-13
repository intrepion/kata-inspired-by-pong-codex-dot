# Windows Nannou held-input pause recheck

On 2026-11-13, Nannou was built from `6825930b8c486b39e0926a19ec8016eca9be6a5e`, including the explicit-pause held-input fix, with locked offline Windows MSVC debug dependencies in a fresh target directory. Executable SHA-256: `ab77b94d1df672b0fbe79ea1f24b1ab33cc58489ad355a293070a0cffc14cb81`. All five frontend/action tests passed, including suppression of held movement until release, aliases and opposing directions. These test the polled-input model, not OS or physical-key delivery.

The foreground scripted replay exited 0 with six app-owned PNGs and 13 [telemetry records](evidence/windows-nannou-held-pause-20261113.jsonl) matching the contact-timing reference line-for-line. The inspected tick 480 frame shows the paused label, score, paddles and ball. This app-owned image is not a desktop screenshot; scripted replay bypasses real held-key input.

The Vulkan loader again logged a missing Epic Online Services overlay manifest, and the requested Mailbox present mode fell back to Immediate; the run continued on Intel UHD Graphics 630. The previous native smoke check predates this fix. Native held-key input across pause, physical input, focus/resize, full match, package and release qualification remain open. Previous binaries and captures and the new local outputs remain preserved. No desktop approval was retried.
