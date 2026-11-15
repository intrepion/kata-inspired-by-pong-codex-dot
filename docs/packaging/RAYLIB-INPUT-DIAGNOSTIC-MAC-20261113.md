# Raylib Mac input diagnostic and native match (2026-11-13)

The uninstrumented release executable from `RAYLIB-COURT-STATES-MAC-20261113.md` has SHA-256 `a326745ac4828221985e0477efbbdf3686ab20addaaa2b9ddadea19dc1065e6c`. Earlier brief automated Return and Space presses did not advance its Ready state.

Two separate temporary diagnostic builds logged raylib focus and key polling without changing gameplay. The first logged `focused=true` but no Return or Space press after the brief automated attempts; it did not log a heartbeat. The second logged a periodic frame counter as well as focus and key presses. With 500 ms desktop-automation presses, it recorded `focused=true` and both Return and Space on advancing frames, and displayed Paused. The second executable SHA-256 was `2b9c8154f8046fe93a0b666ffe7441b36afafc2ba65717598d84a550349fe01c`; its log SHA-256 is `f4c17618071813a772e6fbb7f5f7f286b2aa70b0ea9be304fa62b04bd65346d3`, retained at `/tmp/pong-raylib-focus-heartbeat-20261113-v1.log`. Original source was restored to SHA-256 `ed303fb15e6e6ae03eea278c05d116763e0aed526c676a776815e5dfcf8ce021`; diagnostic bundles were retained.

The uninstrumented binary was then reopened: 500 ms Return and Space presses reached Paused, and another Space resumed. The unattended native match reached 3–7 and displayed “CPU WINS - ENTER OR R TO RESTART”; Return followed by Space showed Paused at 0–0. The Ready, Paused and CPU winner prompts were visually legible at the observed window size.

This distinguishes a successful longer automated input sequence from the initial failed attempt, but does not prove why the brief attempts failed or qualify physical keyboard input. Player win, other window sizes, and release/license readiness remain open.
