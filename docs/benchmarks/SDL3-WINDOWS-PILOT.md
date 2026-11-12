# SDL3 Windows callback-timing pilot

Bundled static locked release build from 0c1ac6f completed in 7m51s. Two release adapter tests and 17 shared runner/sanitizer tests passed. [Sanitized evidence](sdl3-windows-pilot-20261112.json).

| Run | Samples | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1164 | 8.5734 | 8.9658 | 9.0904 |
| 2 | 1163 | 8.5839 | 9.0010 | 9.1113 |
| 3 | 1163 | 8.5840 | 8.9867 | 9.1134 |

All three repeats valid: 5s warmup/10s measured, exit 0, no resize, actual 900x500. Runtime reports video driver `windows`, renderer `direct3d11` for all runs. Existing 8ms sleep retained; sampling after canvas.present returns. Actual vsync/GPU execution not measured. Builds finished first; other user apps open, power/thermals unknown. These are callback intervals, not engine rankings or displayed FPS.

Replay/capture finished in 12.236s, exit 0/no stderr, 13 state records and six PNGs. Tick600 app-owned renderer readback inspected: both paddles, ball, divider and score visible. Native keyboard/focus/resize and clean-machine static dependency qualification remain separate. All test processes exited.
