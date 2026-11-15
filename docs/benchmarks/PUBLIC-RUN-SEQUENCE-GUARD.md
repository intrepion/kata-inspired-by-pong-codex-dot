# Benchmark public run-sequence guard

The sanitizer previously accepted an empty list, repeated iteration numbers, gaps, reordering or more than three runs even though the runner emits an ordered prefix of runs 1 through 3. A focused regression failed against the prior exporter on an empty list. The exporter now requires a nonempty ordered prefix; one- or two-run interrupted/failed reports remain representable and are not promoted to complete runs. Existing validity and privacy checks remain in force.

On Windows at checkpoint `66e81ad0ed4b620537aa57e97e34c65f5b06b579`, the adapter/runner and public-report suites completed with 32 passes and one existing Windows-skipped signal test. The previous private three-run Blue Engine release pilot was sanitized again to a new local file; all three runs remained valid and the output SHA-256 `bf63ce7401b941b14599a791a0a3af8f85e3058bc995eb72f25c05e02fac8799` matched the already published sanitized report. Historical data was not altered.

This guards the representation of the runner's three-run protocol; it does not verify process identity, measurement truth, provenance, physical input or accessibility. No new performance measurement or ranking was made.
