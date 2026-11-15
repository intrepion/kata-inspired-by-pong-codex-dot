# Raw wgpu court action prompts — macOS, 2026-11-13

The raw wgpu court now displays ENTER TO START when Ready, SPACE TO RESUME when Paused, and ENTER TO RESTART after a match. These instructions complement the state label and remain visible without the window title. They do not change game controls or physics.

A cached offline locked release build succeeded, and all 12 existing release-profile tests passed. In the native Mac window the Ready and Paused instructions were legible; the message cleared while playing. An unattended match ended CPU WINS at 3–7, with ENTER TO RESTART visible. Supported desktop automation sent Return and Space, and Return after the match restarted at 0–0. Escape closed the test app.

The executable SHA-256 is `5daf6bbdb9cf51b7b56e12a2e2acaae863664b82bb3c86d54731f7a08287fbae`; shader SHA-256 is `9ddb3b846aef8e146be984e37cd374ebe0aa02ae54e462ffba69d9f14d7baa78`. The isolated mixed-source build used shader base blob `ef14edf9005a206d392b86b45d53dd5267457f7a` from checkpoint `b4d248d34fcec01311f5806440dad8689c967484`; main.rs blob `04c081e6cdb9521ffd82028de0e5be1b51945096`, renderer.rs blob `9ed876996cfc35b432b13a8ad367ebb8a171203c`, and core lib.rs blob `8a9a1f2ecdcbfefde4d67e3f4df0d5f1d2930dd3` also matched that checkpoint. Other inputs were not all compared. Candidate and previous binary are retained in `/tmp/pong-wgpu-prompts-mac-20261113-v1`.

This is a bounded native visual and automated-input check, not physical input. The player-win path, other window sizes and platforms, and release qualification remain unverified. Existing unit tests do not validate shader pixels.
