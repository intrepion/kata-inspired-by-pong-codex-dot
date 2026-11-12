# Slint Windows portable snapshot pilot

Fresh Slint1.18.1 software-renderer release build5m44s. Adapter has zero unit tests; shared GUI support has six passing release tests, including simultaneous held aliases, release stop, focus pause, repeated-pause suppression and bounded catchup. Do not count zero tests as native-input coverage.

New local unsigned ZIP, fresh extraction to a path containing spaces and exact binary hash match passed. [Manifest](slint-windows-manifest-20261112.json). Run from extracted directory: exit0 in12.676s, thirteen states through finished tick1440 and six own-window snapshots. Tick600 inspected: title, score, paddles, ball, divider, full control line and retained Slint attribution badge visible, no clipping at1200x813.

Snapshots use Slint's own-window API; no real keyboard/focus or compositor screenshot acceptance inferred. Shared held-input fix was compiled and its unit tests passed, but OS dispatch remains a separate gate.

Local archive includes executable, project LICENSE and explicit warning that Slint license choices/full third-party notices, clean-machine dependencies, signing/download security and installer/upgrade/uninstall are not qualified. No public binary distribution, system changes, security bypass or build/staging deletion. This does not choose a commercial/GPL/other Slint licensing option on the user's behalf.
