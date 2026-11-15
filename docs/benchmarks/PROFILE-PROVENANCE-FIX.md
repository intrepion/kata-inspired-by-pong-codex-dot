# Benchmark profile provenance

The pilot runner previously wrote `profile: release` for every supplied executable, including its Node fixture executable, without checking how it was built. The new regression failed before the fix. This could mislabel debug or otherwise unknown results even when interval statistics were internally consistent.

The runner now records `unverified` by default. `--profile release` or `--profile debug` records the caller's explicit assertion; invalid and duplicate options are rejected. The public allowlist accepts `unverified` and both local and public reports explicitly state that profile is caller-supplied or unverified and that the checkout revision does not prove the binary's source. This does not independently verify a binary's profile, compiler or revision. Prior results were not modified; review their individual build evidence rather than assuming the old automatic field was verified.

On Omarchy ARM64, Node `v26.12.1`, 25 runner/adapter tests and six public-report tests passed. The fixture process report defaults to `unverified`, both explicit profile choices are accepted, malformed/duplicate options fail, and the privacy-filtered report retains the limitation. No timing sample or cross-engine comparison was run. Tested against checkpoint `0003f6d7b7c893b5d302499986a922f492e627ce`.
