# bracket-lib Windows callback-timing pilot

Fresh locked release build completed in 1m07s; eight release adapter tests passed. [Sanitized evidence](bracket-lib-windows-pilot-20261112.json).

| Run | Samples | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 625 | 16.0027 | 16.3895 | 16.5628 |
| 2 | 625 | 15.9981 | 16.3561 | 16.5028 |
| 3 | 625 | 16.0025 | 16.3390 | 16.5210 |

Three valid runs, 5s warmup/10s sample, no resize, exit 0, reported 910x610. Graphical OpenGL tile renderer, not terminal emulation; existing cap retained. Callback intervals are not displayed FPS/GPU time. Builds finished first, no concurrent builds; user apps open, power/thermals unknown.

Replay/capture finished in 12.351s, exit 0/no stderr, 13 state records ending finished and six PNGs. Tick600 inspected: readable header/controls, score, paddles and ball; center divider remains low contrast. This is not exhaustive visual QA or OS-input/focus qualification. All processes exited.
