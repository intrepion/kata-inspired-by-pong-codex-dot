# Nannou focus-return actions: Mac cross-check (2026-11-13)

Nannou source blob `be499e984676ffdce7db3802cbfbdf4964427116` from `1be1376`, SHA-256 `ae82ba6e845c7afc75fccf8fc5296eccbc12ba2a65462c9f836cc75ee7831242`, was built offline and locked with the cached Mac release target. The executable SHA-256 was `396c43932e25d9a540b8680ee032330b25920c5d0da99da3093c2d52cc3d97c8`. All 15 release tests passed, including the synthetic first-focused-sample action suppression regression.

In the native Mac window, desktop automation sent 500 ms Return, Space, Space and Escape presses and observed Playing, Paused, Playing and app exit. This checks focused actions, not a confirmed native focus transition or an action held across it. The focus-return suppression remains synthetic polling evidence; physical input, full gameplay and release qualification are unverified. The app and prior binary remain under `/tmp/pong-nannou-focus-return-mac-20261113-v1/`.
