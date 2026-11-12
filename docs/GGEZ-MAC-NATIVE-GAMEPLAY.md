# ggez Mac native gameplay follow-up

Existing local unsigned pilot binary SHA-256: `6254ac6bcae55b5e6e468f2f64921e474c2e46dee90331f99681b18f50c786fb`. No rebuild, scripted flags or new dependency downloads were used.

Supported native controls and actual window observations established:

- Enter started play; held S moved the left paddle downward. Space produced visible **Paused** with score 0–1.
- Enter resumed; held W moved the left paddle to the top. Gameplay progressed through 0–4 and then **0–7 Over**.
- Enter began another match; Space then showed **0–0 Paused**, with both paddles recentered.
- The actual 1000×744 window capture showed the score, state overlay, control legend, court, divider, paddles and ball coherently.
- Application-menu Quit returned `procNotFound`; a subsequent process check found no matching test process.

This qualifies a supported native full-match/restart/pause interaction on this existing Mac binary, not physical keyboard release-order, focus loss, resize/DPI, clean-machine installation or release signing. Directly inspected window captures were not saved as repository image files. Earlier package and benchmark hashes remain historical and unchanged.
