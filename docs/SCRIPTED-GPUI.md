# GPUI scripted replay: state evidence only

```sh
cargo run --locked --manifest-path experiments/gui-gpui/Cargo.toml -- --scripted-test
```

The native GPUI app advances the existing shared Demo during its normal render/update loop, ignoring real key/focus events only in scripted mode. It keeps requesting frames even when unfocused and quits on the frame after the scenario completes. Normal interactive behavior is unchanged. No duplicate timeline, OS input injection or desktop capture is used.

`--scripted-captures` is deliberately rejected with exit code 2 before creating a directory. GPUI 0.2.2's inspected public APIs provide OS screen-capture sources, not a supported application-owned pixel-export path for this adapter. We do not use those screen-capture APIs, private renderer internals, or fabricate screenshots from state.

## Linux compatibility

The old transitive xattr 0.2.3 dependency references removed libc::ENOATTR on Linux. A narrowly scoped adapter-local vendored patch uses the equivalent ENODATA on Linux/Android, retaining macOS/BSD behavior. [Source, licenses, rationale and tests](../experiments/gui-gpui/vendor/xattr/LOCAL-PATCH.md). libc and all Windows dependency pins remain unchanged. GPUI itself remains pinned to 0.2.2.

## Omarchy ARM64 verification, 2026-11-12

- Locked offline build and strict adapter Clippy pass. Cargo still reports an existing future-incompatibility warning for proc-macro-error2 2.0.1; this is not a Clippy error.
- Fourteen core/scenario and four GUI-model tests pass.
- Patched xattr's four original integration tests plus one doctest pass.
- Bounded native replay exits 0 with thirteen byte-identical baseline telemetry records through tick 1440.
- Unsupported capture invocation exits 2 and creates no output directory.

This verifies execution/state integration only. No app PNG, visual inspection, compositor result, real keyboard/focus test, resize test, or full-match QA is claimed. Mac and Windows checks of this new integration remain separate.

[Telemetry](evidence/omarchy-gpui-scripted-20261112.jsonl) · [Verification summary](evidence/omarchy-gpui-scripted-20261112.json)
