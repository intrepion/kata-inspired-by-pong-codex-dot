# Windows ggez explicit-pause recheck

On 2026-11-13, ggez was built from `04817c13de2e7f6ad2fcf7206b6930908bcca603`, including the explicit-pause held-input fix, with locked offline Windows MSVC debug dependencies in a fresh target directory. Executable SHA-256: `0371e950a3cdf5dee541c876a3dc68bf31b2439aa6e40b286dbec57b4b46da9b`. All four frontend/action tests passed. The pause regression covers W, Up, S and Down, Space and Escape, repeat suppression and fresh movement after resume. These are mapped action tests, not OS or physical-key delivery evidence.

The foreground scripted replay exited 0 with empty stderr, six app-owned PNGs and 13 [telemetry records](evidence/windows-ggez-pause-20261113.jsonl) matching the contact-timing reference line-for-line. The inspected tick 480 frame shows the paused label, score, court, paddles and ball. This is an app-owned render capture, not a desktop screenshot. Scripted replay bypasses real held-key input and does not independently verify the pause fix in an OS event stream.

The previous Windows native smoke check predates this fix. Native held-key input across pause, physical input, full match, focus/resize, package and release qualification remain open for this binary. Prior artifacts and the new local build and outputs remain preserved. No previous desktop-approval timeout was retried.
