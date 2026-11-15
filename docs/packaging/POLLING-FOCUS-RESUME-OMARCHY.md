# raylib-rs and Nannou focus/resume input — Omarchy, 2026-11-13

In both polling adapters, a focus-loss callback paused the game but left the movement gate marked as previously playing. If the next sample also resumed play while a direction was held, the old gate allowed movement without a fresh press. Focus loss now resets the gate along with pausing and clearing accumulated time, so the first resumed sample suppresses already-held directions until release.

A targeted adapter regression failed before and passed after the change for each frontend; it checks an immediate resume with a held key, a subsequent held sample, release, and fresh press. Full `cargo test --offline --locked` suites passed: raylib-rs 13 and Nannou 12. A fresh raylib debug build passed (SHA-256 `9ca6cc7ccf8161af61c0a052371ea2ddc53e12806baeb11d4b53b521ef0ebb05`). Source Git blobs: raylib-rs `ba68eb4d216c9dfb6feafd0c193a17b07d9350ef`; Nannou `33b455e2a54c3ed4dc71aeb82b8d1a67c381a890`.

These tests invoke the focus-loss and polling boundaries directly. Actual window focus transitions, physical held keys, replay/capture, other platforms and release qualification were not tested in this batch. Prior artifacts remain unchanged.
