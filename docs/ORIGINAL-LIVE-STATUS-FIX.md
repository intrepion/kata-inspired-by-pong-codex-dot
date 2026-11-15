# Original browser live-status updates

The original browser game wrote `textContent` to its `aria-live="polite"` status on every animation frame even when the status was unchanged. A mocked DOM regression counted another write on an unchanged frame and failed before the fix. Repeated live-region mutations can create unnecessary announcement work; the actual behavior of a particular screen reader was not tested.

Status text is now assigned only when it changes, including a scripted-mode error message. Real state transitions still update the text. On Omarchy ARM64, the pinned TypeScript build and all 23 Node tests passed. The mocked-handler test checks no write on an unchanged frame, one write on pause, and no additional write from a paused frame or repeated pause key. This is a simulated DOM check, not a browser, screen-reader or accessibility conformance result; those remain pending and browser testing must use localhost.

Tested source based on `d5ae9c0f0062fc65b41e19e29a92b7c95cd9ee47`; `game.ts` Git blob `8b0d6fca6375b1630114f8c3d668ba2234b52888`, generated `game.js` Git blob `d64f3c4479a4359264ffeec56681ff65d5fafab9`; Node `v26.12.1`, TypeScript `5.9.3`. The independent collision model and its before/after evidence are unchanged.
