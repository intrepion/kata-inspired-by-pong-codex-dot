# Benchmark runner sample validation

On Windows, an incomplete synthetic `PONG_BENCHMARK` sample with `samples: 2` and `sampled_seconds: 10` but no resize flag or timing data was incorrectly marked valid. A regression reproduced the failure before the correction. The shared runner now requires the expected schema and metric, warmup and measurement settings, explicit non-resized state, positive dimensions and sample count, finite nonnegative timing values, and an interval array whose length matches the count. Failed samples remain in the recorded run with `valid: false`; process exit remains a separate validity condition.

The adapter and public-report suites passed 23 tests on Windows; one signal-termination test was skipped on Windows. A read-only scan of 143 previously published valid samples in `docs/benchmarks` found all accepted by the new structural checks. The fixtures are not benchmark measurements. This change does not independently establish presentation timing, correct backend selection or comparable machine conditions; existing pilot limitations remain.

## Malformed output retention

A separate Windows fixture reproduced a parser exception before any run record was saved when the sample or backend line contained malformed JSON. The runner now records the parse error and malformed raw line in the private output, marks the run invalid, and continues the bounded run sequence for successful processes. A malformed backend also invalidates an otherwise complete sample. The publication allowlist excludes these raw lines and diagnostics. Adapter and public-report suites passed 26 tests; one Windows signal test was skipped. These fixtures are not measurements.
