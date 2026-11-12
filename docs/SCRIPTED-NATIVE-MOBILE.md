# Portable native mobile replay (source candidate)

This batch adds platform-independent `ScenarioManifest.parse` and
`ScenarioReplay` APIs to the independent Swift and Kotlin game cores. Both
consume the same `scenarios/smoke-v1.txt` used by Rust and TypeScript. Existing
app entry points, touch controls, game timing and physics are unchanged.

**Not compiled or run on iOS/Android yet.** Omarchy has no Swift, Kotlin or Java
compiler on PATH or in the inspected mise, /opt and /usr/local toolchain paths.
No SDK was downloaded. The XCTest/JUnit suites are added source, not test-pass
claims. Only the Node resource consistency check and whitespace checks ran.

## Resource packaging

Run from repository root:

```
node scripts/package-mobile-scenario.cjs
node scripts/package-mobile-scenario.cjs --check
```

The script copies the sole canonical manifest byte-for-byte into derived test
resources and app assets. Do not hand-edit these copies. The check fails if any
is missing or stale. Android assets use `assets/scenarios/smoke-v1.txt`;
iOS XcodeGen's existing `App` sources include `App/Resources/smoke-v1.txt`.
Swift package test resources are explicit in Package.swift. Actual installed
app resource loading remains pending platform validation.

## API and future UI integration

Load the packaged text, parse it, construct a replay. Call `step()` once per
logical tick, render `replay.game` with the normal renderer, and inspect
`captureDue`/`reportDue` **after** each step. Initial tick zero is a report.
The caller owns pacing, file export and any opt-in launch argument. No new
UI demo switch or capture/export path is included here. In particular,
`captureDue` is a requested checkpoint, not evidence of a screenshot.

Actions execute before their manifest tick; input intervals are half-open;
the logical clock advances while gameplay is paused. Terminal `step()` is a
no-op. Parsers reject unknown fields/actions, duplicates, unordered events,
overlapping ranges, out-of-range captures and invalid/overflowing integers.
No timeline is embedded in either implementation.

These native cores retain their own existing Float physics and PRNG. The
added suites assert deterministic repeatability, action outcomes, pause and
restart behavior, report/capture scheduling and malformed-input rejection.
They do **not** claim Rust bit-identical trajectories.

## Platform verification still required

On Mac: `cd experiments/native-swift && swift test`, then regenerate/build the
Xcode project and inspect packaged resources. On a configured Android host:
`cd experiments/native-kotlin && ./gradlew :app:testDebugUnitTest`, then build
and inspect APK assets. Correct compiler/test failures before treating this
as verified native integration. Follow with opt-in UI wiring, bounded device
or simulator replay and supported app-owned image capture separately.
