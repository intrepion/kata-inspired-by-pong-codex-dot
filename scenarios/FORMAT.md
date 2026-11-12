# Portable Pong scenario format, schema 1

`smoke-v1.txt` is the single declarative timeline consumed by Rust and the original TypeScript adapter. It is a UTF-8, whitespace-delimited format, deliberately trivial to port to Swift/Kotlin without copying events into another language. Blank lines and `#` comments are ignored. Unknown lines, duplicate scalar fields, invalid numbers/actions and unsorted/overlapping ranges fail validation.

Required scalar rows: `schema 1`, `id NAME` (ASCII alphanumeric/hyphen), `seed U32`, `step_hz POSITIVE_U32`, `end_tick POSITIVE_U32`, `report_every POSITIVE_U32`.

- `event TICK start|pause|toggle_pause`: apply before stepping zero-based TICK. Strictly increasing, less than end_tick. Start means start/reset an entire match, not merely serve.
- `input FROM TO -1|0|1`: hold paddle input over half-open [FROM,TO). Ranges sorted, non-overlapping, end no later than end_tick. Outside them input is zero.
- `capture TICK`: after completing this tick, expose the scene before advancing farther. Positive, strictly increasing, no later than end_tick.
- Step always advances the scenario clock, including while paused. Physics receives `1/step_hz`; paused physics must not advance. Stop at end_tick. Report initial tick0 and after each report_every steps.

Rust's std-only build script reads this file, validates it and emits constants/tables into Cargo OUT_DIR. No new dependency/lockfile or checked-in generated timeline is required. All existing Rust smoke-v1 state records remain byte-identical in a regression test.

Original TypeScript parses the same file at runtime only for `?scripted-test`. Its deterministic demo-only 32-bit LCG is **not** Rust's u64 RNG and its number arithmetic is f64 rather than Rust f32. Shared actions/timing/seed labels do not imply identical physics trajectories. Tests assert input boundaries, pause/resume/reset semantics, all checkpoint ticks, deterministic reruns and bounded completion. Normal play retains Math.random and its existing frame loop. Future Swift/Kotlin adapters should consume this file directly and disclose numerical/RNG differences rather than copying a Rust expected state and calling it observed.
