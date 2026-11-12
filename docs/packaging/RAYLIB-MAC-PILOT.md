# raylib-rs Mac local package pilot

2026-11-12, ARM64 macOS. Packaging source based on checkpoint `680553b`;
release binary is the previously measured raylib-rs binary identified by the
[package manifest](raylib-mac-package-manifest-20261112.json).

- Minimal `.app`, repository MIT license, pilot notice, tar archive and SHA-256
  manifest created. `plutil` accepts the bundle plist.
- All nine Mac packaging guard tests passed with real release binaries supplied:
  missing arguments, text input rejection before output creation, and existing
  directory/sentinel preservation for Macroquad, SDL3 and raylib.
- Extracted into a fresh temporary directory outside the checkout. Extracted
  executable hash equals the source and manifest hash.
- Relocated native scripted replay exited zero in 13.095 seconds, with thirteen
  telemetry checkpoints through tick 1440. An initially incorrect capture option
  was ignored; that first run is only a plain replay, not capture evidence.
- Correct `--scripted-captures DIRECTORY` retry exited zero in 12.334 seconds,
  with thirteen states and six PNGs. Tick 600 was inspected: both paddles, ball,
  score and instructions are visible. This is a scripted renderer sample, not
  real-keyboard, focus, resize or full-match acceptance.
- Removed only the generated extracted `.app`; adjacent sentinel survived.
  Source binary, archive, manifest and capture evidence retained. Processes exited.
- Shared benchmark runner robustness tests separately passed 22/22 on Mac,
  including abnormal-exit and POSIX termination-signal rejection.

The shell emitted a sandbox-extension warning for the temporary app path, but
both processes exited zero with complete telemetry; no permissions or security
settings were changed. This is not a Finder launch/Gatekeeper test.

No public binary release or system installation. No signing/notarization,
quarantined-download handling, clean-machine dependency validation, Intel build,
upgrade behavior or complete third-party license notices are qualified.
