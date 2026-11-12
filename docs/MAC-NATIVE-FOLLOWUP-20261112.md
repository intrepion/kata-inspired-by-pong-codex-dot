# Mac native follow-up: 2026-11-12

Worker-observed native-window checks, reported at this checkpoint. These are bounded gameplay checks, not full acceptance or release qualification.

| Client | Binary/source provenance | Build | Native checks |
| --- | --- | --- | --- |
| Yew native Tauri | Cached binary; checkout was `6ceb22a`, exact binary source revision unverified | Existing build, not rebuilt | Start, held W/S movement, Space pause/resume, CPU score 0:1, button restart to 0:0 passed |
| Leptos native Tauri | Cached binary; checkout was `6ceb22a`, exact binary source revision unverified | Existing build, not rebuilt | Start, W/S movement, Space pause/resume, CPU score 0:1, restart to 0:0 passed |
| Dioxus native desktop | Fresh build from clean `6ceb22a`, reusing dependency cache | Passed, 85 seconds with two jobs | Enter start, W/S movement, pause/resume, CPU score 0:1, restart to 0:0 passed |
| Bevy | Fresh build from `55d097573bdfb9a2944b3b0d1260871dbc2350c2`, reusing dependency cache | Passed, 219 seconds with two jobs | Mesh2d court/paddles/center-line render correctly; Enter start, W/S movement, Space pause/resume, CPU score 0:1, R reset to 0:0 passed |

Build durations above are observations, **not controlled performance benchmarks**.

## Binary SHA-256

- Yew: `d5b4bf295727faede5239c3b22e073ca9ea4f67f67364db88d716ba20238962e`
- Leptos: `2fcdf753230e2d5fadc6b5a8d747b7fa5ba8e02a48a3333d89f4a05e60c63169`
- Dioxus: `c399b2d4a14ddbe3fcbc350625d6382f8244c24f07f3d152ff2de7037c163462`
- Bevy: `fd010650755d8d877988093682e1ba2c0f9389b0cfb0d9df8315f44b1696cf94`

The earlier interrupted app-access call ended user-aborted; a fresh Leptos retry subsequently succeeded. No permission prompt remains from that completed phase. All four apps were closed after testing, and a process check confirmed none remained running.

## Limits

Focus-loss, resize, complete end-of-match behavior, performance, packaging and installation were not covered by these checks. Cached Yew/Leptos binaries are not evidence of a rebuild from the latest source. These reports do not imply that unrelated newer source changes were requalified.
