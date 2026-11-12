# raylib-rs Windows callback-timing pilot

Fresh release build passed after correcting bindgen resource lookup. Initial build could load libclang but failed `stdbool.h` lookup. The installed LLVM23 resource header existed. Setting command-scoped `LIBCLANG_PATH` to LLVM/bin and `BINDGEN_EXTRA_CLANG_ARGS` to `-resource-dir="C:/Program Files/LLVM/lib/clang/23"` resolved it. No installation or persistent settings changes. Three adapter tests and 15 shared runner/sanitizer tests passed.

[Sanitized timing evidence](raylib-windows-pilot-20261112.json): three valid runs, 5s warmup/10s sampled, all exit 0, no resize, 900x500 surface.

| Run | Samples | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1140 | 8.7586 | 9.3453 | 9.5646 |
| 2 | 1139 | 8.7713 | 9.4377 | 9.6759 |
| 3 | 1142 | 8.7032 | 9.4089 | 9.5746 |

Existing target120fps retained, sampled after EndDrawing including its pacing wait. These are callback intervals, not GPU time/displayed FPS or engine ranking. Builds were finished; a small Git documentation publish/merge occurred during the pilot, with user apps open. This was not an isolated performance lab. GPU identity, power and thermals unmeasured.

Replay/capture exited 0 without stderr in 12.459s, 13 state records and six PNG frames. Tick600 inspected: paddles, ball, score and court render; footer text is comparatively faint and overlaps the paddle region near the bottom, so this is not an exhaustive visual-quality pass. Native input not requalified. All test processes exited.
