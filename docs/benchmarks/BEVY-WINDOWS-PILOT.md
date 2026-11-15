# Bevy Windows backend-qualified timing pilot

Fresh Bevy 0.20.1 release build completed in 22m41s; two release adapter tests passed. Default Vulkan failed with access violation exit 3221225477 before useful samples. [Rejected evidence](bevy-default-windows-rejected-20261112.json) is retained; this is not a default-backend pass.

The same binary with process-only `WGPU_BACKEND=dx12` completed [three valid runs](bevy-dx12-windows-pilot-20261112.json). No driver or system settings changed.

| Run | Samples | Median ms | p95 ms | p99 ms |
| --- | ---: | ---: | ---: | ---: |
| 1 | 1395 | 6.9674 | 8.1583 | 13.8474 |
| 2 | 1424 | 6.9412 | 7.7268 | 13.3047 |
| 3 | 1440 | 6.9477 | 7.4093 | 7.7367 |

Five-second warmup and ten-second measured interval per run; 1250x900 surface, no resize, Intel UHD 630 / DirectX 12. This measures main-world Last callback intervals, not completed GPU work or displayed FPS. Builds finished first; other user applications remained open and power/thermals were not controlled. Do not rank frameworks from these pilots.

DX12 scripted replay exited 0 after 12.750s, produced 13 state records ending finished and six screenshots. The tick-600 image was inspected: score, paddles, ball, court, and controls rendered coherently. This is app-scripted evidence, not held-key/focus/full-match native input acceptance or exhaustive visual qualification. Vulkan remains blocked; packaging and installation are separate gates.
