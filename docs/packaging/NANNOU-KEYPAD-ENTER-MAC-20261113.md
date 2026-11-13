# Nannou keypad Enter — macOS, 2026-11-13

Nannou now accepts `NumpadEnter` for the same start/restart action as Enter. Its focus-return gate tracks the two keys independently, preserving suppression of keys held during focus loss while allowing the other key's fresh press.

All 16 offline locked release-profile tests passed on Mac. Synthetic polling tests cover all five action bindings and independent main/keypad Enter suppression across focus return. In the rebuilt native window, the supported desktop tool's `KP_Enter` command started from Ready; after Space paused at 1–0, a second `KP_Enter` restarted at 0–0. Escape closed the app. The first app-selection call timed out, but inventory showed the single candidate running; it was selected and checked without launching another.

The executable SHA-256 is `159a36e5e5f11d5f2d282ec673732498c10d92fae424c2fcf1b5d1757d00e09e`; edited adapter SHA-256 is `b466f872cafd66869d3c2414405d776641f09d6a61ad18e20a2f70d82970263b`. The mixed isolated tree used adapter base blob `be499e984676ffdce7db3802cbfbdf4964427116` from checkpoint `783a6cb56cbb8d36b3ab8ae077f2e5492ec6faff`; other inputs were not all compared. The previous source and executable and candidate app remain in `/tmp/pong-nannou-keypad-mac-20261113-v1`.

The native commands were automated, and their mapping to physical keypad events was not independently observed. Physical keyboard input, native focus-return timing, other platforms and release qualification remain open.
