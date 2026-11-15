# Benchmark public-report validity guard

The public exporter previously trusted a private run's `valid: true` flag even when its exit status or measurement contradicted the runner's validation rules. A synthetic valid-labeled fixture with a nonzero exit reproduced this: export succeeded before the fix. Other invalid examples include too little sampled time, resize and statistics inconsistent with the interval array.

The exporter now checks successful exit, absence of reported signal, execution or sample/backend parsing error, and the runner's existing `validSample` rules before publishing any run marked valid. A contradictory run fails export instead of being silently represented as valid. Runs already marked invalid retain the previous bounded, privacy-filtered representation. This guard checks internal consistency of supplied data; it cannot establish that the source measurements or their provenance are truthful.

On Omarchy ARM64, Node `v26.12.1`, five public-export tests and 24 adapter/runner tests passed, including failed exit, signal, error, parse failure, short duration, resize and inconsistent percentile regressions. The public-report fixture was corrected to contain a full ten seconds of synthetic intervals; it is not a real measurement. An existing private Miniquad pilot exported to a new local file with all three runs still valid, while the private original remained intact. No new timing sample or cross-engine comparison is claimed. The sanitized output remains `/tmp/pong-miniquad-public-revalidated-v2-20261113.json`.

Tested against checkpoint `66648a09681ac41659b1c30780b5210a2def6d09`; exporter Git blob `edc3bbfa2568f56012a0a0885b2108c322cf792d`.
