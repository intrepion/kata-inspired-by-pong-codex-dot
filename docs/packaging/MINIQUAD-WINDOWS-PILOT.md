# Miniquad Windows portable ZIP pilot

Local unsigned archive creation, fresh extraction to a path containing spaces, exact executable SHA-256 match, and execution from the extracted working directory passed. [Manifest](miniquad-windows-manifest-20261112.json). The existing release binary from the timing pilot was reused; source and build caches remain intact.

The ZIP contains the executable, project MIT LICENSE and explicit pilot warning. PowerShell Compress-Archive and Expand-Archive were used with literal paths; both destinations were verified absent first. No registry/PATH/permissions changes or security-warning bypass occurred. Staging and extracted artifacts remain local, not publicly distributed.

Extracted scripted run exited 0 in 12.368s, thirteen state records through tick1440/finished=true and six screenshots. Inspected tick600: paddles, ball and center divider are visible; this adapter's minimal rendering does not provide the rich text HUD of other adapters. This does not qualify native keyboard/focus or full-match interaction.

This is NOT an installer, clean-machine dependency, signing/SmartScreen, upgrade/uninstall, or completed third-party license-notice pass. Those remain open. No staging files were removed during ongoing development.
