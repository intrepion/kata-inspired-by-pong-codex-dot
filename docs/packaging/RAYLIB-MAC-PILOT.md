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

## Native interaction and metadata follow-up

A second fresh extraction of the same archive was launched through the supported
native application interface. Initial brief Enter/W input left the game Ready;
held R subsequently started Playing. Held W moved the player paddle upward and
held S moved it downward. Scores advanced to 1:3, Space paused and resumed,
and R reset scores to 0:0. The final native query reported `procNotFound`,
confirming exit; intermediate Escape/Cmd-Q queries still showed an old window,
so no claim is made about which exit action took effect. The generated extracted
bundle was then removed precisely, preserving its neighboring sentinel.

This adds bounded real-input package coverage, not full-match, focus/resize,
quarantined download or clean-machine coverage.

The target-filtered ARM64 Mac Cargo closure contains 50 packages with declared
licenses and no missing declarations; three inventory tests pass. The sanitized
[inventory](raylib-mac-license-inventory.json) excludes local source paths and
unrelated packages. Declarations are not verified license texts or complete
bundled native-source notices, and do not establish redistribution clearance.
