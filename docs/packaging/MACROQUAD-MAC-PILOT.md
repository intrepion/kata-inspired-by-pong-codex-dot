# Macroquad macOS ARM64 packaging pilot

2026-11-12 UTC. **Package creation passed; extracted runtime failed in this execution session. Not distribution-ready.**

Source binary: release build from `c2dacefad86aebd0fbd0231a6e32b94fc9d3e7e1`, SHA-256 `f8eadd114d1d7a6127b4e9db6cab121f4773d6c2833223734d510a6d8f7fee3d`. macOS ARM64. Existing binary reused; no dependency installation or security changes.

The app-owned `clients/macroquad/packaging/package-macos.cjs` makes a minimal `.app` and portable tar.gz in a NEW output directory, retaining the source binary and MIT license. It refuses existing output and non-arm64 Mach-O inputs. Node syntax, plist lint, archive creation/extraction and refusal-to-overwrite checks passed. Dynamic dependencies reported by `otool -L` are system libraries/frameworks. No public package uploaded.

Launch from an extracted staging directory outside the checkout with `--scripted-test` aborted with SIGABRT before telemetry. Crash report's main-thread stack involves LaunchServices initialization. A plain copy outside the bundle exited101: Miniquad0.4.11 `macos.rs:1039`, `assertion failed: !glpixelformat_obj.is_null()`. This is an observed runtime failure in the current tool/session, not proof of an application logic or package-only bug. No scripted state or visual pass is claimed. Retest in a working graphical execution session is needed.

Read-only `codesign -dv` showed the existing linker ad-hoc signature, no TeamIdentifier, Info.plist not bound and no sealed resources. This is NOT Developer ID signing/notarization. No signing modification or Gatekeeper bypass was performed.

Pending: successful bundle launch through LaunchServices, gameplay from extracted package, staged uninstall, downloaded/quarantined Gatekeeper behavior, third-party license inventory, clean-machine install, upgrades, Intel macOS support and signed/notarized distribution. No `/Applications` installation occurred. Pilot files and source build artifacts are retained.
