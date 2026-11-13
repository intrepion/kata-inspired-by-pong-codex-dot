# Original browser button Space activation

The original game's document-level Space handler called `preventDefault()` even when Start or Pause had keyboard focus. In Ready or Over this prevented normal Space activation of Start; on Pause it also intercepted the control's own activation. A mocked DOM-handler regression failed before the fix because Space on Start was default-prevented.

The handler now leaves Space targeted at Start or Pause to the browser's normal button behavior. Space elsewhere remains the game pause/resume shortcut, with repeat suppression. On Omarchy, the pinned TypeScript build and all 19 Node tests passed. The mocked-handler test checks that Start and Pause Space events are not prevented, simulates the resulting button click, and checks the court shortcut and its repeat behavior. It does not demonstrate actual browser focus, default click dispatch, assistive technology or physical keyboard input; real browser recheck remains pending and must use localhost.

Tested tree based on `7cb29f8cbc5fbed06bb671f36e532fbfdc5fb9ec`; `game.ts` Git blob `b41dc7ec6b2f3d90efbc3b9730e46fe65ff4a0a6`, generated `game.js` Git blob `3276da3db9d74bd0039c37026231bbebe505237b`; Node `v26.12.1`, TypeScript `5.9.3`. Earlier held-input evidence remains at [the previous fix](ORIGINAL-PAUSE-INPUT-FIX.md).
