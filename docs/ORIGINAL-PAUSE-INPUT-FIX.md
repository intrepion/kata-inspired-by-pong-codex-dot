# Original browser held input across pause

The original TypeScript browser adapter retained keyboard and on-screen button input when explicitly paused, so a still-held control could move the paddle on resume. Its canvas touch handler also accepted a continuing drag after resume. The mocked-handler regression also fails against the previous generated JavaScript: the drawn paddle moves after pause/resume while a key/button remains held.

The browser now clears keyboard, button-pointer and active canvas-drag sources when entering pause or restarting, and on blur. Only fresh keyboard presses and pointer-downs during play can start movement; keyboard repeat cannot re-arm a cleared key, and continuing canvas pointer moves cannot re-arm a cleared drag. Pointer up, cancellation and lost capture release individual ownership. Visibility loss while playing takes the same pause path. The generated `game.js` was rebuilt from `game.ts`.

On Omarchy, offline pinned TypeScript build and all 19 Node tests passed. The added tests cover independent keyboard/button/drag ownership and use a mocked DOM to exercise actual browser event handlers across pause/resume, a continuing drag and a fresh drag. This is controller and simulated-handler evidence, not browser, native or physical touch verification. A real browser recheck must use a localhost server, never a file URL; multi-touch, focus/visibility delivery and other platforms remain pending.

Tested tree based on `f2fa58ff5a178451786b3120880db00eef5d0f1e`; `game.ts` Git blob `a9b4835d2555fca8cd44b8c5c853f3b4f0865873`, generated `game.js` Git blob `ca32cd7b030c2463027fa7b3d913486520ac8728`. Node `v26.12.1`; pinned TypeScript `5.9.3` was installed from a copied local npm cache. No prior builds or artifacts were deleted.
