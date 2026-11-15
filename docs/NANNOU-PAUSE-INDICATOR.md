# Nannou pause indicator follow-up

The earlier Omarchy contact-timing replay at `f40c9a5` recorded `Paused` at tick 480 but its application-requested frame showed no pause label. The view rendered only score and playfield; the native title state was not part of the court. The earlier capture and package remain historical.

The view now draws `PAUSED • SPACE TO RESUME` near the bottom of the court only while the game is paused. On Omarchy ARM64, source change `a9b6b95357420766b91733984e6c025d30d7a511` was built offline and locked in release mode; both existing frontend tests passed. A host-display scripted run exited 0 with all 13 telemetry records matching the revised model reference. Its app-requested tick-480 frame was inspected and showed the label, score, court, ball and paddles; tick 600 is Playing. OCR found `PAUSED` at tick 480 and not tick 600. This is application frame evidence, not physical-input or compositor validation. Binary SHA-256: `541c8dd78b2e2ab3e8898b7428ec079eb38d527345732d9b6169661ae86ad355`. New captures and telemetry are retained under `/tmp/pong-nannou-pause-fix-20261113` and `/tmp/pong-nannou-pause-fix-telemetry-20261113.jsonl`; no historical artifacts were overwritten.

Actual keyboard pause/resume, focus/resize, full-match behavior, clean-machine installation and other platforms remain unverified for this revision.
