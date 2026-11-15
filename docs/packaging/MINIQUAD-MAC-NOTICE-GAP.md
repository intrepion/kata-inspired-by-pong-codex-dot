# Miniquad macOS notice preparation gap

2026-11-12, ARM64 Mac target metadata, offline/locked Cargo resolution at
`730881f`. Shared notice collector inspected 19 resolved packages and found 34
actual license/notice texts. The [sanitized index](miniquad-mac-notice-index-20261112.json)
contains only package-relative source paths and hashes.

Unlike the Linux target graph, this graph contains `malloc_buf 0.0.6`. Its package
metadata declares MIT but its installed crate contains no detected license file.
No license text was invented from that declaration. The candidate notice bundle
remains private/incomplete; no new notice-qualified distribution archive was made.
The existing local runtime package pilot is unaffected.

Miniquad's optional notice-text/index packaging inputs now reject an incomplete
inventory or hash mismatch **before creating the output directory**. The real
incomplete inventory was rejected as expected with no output. Three packaging
fixture tests cover missing notices, hash mismatch and inclusion of matching text;
three shared collector tests also pass (six total). Synthetic fixture archives
are test-only and were removed. Existing two-argument local-pilot packaging still
works without claiming notice clearance.

Next requirement: obtain and verify the exact upstream license notice applicable
to the pinned dependency, retain provenance, then rerun notice-bearing packaging.
Even a complete detected-file index would not establish legal clearance, nested
unlabelled-source attribution, platform-system-library compliance, signing,
notarization or clean-machine qualification. No rebuild, UI automation or security
setting changes were needed for this batch.

## Authoritative upstream investigation

The repository URL was read from the installed crate's own Cargo metadata:
<https://github.com/SSheldon/malloc_buf>.

- Pinned upstream tag `0.0.6` resolves to
  [`a7811e5f4c6f9685dd57ad9bfe34e9bf6b0ba4f9`](https://github.com/SSheldon/malloc_buf/tree/a7811e5f4c6f9685dd57ad9bfe34e9bf6b0ba4f9), dated 2015-11-06.
  Its tracked tree contains `.gitignore`, `Cargo.toml` and `src/lib.rs`, but no
  license file. Cargo metadata declares MIT.
- Installed `src/lib.rs` is byte-identical to that pinned tag. SHA-256:
  `1130dacbe045a20c1ea1e65e00976f18eb7513da492b07d538e7b1b5c64b8842`.
- The canonical upstream later added an MIT license in
  [`d9a3e539642bd90e07df458d226b19cdfa606863`](https://github.com/SSheldon/malloc_buf/commit/d9a3e539642bd90e07df458d226b19cdfa606863), dated 2020-06-29.
  That commit's explanation says it adds MIT because crates.io indicated that
  intention. The license has a 2020 copyright line. It follows the 1.0.0 release
  and source changes, rather than being a file in the pinned 0.0.6 release.
  Exact later license SHA-256:
  `aced7b57e474a63edb5a724954222bf7659c01ceef0e648a03af799582991db7`.

This verifies where a real upstream license text exists, but does not establish
an explicit upstream statement applying that later notice to the older pinned
release. The build dependency was not upgraded, Cargo cache was not modified,
and the later text was not silently relabeled as a 0.0.6 notice. The packaging
gap remains open pending a documented attribution decision or verified upstream
clarification; no external outreach was sent.
