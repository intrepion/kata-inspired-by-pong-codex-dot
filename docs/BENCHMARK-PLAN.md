# Benchmark and packaging architecture plan

Status: **approved direction; not implemented or benchmarked**.

## Shared benchmark harness

Use one shared harness for repeatable Pong comparisons rather than separate benchmark projects for each operating system. Keep scenario definitions, run configuration, result schema and reporting shared. Add platform-specific helpers only when a useful capability actually requires them, such as process/resource measurement, device launch or packaging invocation.

Build on the existing deterministic scenario work where appropriate, but keep correctness replay and performance measurement distinct. Existing replay telemetry, framebuffer samples, development build times and debug executable sizes are not comparable benchmark results.

Before publishing performance comparisons, record the tested revision, hardware, OS, graphics backend, compiler/profile, window resolution and timing methodology. Separate warm-up from measured runs, repeat measurements, and report unsupported/unmeasured metrics explicitly. Do not rank different machines as if they were identical test conditions. Native, WebView, browser and terminal implementations have different presentation costs; retain those labels in reports.

The first implementation should establish a small reproducible result format and one end-to-end measured adapter run before broadening the adapter matrix. Exact metrics and measurement tooling remain implementation choices to validate, not completed features.

## Packaging stays with each app

Keep packaging configuration, assets and platform-specific instructions alongside the application they package. Do not prematurely introduce a universal packaging framework. Extract a shared helper only after repeated, stable steps across applications justify it; preserve app-specific dependency notices, signing and runtime requirements.

A successful build is not a tested installer. Distribution readiness needs separately recorded package creation, installation/launch on the target platform, required license notices and any applicable signing checks. No installer publication, store submission or additional signing/account changes are implied by this architecture decision.

## Evidence boundary

This document records the user's chosen architecture. It adds no performance measurements, package results or new platform passes. See the root README testing dashboard and linked evidence for actual verification.
