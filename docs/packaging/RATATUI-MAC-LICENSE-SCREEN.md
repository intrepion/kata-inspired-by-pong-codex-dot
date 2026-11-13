# Mac Ratatui target-filtered license metadata screen

On 2026-11-13, the historical checkout `e2b3cb455045a5c31b40539bf1b4980b91078cf3` used for the [Mac ARM64 Release package pilot](RATATUI-MAC-RELEASE-PILOT.md) was screened with:

```sh
cargo metadata --offline --locked --format-version 1 --filter-platform aarch64-apple-darwin --manifest-path experiments/ratatui/Cargo.toml
cargo tree --offline --locked --manifest-path experiments/ratatui/Cargo.toml --target aarch64-apple-darwin -e normal --prefix none
```

The metadata reported 68 resolved packages, including build dependencies and proc macros, and every package supplied a license expression. Case-insensitive text screening found no expression containing `LGPL`, `GPL-3` or `MPL`. The declared expressions consisted of MIT, Apache-2.0, Zlib, 0BSD, Unlicense, BSL-1.0, Unicode-3.0 and LLVM-exception forms or alternatives; `(MIT OR Apache-2.0) AND Unicode-3.0` includes both conjuncts. This is a report of manifest declarations, not verification of each source file or an assertion that all resolved packages are embedded. The local metadata SHA-256 is `07b2cf3f1aed0ac68dea468e33b23374f4443d1a52753220dd391da931fcd6de`; the normal dependency tree output SHA-256 is `959fa8941b3d225b0656e74d21b269bb0f0c0efa175c511eb7952c2e17696e16`. Both remain under `/tmp/pong-ratatui-mac-*-20261113-v1.*`. The lockfile SHA-256 is `3cce5b2b1629f0bd517b4a85b368ed3632084af6e0e5f9f47ad49fbaa5456210`.

For the tested executable SHA-256 `69a33c4dc18c8f537e6194958cf7f7ef26efb2f6408f94537f24200cfd643e0b`, `otool -L` lists direct system loads `/usr/lib/libiconv.2.dylib` and `/usr/lib/libSystem.B.dylib`. Apple source and runtime evidence suggests a FreeBSD-derived iconv implementation and a separate libcharset re-export, but the precise installed-build source mapping and complete system-component licenses remain open; see [the provenance investigation](MAC-LIBICONV-LINK-OBSERVATION.md). No replacement is proposed from a name match alone.

This scan does not establish actual statically linked code, dynamic loads beyond direct load commands, exact source-license coverage, notices, clean-machine behavior or release clearance. The screened graph is a historical Mac target and can change with dependencies or features. Original builds, archives and evidence remain intact. Checkpoint when recorded: `75c5640ac395491dbbcab2be18907b34422fa179`.
