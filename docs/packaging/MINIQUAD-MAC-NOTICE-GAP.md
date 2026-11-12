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
