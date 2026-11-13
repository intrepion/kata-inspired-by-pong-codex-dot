# Tauri Linux ARM64 notice feasibility

On 2026-11-13, Omarchy generated offline, locked Cargo metadata at revision `83ab151f854db9c6ddd076c688b12603a328a09b`, filtered to `aarch64-unknown-linux-gnu`. Traversing the resolved node dependencies from `pong-tauri` yielded 291 packages. The existing collector found no conventional notice file or declared `license-file` for five resolved packages:

| Package | Manifest declaration |
| --- | --- |
| `alloc-stdlib` 0.3.0 | BSD-3-Clause |
| `dlopen2` 0.8.2 | MIT |
| `dlopen2_derive` 0.4.3 | MIT |
| `libappindicator-sys` 0.9.0 | Apache-2.0 OR MIT |
| `selectors` 0.38.0 | MPL-2.0 |

These are missing-file findings, not determinations that notices do not exist or that redistribution is unlawful. Cargo metadata covers the resolved dependency graph, including build dependencies and potentially workspace-unified features; it does not establish which code is in the binary or inventory system libraries. The workspace lockfile SHA-256 was `660e386ac75417f47920c4809c755550e997bbbb3d9ae6178ae23b7971f57d06`. The lockfile and Tauri/core/scenario source paths were unchanged from the [previously qualified binary's source](../OMARCHY-TAURI-CONTACT-TIMING.md). The prepared notice text SHA-256 is `434bc1483f35d9221cff1fdddf591684594d6e9eb1a9a2224f34d48018c1b935`, with index SHA-256 `fe072dcf903a24ccd1919f69f601b9192b09455632d6b143d232e382a3e2f2a9`; the local files remain under `/tmp/pong-tauri-notices-*arm64-20261113.*`.

Passing this incomplete real bundle to the Linux packager was rejected before output-directory creation; the notice and portable-package test files also passed. The existing [portable pilot](TAURI-LINUX-PILOT.md) was not changed or overwritten. Review exact-version upstream sources and any embedded notices for the five gaps, as well as native and system libraries, before preparing a notice-bearing package. This scan is not legal clearance, a complete attribution review, a clean-machine install or a release qualification.
