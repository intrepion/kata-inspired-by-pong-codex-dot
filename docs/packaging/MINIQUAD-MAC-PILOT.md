# Miniquad Mac local package pilot

2026-11-12, ARM64 macOS, existing qualified release binary, no new dependency
build. The [manifest](miniquad-mac-package-manifest-20261112.json) identifies the
binary and archive hashes. Minimal app/plist, MIT license and pilot notice were
archived, extracted outside the repository and hash-checked against the source.

- Twelve packaging guard tests pass across Macroquad, SDL3, raylib and Miniquad,
  including existing-directory/sentinel protection using real ARM64 binaries.
- Relocated native `--scripted-test --scripted-captures DIRECTORY` exited zero
  in 12.764 seconds, with thirteen states ending at tick 1440 and six PNGs.
- Tick 600 inspected: paddles, ball, central divider and tally-style score appear.
  This renderer lacks the other adapters' text HUD; no text/readability parity
  or manual-input acceptance inferred from the scripted sample.
- Only the extracted generated app was removed; adjacent sentinel survived.
  Archive, source and capture evidence retained. No runtime remains active.

No Finder launch/manual-input test of this package, quarantine/Gatekeeper test,
Developer ID signing, notarization, full third-party notice clearance, clean-machine
install, upgrade or Intel qualification. This is not a public binary release.
