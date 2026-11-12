# GPUI Windows portable state-replay pilot

Fresh locked release build completed10m37s. Adapter unit-test target has zero tests; six shared GUI support release tests were separately verified in the Slint batch. Existing source/patches retained. Build warning: proc-macro-error2 2.0.1 contains future Rust incompatibility; current build succeeds, not a warning-free claim.

New local ZIP with executable, project LICENSE and incomplete-pilot notice. Fresh extraction into a path containing spaces; executable hash matched source exactly. [Manifest](gpui-windows-manifest-20261112.json). Run from extracted working directory using `--scripted-test`: exit0 in13.314s, no stderr, thirteen state reports ending finished tick1440. Source/build/staging retained; no system install or security changes.

GPUI0.2.2 here has no supported app-owned pixel export. This validates native event-loop state replay, not screenshots or real OS keyboard/focus handling. No desktop-control approval retry was made. No actual-input or visual acceptance inferred.

This is a local portable runtime pilot, not installer/clean-machine, signing/download, complete third-party notice, upgrade/uninstall or public distribution qualification. Shared GUI alias fix was compiled, not manually exercised through Windows input in this batch.
