# Windows notice-bearing local ZIP pilots

## Scope

Softbuffer, SDL3 and ash now have new local unsigned ZIPs containing their executable, project LICENSE, a dependency-notice text bundle, a hash-indexed notice inventory and an explicit pilot warning. This is preparation evidence, not licensing clearance, proof of exact linked-code attribution or a public release.

Cargo metadata was resolved offline and locked for x86_64-pc-windows-gnu. The shared collector retained actual discovered license/notice texts verbatim, including nested notices and alternatives, without selecting a license option. Its new optional platform label is constrained to Linux, Windows or macOS; omitted labels retain byte-identical Linux output. The added regression failed before correction and passes afterward. All thirteen applicable packaging tests pass on Windows; seven platform cases are skipped.

| Adapter | Cargo packages | Discovered notice files | Packages lacking discovered files |
| --- | ---: | ---: | ---: |
| softbuffer | 35 | 68 | 0 |
| SDL3 | 34 | 113 | 0 |
| ash | 36 | 69 | 0 |

For each archive, fresh extraction into a path containing spaces matched all five staged files by SHA-256; the bundled text hash also matched the notice index. Extracted native scripted runs each exited 0 with thirteen state records through finished tick 1440 and six app-owned PNGs. Tick 600 from each was inspected. [Archive, executable and notice hashes](windows-notice-packages-20261112.json).

All archives, staging, extractions, logs, source metadata and captures remain local and retained. The source metadata contains host paths and is not committed. No binaries were publicly uploaded, no installer or system/security settings changed, and prior archives were not overwritten.

## Still open

A discovered file for every resolved Cargo package does not establish complete obligations: optional/build dependencies, actual linked content, nested unlabelled source, native DLLs/system libraries, licensing choices and attribution requirements still need review. Other Windows adapters retain explicit missing-file gaps in the [feasibility report](WINDOWS-NOTICE-FEASIBILITY.md). Clean-machine dependencies, signing/download handling, installer/upgrade/uninstall, physical keyboard/focus and release qualification are separate. Existing benchmark evidence is not rerun by this packaging work.

## Windows containment correction

A new sibling-checkout test exposed an existing Windows path-separator bug in first-party MIT fallback: a local package outside the repository could be incorrectly assigned the root license when its relative path began with a Windows parent-directory separator. The failing regression was reproduced, both separator styles are now rejected, and the test passes. Regenerating these three actual notice bundles after the fix preserves their exact hashes, so their source containment was valid and their archived texts are unchanged.

## Linux cross-platform validation (2026-11-12)

On Omarchy ARM64, source checkpoint `0e2bf9b98266b592f35e82d6ebd6b19cfdec39d1` passes all five notice-bundle tests, including rejection of an outside sibling checkout and byte-identical implicit/explicit Linux platform output. The combined notice, Linux portable packaging and Fyrox state-safety suites pass 18 tests; two Windows-only junction tests skip. The packaging fixture covers eighteen adapters. No production package, archive, registry or user data was modified by this cross-check. This validates shared logic, not Windows runtime behavior or license clearance.
