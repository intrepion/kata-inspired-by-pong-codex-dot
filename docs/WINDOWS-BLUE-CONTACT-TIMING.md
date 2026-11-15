# Windows Blue Engine contact-timing recheck

On 2026-11-13, Blue Engine was built from `13477617182bbb3ee2fd75b102499b64474e8fe3` with locked offline dependencies in Windows MSVC debug mode and fresh separate target directories. Both windowed frontend tests passed. Windowed executable SHA-256: `458fe3668b92fd961747b773a6c157da6e351eb36a0e1657f390e348ce823ff7`. The separate `--no-default-features --features capture-headless` executable SHA-256: `4fd4665a938a079c475c7ec1be672b7fed6b539e97fbbd02f10756dc37cfbb93`.

The foreground windowed scripted run exited 0 and produced 13 [telemetry records](evidence/windows-blue-contact-timing-20261113.jsonl) matching the contact-timing model reference line-for-line through finished tick 1440. Windowed capture is unsupported, and the window was not visually inspected. The separate headless renderer scripted run also exited 0, produced 13 exact-match records and six app-owned PNGs. Its inspected tick 480 frame shows the court, paddles and ball. The headless image is not a window or desktop screenshot and does not demonstrate input.

Both runs logged an unavailable Vulkan validation layer and a Vulkan loader error for a missing Epic Online Services overlay manifest; both continued to completion. Normal native interaction, physical input, full match, focus/resize, packaging and release qualification remain open for this revision. Previous binaries and captures remain historical.

## Held-input pause recheck

At checkpoint `85d423dbed3050261c3b996118a8fc439e27562b`, including fix `582ab29`, the offline locked Windows MSVC debug test and windowed build succeeded. All four tests passed: two benchmark argument tests and two movement-gate tests covering suppression through resume, release, fresh press, aliases and opposing keys. Tested source blob `613b25e8d280bed7956741850e547e10dfc2f4a5`; executable SHA-256 `1eca7e983a6f47c7b23f8ab165a406330bf2fd721ae3f2f36c8e04e96e377a20`.

The separate windowed scripted run exited 0 and all 13 telemetry lines matched the contact-timing model reference through finished tick 1440. The run logged an unavailable Vulkan validation layer and loader diagnostics; it still completed. The new binary, prior binary and raw logs are preserved in workspace `blue-held-pause-windows-20261113`. These are gate/model and scripted replay checks, not proof of native keyboard, focus events, physical input, visual presentation, packaging or release qualification. Previous captures remain historical; no fresh headless capture was run.
