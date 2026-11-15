# Mac ARM64 direct libiconv linkage observation

On 2026-11-13, `otool -L` was run against four previously qualified local Mach-O ARM64 Release executables on macOS 27.2.1 (build 26B131). Each listed the direct load command `/usr/lib/libiconv.2.dylib`, compatibility and current version 7.0.0:

| Local pilot executable | SHA-256 |
| --- | --- |
| Ratatui | `69a33c4dc18c8f537e6194958cf7f7ef26efb2f6408f94537f24200cfd643e0b` |
| bracket-lib | `2518e0228d105b89f392c3e50efd09774d6591a745b5b6c4e3e83c36d62b90e7` |
| softbuffer | `51ebf96596fc6e92d1c1499eb03604599db19b4d8308e86f4d79a4b2139a5694` |
| Fyrox | `c68c5c4f7039fca32da063008336af4978a0db04c18f3fe2f99e6ed92db67cc7` |

These are system library load commands, not evidence that the library was included in the archives. `nm -m` on the Ratatui executable returned no symbol containing `iconv` or `charset`; the cause of its direct load command and whether it can be removed without affecting functionality have not been established. Other direct system framework links were not audited here.

License identification needs care: [GNU libiconv's own description](https://www.gnu.org/software/libiconv/) states its libraries are LGPL; [Apple's libiconv source notice at revision f96c1fc](https://github.com/apple-oss-distributions/libiconv/blob/f96c1fcdbb308374e39290676b5fea40a3859d17/libiconv.txt) describes BSD variants. The install name alone does not prove which source revision or license applies to the installed macOS library. Exact OS-build source mapping, complete per-file notices, indirect dependencies and possible replacement/removal remain open. This is a lead for the target-specific audit, not a license clearance, a GPL/LGPL determination for this installed library, or authorization to publish binaries.

The tested artifacts and prior package evidence remain intact. Branch checkpoint at inspection was `56756e7c2e34e8139e97a4572b006105f058fbd4`.

## macOS system-library provenance follow-up

On the same macOS 27.2.1 build 26B131, `/usr/lib/libiconv.2.dylib` is not an ordinary file visible at that pathname, but `xcrun dyld_info -uuid /usr/lib/libiconv.2.dylib` resolved the arm64e image and reported UUID `4FF0A750-C631-3959-8C50-030C5B02A61E`. `dyld_info -exports` showed `___bsd_iconv`, `___bsd_iconv_open` and many `__citrus_*` exports. Its `-dependents` output reported a **re-export** of `/usr/lib/libcharset.1.dylib` and a dependency on `/usr/lib/libSystem.B.dylib`. The local full export listing is retained at `/tmp/pong-mac-libiconv-exports-20261113-v1.txt`, SHA-256 `a20cf30c8677895133c9e317b69ce2d8084faeb0c8f73acbf3b98bfab784fe85`.

The installed Xcode macOS 27.2 SDK's `usr/lib/libiconv.2.tbd` (SHA-256 `35f0a64f0171c32c851d3e2f6739109d9fb7b94b992be990b7a6d75168b50fd7`) declares the same install name and re-export, and lists BSD/Citrus symbols. A stub documents the SDK interface; it is not the runtime binary or proof of source identity.

Apple's [published source metadata](https://github.com/apple-oss-distributions/libiconv/blob/f96c1fcdbb308374e39290676b5fea40a3859d17/libiconv.plist) identifies a 2022 FreeBSD libc iconv import and declares BSD for that project, and the [project notice](https://github.com/apple-oss-distributions/libiconv/blob/f96c1fcdbb308374e39290676b5fea40a3859d17/libiconv.txt) lists BSD variants. Apple's [build project](https://github.com/apple-oss-distributions/libiconv/blob/f96c1fcdbb308374e39290676b5fea40a3859d17/libiconv.xcodeproj/project.pbxproj) includes both BSD iconv and a separate libcharset target; its [libcharset.c header](https://github.com/apple-oss-distributions/libiconv/blob/f96c1fcdbb308374e39290676b5fea40a3859d17/libcharset/libcharset.c) states Apple Public Source License 1.0. These matching names and re-export are strong evidence consistent with Apple's FreeBSD-derived implementation, not proof that this installed image was built from that exact source revision. In particular, do not apply the BSD project summary to every re-exported component without per-file review.

Apple's public macOS distribution main tree pointed to libiconv revision `593134fb16998e214952a13ea0826a50eff4dfbf`, but the observed distribution page identified macOS 26.0; no authoritative mapping to the installed 27.2.1 build was established. The exact installed source revision and full license inventory, including charset data and re-exported components, therefore remain unresolved. The GNU LGPL description in the earlier section is a separate implementation, not evidence that this Apple runtime is GNU libiconv. No binary or implementation was removed or published.
