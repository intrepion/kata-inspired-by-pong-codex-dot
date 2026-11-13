# Nannou Ready controls: macOS check (2026-11-13)

Nannou now displays movement, pause, restart and exit controls on the Ready court, where they are available even if the window title is hidden. The hint is deliberately limited to Ready: an initial always-visible candidate overlapped the ball during a paused frame. The adapter was based on blob `10bfc6269964086291bdd0a042b4f666d9c4aa3d` at checkpoint `9ec2a0a13d4215add620989fde3c12e4dc4ef05e`. The isolated build tree was not established to match that whole checkpoint.

The offline locked release build succeeded and all 16 existing frontend tests passed. Adapter SHA-256: `cb26a3e902c8c6c1fd46bbc195b21ae154da1de06cc5ad13fa1a0221e6829ac9`; executable SHA-256: `72b69ccccd875408fb4a1cb957d179f555211f5a508d5bacb566afda113a8d3b`. A native Mac window showed the complete hint at Ready. An automated Space key started play, and a subsequent image showed the hint absent. The test window was closed.

This is windowed visual and automated input evidence, not physical input or release qualification. A fullscreen shortcut did not visibly enter fullscreen; fullscreen layout, other sizes and platforms, and full-match behavior for this build remain unverified. Complete notices and native/system license review remain open.
