# Original browser movement-button keyboard input

The on-screen Up and Down buttons previously only handled pointer events. Space on a focused movement button bubbled to the document pause shortcut instead of moving the paddle; Enter had no movement handler. A mocked DOM-handler regression failed before this change.

Space or Enter now holds the focused movement button during play and releases it on keyup or button blur. Key events are stopped before the global Space pause shortcut. Each button/key combination owns its movement independently of other keys and pointers; repeats do not re-arm movement after pause clears it. These buttons are displayed by the existing narrow-layout CSS. This change does not add handling for assistive-technology synthesized clicks.

The offline pinned TypeScript build and all 19 Node tests passed on Omarchy. Mocked handler checks exercise Space and Enter, opposing inputs, release, blur, pause/resume and repeat, plus existing pointer/drag checks. This is simulated event handling, not a real browser, physical keyboard or assistive-technology pass. Browser recheck must use localhost; synthesized click behavior, device interaction and cross-platform qualification remain open.

Tested tree based on `375e1336ab11314eda77ed5b3eabc48ecd374315`; `game.ts` Git blob `e2a0360fb8482d385eeeec0f44fdad2519e71256`, generated `game.js` Git blob `9b3e7f6f66c3c7d14d5e3c16b9dbea5adf374a6b`; Node `v26.12.1`, TypeScript `5.9.3`. [Start/Pause Space fix](ORIGINAL-BUTTON-SPACE-FIX.md) remains separate evidence.

## Windows Chrome recheck

At checkpoint `53f942b82f88fb7333eac3fc15b076199ff51736`, retaining the source and generated JavaScript blobs above, Node v24.20.0 passed all 19 checked-in JavaScript tests (no TypeScript rebuild). On the localhost server, supported Chrome automation used a 500-by-900 viewport to reveal the movement buttons. After starting, Enter on focused Down moved the paddle downward, and Space on focused Up moved it upward; status remained in play. The viewport override was reset after testing.

This is a bounded automated keyboard check, not physical keyboard, touch, assistive-technology or synthesized-click verification. It does not establish continuous input held across pause/resume or a full match.
