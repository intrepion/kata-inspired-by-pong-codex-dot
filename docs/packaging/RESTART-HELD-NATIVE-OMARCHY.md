# Active restart held input: native follow-up, Omarchy 2026-11-13

ggez and Piston retained held movement when R restarted an active match. Miniquad did so for Enter, keypad Enter and R. Each newly added event-handler regression failed before its fix, then passed after the restart handler cleared held movement. Repeat movement events remain ignored until a fresh press; Piston's regression also checks release and fresh press. Locked offline full test suites passed: ggez 7, Miniquad 13, Piston 12. These are handler tests on Omarchy, not physical keyboard, installed package, or cross-platform checks.

A bounded source review of the native mobile restart paths found Swift's button calls `touch.suppressActive()` before `game.start()`, and Kotlin's restart calls `TouchInput.start`, which clears the owner before starting. Kotlin already has `heldGestureDoesNotMoveAfterRestart`, covering a held gesture and fresh down in an active game. This review did not run mobile tests or establish physical-touch behavior; mobile files were not changed.

Further adapters remain to be audited. In particular, Fyrox's R handler and Nannou/raylib polling paths deserve focused regressions: their source suggests they may also retain movement across an active restart. This report does not assert those cases as independently reproduced or claim repository-wide coverage.

Tested source Git blobs: ggez `9f3ef41f5376e8a7917ea5ace9386506717b43db`, Miniquad `77abb5d0d58a29be8003371853b86d204646d54c`, Piston `a730d72e1353d80a8c8596f77d6028d6736854e1`. Earlier binaries, captures and qualification remain historical.
