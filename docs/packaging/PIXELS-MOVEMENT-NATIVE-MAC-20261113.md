# Pixels automated movement: Mac (2026-11-13)

Used the native release build documented in `PIXELS-COURT-NATIVE-MAC-20261113.md`, executable SHA-256 `49a879c01da5be7abf8acc7d236a45d97032c17dc29e691621e0015c1e5cb0b2`, with Pixels source matching checkpoint `ce015a506bbaab4851fbaec995803f61ff01e612`.

Desktop automation started from Ready, held Up for 800 ms, then paused. The left paddle appeared at the top of the court. After resuming, holding Down for 800 ms, and pausing, it appeared at the bottom. Holding Up for 800 ms while paused left the paddle at the bottom. These are visual observations following automated key events, not physical keyboard tests; they do not establish simultaneous held-key pause/resume behavior.

Two attempted bottom-right window drags did not change the observed window size. This does not determine whether the application or automation caused that outcome; resizing remains unverified. No player-win or release qualification is claimed.
