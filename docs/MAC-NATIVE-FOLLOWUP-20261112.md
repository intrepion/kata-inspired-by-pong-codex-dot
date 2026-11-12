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

## Fresh-source follow-up at b92b7df

Mac worker subsequently validated clean checkpoint `b92b7dfacdf24113f5431ad749d18cda5750069f`:

- Swift: all 11 tests passed (nine core/physics and two scenario replay/parser); iOS simulator Debug build succeeded for arm64 and x86_64. This validates the replay core, **not app replay**: `PongApp.swift` does not yet wire `ScenarioReplay`. No new simulator UI or capture pass.
- Yew: frontend build passed (65 seconds), native build passed (9.05 seconds).
- Leptos: frontend build passed (110 seconds), native build passed (3.51 seconds).
- Both native wrappers completed their own scripted demo with exit 0, 13 state records through tick 1440, and six SVG scene-source exports each. The native bridge validates states against the shared Rust scenario.
- SVG exports are **not screenshots**. Fresh manual UI interaction was not retested. Earlier cached-binary interaction evidence remains separate.
- Sources remained clean; the assigned batch completed and apps were closed. Build durations are cache-dependent observations, not benchmarks.

Fresh binary SHA-256:
- Yew: `bd4323bdc07be4ffcda605a2e44aaf0a39ea8186906bfc3d61f82c467038eddb`
- Leptos: `3b03547d82b7986116d005ff581b40e249f4cedeb4bb61e5a424221218ac76c3`
- Both telemetry files: `7d8c782151bb25660566c848023793aacd0da2816a0a6ea35de407fb2f34f5f1`
