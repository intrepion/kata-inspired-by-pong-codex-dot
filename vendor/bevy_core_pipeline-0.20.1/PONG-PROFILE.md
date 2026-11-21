# Pong Linear-only profile
Based on exact registry bevy_core_pipeline 0.20.1 (2463cf477b683bfe8567c43142295d00ed980f8e2a293c0a69e2f9e10e9c0aaf), original MIT and Apache notices retained.
The Pong workspace explicitly opts into this audited subset. Replaces the tonemapping shader module and fullscreen fragment, omits the three LUTs, and retains shader ABI/default SDR exposure, post-saturation, and alpha. No claim to stock alternate tonemapping, grading options, HDR or dither. Other upstream sources are unchanged; this is not a complete attribution assessment.
