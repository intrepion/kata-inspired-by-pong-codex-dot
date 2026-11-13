# Windows Piston explicit-pause recheck

On 2026-11-13, Piston was built from `8ce3d0fbe034b5ea278e12d1694e1301addddbd1`, including the explicit-pause held-input fix, with locked offline Windows MSVC debug dependencies in a fresh target directory. Executable SHA-256: `9147d445fd9a7f6d626499400d5690334ac72b4b63ab71b5df5f4c03dbbe5a21`. All nine frontend/action and clock tests passed. The pause regression covers W, Up, S and Down, Space and Escape, repeat suppression and renewed movement after release and fresh press. These are mapped action tests, not OS or physical-key delivery evidence.

The foreground scripted replay exited 0 with empty stderr, six app-owned PNGs and 13 [telemetry records](evidence/windows-piston-pause-20261113.jsonl) matching the contact-timing reference line-for-line. The inspected tick 480 frame shows the court, paddles and ball; it is an app-owned render-target capture, not a desktop screenshot. Scripted replay bypasses real held-key input and does not independently verify the pause fix in an OS event stream.

Native held-key input across pause, physical input, full match, focus/resize, package and release qualification remain open for this binary. Prior artifacts and the new local build and outputs remain preserved. No previous desktop-approval timeout was retried.
