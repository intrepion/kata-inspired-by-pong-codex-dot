# Mac softbuffer and Blue Engine local package pilots

2026-11-12, ARM64 macOS, source base `906fe03`. Existing qualified native release
binaries reused without compilation. [Sanitized manifests/results](mac-softbuffer-blue-package-20261112.json)
identify exact source/archive hashes. Minimal app plists validated; archives
extracted outside the checkout and executable hashes matched source manifests.

Softbuffer relocated native replay exited zero in 13.208 seconds with thirteen
states through finished tick 1440 and six PNGs. Tick 600 inspected: score,
paddles, ball and divider visible. Blue Engine relocated native replay exited
zero in 14.749 seconds with thirteen finished states. **Blue Engine native
captures are unsupported: zero captures is an explicit limitation, not a visual
pass.** No headless renderer was substituted. Its scripted process-exit path
also does not establish graceful normal-window shutdown.

Exact extracted generated apps removed, adjacent sentinels preserved; source
binaries, archives and softbuffer captures retained. All test processes exited.
Forty-two packaging guard tests passed across fourteen Mac scripts, including
existing-destination protection with real binaries. These lifecycle wall times
include startup, not controlled benchmarking.

No real-input/Finder/quarantine test for these packages, Developer ID signing,
notarization, complete third-party notice clearance, clean-machine dependency
validation, upgrade or Intel qualification. No public installer released and
no security settings changed.
