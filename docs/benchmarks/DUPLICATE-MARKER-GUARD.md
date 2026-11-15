# Benchmark duplicate-marker guard

At checkpoint `c5fe4269012f2e6008ae47dc86bf7c8d06d4f69d`, the runner selected the first `PONG_BENCHMARK` and optional `PONG_BENCHMARK_BACKEND` line even when a process emitted multiple records. A synthetic process that printed two otherwise valid sample markers reproduced acceptance before the fix. A second fixture covers duplicated backend records.

The runner now counts each marker and records a parse error for multiple samples or multiple backend records; those runs cannot be marked valid. The first record is retained for diagnosis and normal single-record behavior remains unchanged. This does not prove a measurement or its provenance, detect all malformed producers, or revise historical reports.

On Windows, Node v24.20.0 ran the focused regression against a preserved copy of the old runner and observed the expected failure. The updated adapter/runner and public-report suites completed with 31 passing tests and one existing signal-termination test skipped on Windows. Fixture processes are synthetic and are not timing measurements; no benchmark ranking, physical input, accessibility or release qualification is implied.
