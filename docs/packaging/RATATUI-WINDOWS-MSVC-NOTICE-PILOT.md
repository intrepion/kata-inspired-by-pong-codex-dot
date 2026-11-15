# Windows Ratatui MSVC notice-bearing debug pilot

On 2026-11-13, the previously [built and tested Windows MSVC debug executable](../WINDOWS-RATATUI-CONTACT-TIMING.md) was staged with the root LICENSE, the [MSVC notice discovery bundle and index](RATATUI-WINDOWS-MSVC-NOTICE-DISCOVERY.md), and an explicit incomplete-pilot warning. Its source was `c8caa9b8c17b296b1401da88f91786db364b3d56`; the later manifest-only MIT declaration enabled the first-party notice fallback. This local unsigned ZIP is not a release build.

A new 435,194-byte ZIP was extracted to a new path containing spaces. The extracted list contained exactly five expected files, and all matched staged SHA-256 hashes. ZIP SHA-256: `66c02dc6a569e9107f5b25a07caaf908d483825438710111254f9d2c50dce844`. Executable SHA-256: `c2a6a19a4df58e49914b715c6061ff42fbac2f83d5f0532c82f1d3d2c01f527b`. Notice bundle SHA-256: `115470a57bd719dcdccfd53815c8c43180ea3b459f2094c7c015c8f9257aad0d`, also verified against the included index. Index SHA-256: `2ddf2cb169e9140c9eb1af9e2af7c0c2e5acaa3041b91d341248e62c5b9441f0`.

The extracted executable is byte-identical to the binary already exercised in the [earlier relocation replay](RATATUI-C77-WINDOWS-DEBUG-PILOT.md); no replay or interaction was repeated for this archive. ZIP, staging and extraction remain locally in `c77-ratatui-msvc-notice-package-20261113`. Earlier packages and outputs remain intact; no installer or system changes were made.

Discovered texts are not legal clearance or exact linked-code attribution. Nested unlabelled code, system libraries, license choices and obligations require review. Real terminal presentation and input, clean-machine dependencies, signing, installation, upgrades, uninstall and release qualification remain open.
