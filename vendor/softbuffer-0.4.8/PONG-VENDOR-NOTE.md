# Softbuffer 0.4.8 macOS-only prototype fork

Upstream source: https://static.crates.io/crates/softbuffer/softbuffer-0.4.8.crate
SHA-256: aac18da81ebbf05109ab275b157c22a653bb3c12cf884450179942f81bcbf6c3
Recorded upstream revision: https://github.com/rust-windowing/softbuffer/tree/d871852faa1137d4615b99b9ad31c3ba80f345b5
The MIT and Apache-2.0 notices are unmodified. Cargo registry state, the
standalone lockfile and CI metadata are omitted. Cargo.toml.orig is the
original manifest; Cargo.toml is the active patched manifest.

The macOS backend keeps the upstream public dispatch, buffer ownership and
RGB pixel contract. It supplies a main-thread-only AppKit surface and uses
a no-copy CGDataProvider with a Rust slice-owner release callback. This
intentionally sacrifices the prior Send guarantee for macOS surfaces and
buffers; there is no unsafe Send override. The native shim retains the view,
owns its own CALayer and removes it before the host window is released.
Core Graphics owns each submitted buffer until it releases the data
provider; the caller does not touch those pixels again. The existing non-macOS
backend source is unchanged. Nothing in this fork copies or generates Apple
SDK binding source: native compilation uses installed SDK public interfaces
and the package links the system frameworks without bundling SDK files.

Public API references:
- https://developer.apple.com/documentation/appkit/nsview
- https://developer.apple.com/documentation/appkit/nswindow
- https://developer.apple.com/documentation/appkit/nsapplication
- https://developer.apple.com/documentation/appkit/nsevent
- https://developer.apple.com/documentation/quartzcore/calayer
- https://developer.apple.com/documentation/quartzcore/catransaction
- https://developer.apple.com/documentation/coregraphics/cgdataprovider
- https://developer.apple.com/documentation/coregraphics/cgimage

Pong uses AppKit instead of Winit only on macOS. The obsolete standalone
Dispatch2 fixture still has its separate historical development dependencies;
the macOS release binary does not link them. This is a prototype, not a
release clearance or a claim that all builds with the fork have these properties.
