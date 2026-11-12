# xattr 0.2.3 compatibility patch

Source: the official crates.io xattr 0.2.3 package, SHA256 checksum `6d1526bbe5aaeb5eb06885f4d987bcdfa5e23187055de9b83fe00156a821fabc`. MIT/Apache-2.0 licenses and original tests retained. All upstream files included here are unchanged except `src/sys/mod.rs`.

GPUI 0.2.2 -> gpui_http_client -> zed-async-tar 0.5.0-zed requires xattr 0.2.x. On current Linux libc, the obsolete `libc::ENOATTR` alias no longer exists. This patch uses `libc::ENODATA` for Linux/Android only; macOS/BSD retain `libc::ENOATTR`. This preserves the missing-attribute error semantics, without downgrading libc, disabling attributes, or touching filesystem/security settings. Older libc source explicitly documented ENOATTR as a deprecated alias for ENODATA on Linux/Android; modern xattr uses NODATA on these platforms too.

The adapter-local `[patch.crates-io]` selects this source. Remove it when the upstream dependency chain supports modern xattr/libc directly. No other crates are patched.

Omarchy verification: build and strict adapter Clippy pass; upstream four integration tests and one doctest pass (path/fd missing attributes, get/set/list/remove, multiple attributes). Tests run from a temporary copy with only an empty `[workspace]` added to its manifest, avoiding nested-workspace test restrictions. No temporary test lockfile or target is vendored.
