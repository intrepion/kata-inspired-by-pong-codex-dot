# Fyrox Windows timing pilot

Fresh Fyrox 1.0.1 release build completed in 15m27s. Two release adapter tests passed. [Sanitized timing evidence](fyrox-windows-pilot-20261112.json).

| Run | Samples | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1440 | 6.9393 | 7.2705 | 7.5330 |
| 2 | 1440 | 6.9463 | 7.2190 | 7.6200 |
| 3 | 1439 | 6.9459 | 7.3448 | 7.9202 |

Three valid normal exits with five-second warmup and ten-second measurement, no backend override or resize. This samples the native before_rendering callback, not completed GPU work or displayed FPS. Existing engine pacing is unchanged. Builds finished first; user applications remained open, and power/thermal conditions were not controlled. This is not a framework ranking.

A separate scripted replay exited 0 in 12.550s, produced thirteen state records through tick1440/finished=true, and six PNG captures. Inspected tick600: court, ball, both paddles, score and controls are present; header text remains small relative to the court. Scripted capture is not OS keyboard/focus/full-match qualification. Packaging/install and clean-machine tests remain separate. Shared benchmark tests: 21 passes, one POSIX-only skip on Windows.
