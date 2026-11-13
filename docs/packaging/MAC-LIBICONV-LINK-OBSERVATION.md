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
