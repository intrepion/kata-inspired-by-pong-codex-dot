# Macroquad macOS ARM64 packaging pilot

2026-11-12 UTC. **Package creation and extracted-app GUI gameplay passed; earlier shell-context launch failed. Not distribution-ready.**

Source binary: release build from `c2dacefad86aebd0fbd0231a6e32b94fc9d3e7e1`, SHA-256 `f8eadd114d1d7a6127b4e9db6cab121f4773d6c2833223734d510a6d8f7fee3d`. macOS ARM64. Existing binary reused; no dependency installation or security changes.

The app-owned `clients/macroquad/packaging/package-macos.cjs` makes a minimal `.app` and portable tar.gz in a NEW output directory, retaining the source binary and MIT license. It refuses existing output and non-arm64 Mach-O inputs. Node syntax, plist lint, archive creation/extraction and refusal-to-overwrite checks passed. Dynamic dependencies reported by `otool -L` are system libraries/frameworks. No public package uploaded.

Launch from an extracted staging directory outside the checkout with `--scripted-test` aborted with SIGABRT before telemetry. Crash report's main-thread stack involves LaunchServices initialization. A plain copy outside the bundle exited101: Miniquad0.4.11 `macos.rs:1039`, `assertion failed: !glpixelformat_obj.is_null()`. This is an observed runtime failure in the current tool/session, not proof of an application logic or package-only bug. That failed shell run produced no scripted-state or visual pass. The subsequent supported GUI launch below succeeded without modifying the binary or security settings.

Read-only `codesign -dv` showed the existing linker ad-hoc signature, no TeamIdentifier, Info.plist not bound and no sealed resources. This is NOT Developer ID signing/notarization. No signing modification or Gatekeeper bypass was performed.

Pending: downloaded/quarantined Gatekeeper behavior, complete third-party license texts/notices, clean-machine install, upgrades, Intel macOS support and signed/notarized distribution. No `/Applications` installation occurred. Pilot files and source build artifacts are retained.

## Supported GUI follow-up

2026-11-12 UTC: launched the exact extracted `.app` by path using the supported native-app tool. Ready court rendered normally. Actual keyboard Enter started play; held W and S moved the player paddle in opposite directions; CPU scoring advanced to2; Space visibly paused; Enter resumed; R reset scores to0:0 with play continuing. The window was closed after the check and the tool reported no eligible process. This is bounded native gameplay from the extracted package, not a full match, clean-machine install or downloadable Gatekeeper qualification. The contrast with the earlier shell-context failure points to launch/session conditions; exact underlying cause remains unproven.

## Staged removal and dependency inventory

The exact extracted test `.app` was removed only after its executable SHA-256 matched the retained source and the retained archive checksum matched the original manifest. Removal preserved a neighboring sentinel, original release binary and archive byte-for-byte. This is a staged removable-bundle lifecycle, not an installer upgrade/uninstall or removal from `/Applications`. No development build artifacts were removed.

[Macroquad-target macOS dependency metadata](macroquad-mac-license-metadata-20261112.json) contains49 packages, all with declared license expressions. The offline locked Cargo graph was filtered to `aarch64-apple-darwin`, then traversed from `pong-macroquad` only, excluding unrelated workspace roots. Three inventory regression tests passed (cycle/closure, missing root, missing node); private paths and hardware/process metadata are not emitted. Missing-input and existing-output packaging guard probes passed. Metadata declarations are NOT verified license texts, binary linkage proof or redistribution clearance. Bundled native-source notices remain a review gate.
