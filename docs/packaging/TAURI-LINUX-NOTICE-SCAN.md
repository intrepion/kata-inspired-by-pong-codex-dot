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

## Exact-version source follow-up

The cached `.crate` archives for all five package versions match their checksums in `Cargo.lock`. Each crate's `.cargo_vcs_info.json` identifies a revision and package path. The connected GitHub app retrieved the corresponding upstream `lib.rs` for each package at that revision; each matched the cached crate's file byte-for-byte. This checks one source file per crate, not the entire archive or every notice.

| Package | Recorded upstream revision and verified notice evidence |
| --- | --- |
| `alloc-stdlib` 0.3.0 | [`0a81fd6928ea3b33c8cd484aa4575d50ffb98012`](https://github.com/dropbox/rust-alloc-no-stdlib/tree/0a81fd6928ea3b33c8cd484aa4575d50ffb98012); the upstream root [LICENSE](https://github.com/dropbox/rust-alloc-no-stdlib/blob/0a81fd6928ea3b33c8cd484aa4575d50ffb98012/LICENSE) was retrieved. Its package manifest also matches the archive's original manifest. |
| `dlopen2` 0.8.2 and `dlopen2_derive` 0.4.3 | Both record [`cc80e4a0a90d499b677fdf7743699b4b3a43a989`](https://github.com/OpenByteDev/dlopen2/tree/cc80e4a0a90d499b677fdf7743699b4b3a43a989); the common upstream root [LICENSE](https://github.com/OpenByteDev/dlopen2/blob/cc80e4a0a90d499b677fdf7743699b4b3a43a989/LICENSE) was retrieved. Their upstream manifests differ from the packaged `Cargo.toml.orig`, so only the corresponding source files are asserted byte-identical. |
| `libappindicator-sys` 0.9.0 | The matching upstream `sys/Cargo.toml` and `sys/src/lib.rs` identify [`eafd1e3682a1247f595410266091e9684021cb6f`](https://github.com/tauri-apps/libappindicator-rs/tree/eafd1e3682a1247f595410266091e9684021cb6f). Root [MIT](https://github.com/tauri-apps/libappindicator-rs/blob/eafd1e3682a1247f595410266091e9684021cb6f/LICENSE-MIT) and [Apache](https://github.com/tauri-apps/libappindicator-rs/blob/eafd1e3682a1247f595410266091e9684021cb6f/LICENSE-APACHE) texts were retrieved. The published manifest does not itself specify a repository URL. |
| `selectors` 0.38.0 | The matching manifest and `selectors/lib.rs` identify [`572ecba2d1600e7c3d490586692a209faf703baa`](https://github.com/servo/stylo/tree/572ecba2d1600e7c3d490586692a209faf703baa). Its source header refers to MPL 2.0. A request for root `LICENSE` at that revision returned 404; the web tree view was restricted, so this does not prove that no license text exists elsewhere. |

The collector still reports all five: it only reads notice files contained in package roots and has not been changed to import upstream files. These upstream texts are leads for a separately reviewed, hash-pinned inclusion approach, not a complete audit or legal clearance. Source-level MPL obligations, bundled/native code, system libraries and all other distribution requirements remain open.
