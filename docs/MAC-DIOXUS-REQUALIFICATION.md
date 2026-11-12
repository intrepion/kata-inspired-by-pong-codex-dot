# Mac Dioxus current-source requalification

Tested 2026-11-12 06:51–06:54 UTC from clean source checkpoint `4088089`.

## Passed

- Locked shared web controller suite: **4/4** tests (fixed steps, repeat protection, blur input clearing, bounded scenario exports).
- Locked Dioxus native development build using existing dependency cache: passed. Cache-dependent 3.38-second build is **not a benchmark**. Dependency `block 0.1.6` reports a future-incompatibility warning.
- App-owned scripted replay: normal native application completed with exit 0, **13 telemetry records**, finished tick **1440**, paused state at tick480, and **six SVG scene-source exports**. These are not screenshots or proof every frame was displayed.
- Fresh binary installed into the existing local test bundle. Supported Computer Use visibly verified start, held W movement upward, held S downward, release stopping the paddle, Space pause/resume, real scoring, genuine full match ending **2:7 CPU wins**, and Enter restart to **0:0**. No score injection or scripted input was used for this interactive match.
- Test application exited after checks. No unrelated applications or build caches were removed.

## Provenance

- Binary SHA-256: `58722f901f45ad7ae937e2578ce0207706a02a97cf95cff901caa11d4c4777d8`
- Replay telemetry SHA-256: `17cbe3c7299c64e6d12bbfcb292cdb7ea9a8e9821c92fa512c027df5285242bf`
- Evidence retained in `docs/evidence/mac-dioxus-4088089/`.
- Correct export command: `pong-dioxus --scripted-test --scripted-scenes NEW_DIRECTORY`. An initial mistaken `--scripted-captures` invocation was correctly rejected before replay, reinforcing that SVG exports must not be called screenshots.

## Remaining limits

This closes Dioxus native source freshness and full-match/restart gaps only, not all Mac qualification. OS focus-loss, minimize/resume, DPI/resize matrix, accessibility, mobile exports and installation remain separate.

Yew focus-loss was retried with supported app-targeted Cmd-Tab and a separate application Raise action. These did **not establish actual OS application deactivation**, so no focus-loss pass or definitive gameplay bug is claimed. Source still attaches an element-level blur callback rather than a window-level lifecycle listener. The shared unit test verifies input clearing when pause is called, not whether the OS delivers that event. Yew was closed afterward.
