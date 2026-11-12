# Blue Engine Windows callback-timing pilot

Fresh native release build completed in 4m16s; two release adapter tests passed. [Sanitized timing](blue-engine-windows-pilot-20261112.json).

| Run | Samples | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1440 | 6.9419 | 7.3417 | 7.8095 |
| 2 | 1440 | 6.9448 | 7.4596 | 7.8507 |
| 3 | 1440 | 6.9441 | 7.3727 | 7.6269 |

Three valid runs, 5s warmup/10s measured, native 1000x680 surface, no resize, normal exit 0 with the window-lifetime fix present. Existing pacing/default backend retained. CPU callback/upload boundary, not completed GPU or displayed FPS. Builds finished first, user apps open, power/thermals unknown.

Native capture attempt intentionally rejected with exit101: `Blue Engine capture requires --no-default-features --features capture-headless`. It produced no native screenshot pass. Plain native scripted replay then exited 0 in 12.376s with 13 state reports ending finished. Replay uses the existing forced-exit path, so only the three timing runs qualify normal engine teardown. No headless substitute was used to claim native visuals. Native-input/visual capture gates remain open. All processes exited.
