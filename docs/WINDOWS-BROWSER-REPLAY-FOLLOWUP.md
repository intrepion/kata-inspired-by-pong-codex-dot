# Windows browser replay completion follow-up

2026-11-12, source `0cbe38f`, Chrome on Windows. Tested only through local HTTP
servers: original TypeScript on port5249 and freshly rebuilt Rust WASM on5251.
The older server on5188 served an older frontend and was not used for this result.

- Fresh TypeScript build and all15 Node tests passed.
- Fresh release WASM build, two compiled-WASM ABI tests and seven Rust
  scenario/WASM tests passed.
- Both browser scripted runs reached their final download-link creation path.
  Original displayed "Download original TypeScript demo evidence"; raw WASM
  displayed "Download scripted canvas captures + telemetry". Those paths run
  only after the bounded1440-step replay completes. Controls were disabled.
- Original reset returned scores to0:0. Raw WASM final court, two paddles, ball
  and0:0 score were visually inspected and looked correct.
- Original normal Start and Pause were rechecked; Paused overlay and Resume
  controls appeared. Raw normal Start reached Playing.

This proves bounded browser execution and a final rendered raw-WASM court, not
integrity of all intermediate captures/telemetry. The bundle links use blob URLs;
no bundle download was attempted because the prior browser download restriction
allows only HTTP/HTTPS. No workaround or hidden-state extraction was used.

Playback advanced much slower than12 wall-clock seconds; background throttling
is a possibility, not established causality or a benchmark result. Creating a
second test tab did not establish a true focus transition, so focus-loss remains
unverified. A390x844 viewport override was requested, but observed DOM dimensions
remained1282x734; it was reset, and no responsive-breakpoint pass is claimed.
Test tabs were closed. No full-match/physical-input/mobile/package pass inferred.
