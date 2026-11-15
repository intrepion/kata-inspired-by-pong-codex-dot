# Miniquad Linux notice-bundle pilot

2026-11-12. This prepares actual notice text, not just SPDX declarations. It is not legal clearance or a public release.

- Offline locked Linux ARM64 Cargo metadata: **17 packages**.
- **33 license/notice files** collected without altering their text; no package missing a notice file.
- Four first-party packages use the repository root MIT text, explicitly marked in the index. External packages never receive that fallback.
- All discovered alternative license texts are retained; no license-choice decision is inferred.
- [Bundled texts](miniquad-linux-NOTICES-20261112.txt) and [per-source SHA-256 index](miniquad-linux-notice-index-20261112.json) contain no local source paths.
- Bundle SHA-256: `615383437f67456e2483e0842e8effac95e23685de9689a0a76831a88883ea46`.

## Archive verification

A new local Miniquad archive includes these texts and index. Its binary is the same previously qualified release binary. All six manifest-listed files passed SHA-256 checks after fresh extraction. Relocated app-owned replay exited normally with 13 states through tick 1440, finished=true, and six captures. Tick 600 inspected. Exact seven-file removal preserved the neighboring sentinel and original build. [Archive evidence](miniquad-linux-notice-package-20261112.json).

The Linux packager accepts an optional `{text,index}` fifth argument through its module API. It rejects a mismatched text digest or nonempty missing-notice list before creating output. Existing no-notice packages remain unchanged. The command-line interface still creates the original no-notice pilot; it does not silently assume a bundle.

## Preparation tooling

`scripts/packaging/notice-bundle.mjs` exports `buildNoticeBundle(metadata, repoRoot)`. Feed resolved Cargo JSON from:

```sh
cargo metadata --offline --locked --format-version 1 \
  --manifest-path experiments/miniquad/Cargo.toml \
  --filter-platform aarch64-unknown-linux-gnu
```

The helper scans conventional license/copying/notice/copyright filenames, including nested directories, and explicit Cargo license-file paths. It rejects escaping paths, symlink notice sources, oversized files, binary/non-UTF-8 notice text. Tests cover exact-byte hashing, nested attribution, constrained first-party fallback, explicit missing external notices, symlink/path rejection and archive inclusion. All **28 shared benchmark/packaging tests pass** on Linux.

## Remaining limitations

A resolved graph includes build/optional dependencies and is not exact linked-binary attribution. Conventional filename scanning does not prove that every embedded source-comment attribution or unlabelled bundled native component is covered. Directory symlinks are not traversed. System libraries, GPU drivers and distribution-specific obligations are not inventoried. These require review before redistribution. No signing, installer, clean-machine or cross-distro release qualification is claimed.
