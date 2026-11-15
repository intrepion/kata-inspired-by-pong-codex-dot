# SDL3 packaging license-inventory checkpoint

2026-11-12, source base `839f81c`. Offline locked Cargo metadata for the Linux ARM64 target resolves **34 package records**. [Sanitized generated inventory](sdl3-license-metadata-20261112.json) preserves names, versions and declared license expressions, not local paths, registry URLs, package IDs or machine metadata.

## Findings

- 32 records declare a license expression; two local manifests (`pong-sdl3`, `pong-low-level-common`) omit one. The repository has a root MIT license, but this tool reports manifest omissions rather than inventing declarations.
- Declared expressions include MIT, Apache-2.0, Zlib and 0BSD alternatives; this is metadata, not a legal compatibility conclusion.
- The resolved graph contains SDL image/mixer/TTF source and sys crates as well as SDL itself. Resolution is NOT proof every package is linked into this Pong binary. Cargo build/optional dependencies and native source attribution must be audited separately.
- Full license texts, bundled native third-party notices, actual linked components and platform runtime obligations remain open. **This does not close the distribution-license gate.**

## Reproduce and verify

```
node --test experiments/sdl3/packaging/license-inventory.test.cjs
node experiments/sdl3/packaging/license-inventory.cjs NEW_OUTPUT.json
```

Three tests pass: resolved-record filtering/privacy, deterministic ordering and compound-license preservation, and refusal of unresolved metadata. The generator uses pinned offline Cargo metadata, targets `aarch64-unknown-linux-gnu` explicitly and refuses existing output. No dependency installation or license acceptance occurs.

The original metadata remains local. The generated publication copy is deliberately narrower than a software bill of materials and must not be advertised as complete attribution or redistribution approval.
