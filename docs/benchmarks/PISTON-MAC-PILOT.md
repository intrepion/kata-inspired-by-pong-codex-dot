# Piston Mac callback-timing pilot

2026-11-12 UTC. Offline locked release build from `ebdc9da` passed in 77 seconds; 2 adapter tests and 12 shared runner/sanitizer tests passed. Harness revision `7ac2b52` adds only README activity. Three valid repeats each used 5 seconds warm-up and 10 seconds measured. [Sanitized timing data](piston-mac-pilot-20261112.json) preserves binary hash, exact intervals and dimensions; private host/environment diagnostics remain local.

| Run | Intervals | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 243 | 33.582 | 50.123 | 50.230 |
| 2 | 248 | 33.517 | 50.136 | 50.251 |
| 3 | 243 | 33.574 | 50.120 | 50.216 |

Reported draw surface 2000×1360; no resize, all exits 0. Pinned piston_window uses wgpu, not the historical OpenGL backend; actual runtime GPU/backend/driver were not queried. Existing 120 UPS update scheduling and render configuration remain unchanged. Measurement is after draw_2d on render events, not update events. Builds/tests completed first and a worker quiet window was reserved. Other user applications remained open; power, thermals and whole-machine idleness remain unverified.

The approximately 33.5 ms median and 50.1 ms p95 are slower callback intervals than several prior Mac pilots, but are **not displayed FPS or GPU execution time**. Different pacing, dimensions and callback boundaries prevent an engine ranking. A controlled pacing/dimensions investigation is needed before attributing this result to Piston or the hardware. No new manual-input, visual-quality or installer qualification is claimed. All benchmark processes exited.

## Replay slowdown found (not a normal-deadline pass)

The same binary's initial 30-second bounded scripted replay **timed out**, with 7 records and final tick 720. A separately labelled 75-second diagnostic completed in 59.266 seconds, exit 0, 13 records and final tick 1440. A nominal 12-second scenario taking about 59 seconds exposes a simulation timing problem, not a successful normal-duration acceptance test. Both original logs remain local.

Source inspection found the adapter advances using update-event `u.dt`, while pistoncore-event_loop 1.0.0 defaults `ups_reset` to 2 and skips overdue updates. With 120 requested UPS and slow render callbacks this is a plausible cause of lost simulated time; no isolated causal fix has yet been tested. A wall-clock/fixed-step regression fix should be separately validated and rebenchmarked. Existing raw timing results are preserved rather than relabelled as post-fix evidence.

## Wall-clock fix and Mac replay requalification

A red regression reproduced the mechanism: twelve wall-clock seconds delivered in 50 ms callbacks advanced only 240 simulated ticks using update-event dt instead of approximately 1440. The fix measures monotonic elapsed time, preserves 120 Hz fixed steps and the 100 ms catch-up cap, and resets backlog on start/pause/resume/restart/focus loss. Seven Piston unit tests, eight shared scenario integration tests and thirteen shared Node tests passed. Source originated as commit `4903aa0`; runtime binary SHA-256 `9ea7bd078b870d7bf66fd19242f74561c9fb0b1aaddcf7ecf3bd4e15ddbb8f2f` was built before one additional test-only change.

A first worker's shell session could not attach to the Mac GUI (LaunchServices error, no scenario telemetry), so those timed-out attempts are not gameplay failures. Retrying the identical binary in the established Mac execution session succeeded without changing permissions: plain replay completed in **12.723 seconds**, capture replay in **12.380 seconds**, both exit 0 with 13 state records, tick 1440 and finished=true. Capture produced all six PNGs; tick 600 was inspected and court/paddles/ball/score pip were visible. This is a bounded replay and framebuffer check, not a new manual-input pass.

The original 30-second timeout, 59.266-second diagnostic and pre-fix timing data above remain unchanged. Post-fix timing requires separately named evidence because simulation progress now tracks wall time correctly.
