// Pong-specific AppKit backend. All public framework types are used via the
// ordinary SDK compiler includes; no translated or generated bindings.
#import <AppKit/AppKit.h>
#import <QuartzCore/QuartzCore.h>
#import <CoreGraphics/CoreGraphics.h>
#include <stdint.h>

@interface PongSoftbufferLayer : NSObject
@property(strong) NSView *view;
@property(strong) CALayer *pixels;
@end
@implementation PongSoftbufferLayer
@end

void *pong_sb_surface_new(void *nsview) {
    @autoreleasepool {
        if (![NSThread isMainThread] || !nsview) return NULL;
        PongSoftbufferLayer *surface = [PongSoftbufferLayer new];
        surface.view = (__bridge NSView *)nsview;
        surface.view.wantsLayer = YES;
        if (!surface.view.layer) return NULL;
        surface.pixels = [CALayer layer];
        surface.pixels.contentsGravity = kCAGravityTopLeft;
        surface.pixels.magnificationFilter = kCAFilterNearest;
        [surface.view.layer addSublayer:surface.pixels];
        return (__bridge_retained void *)surface;
    }
}

void pong_sb_surface_free(void *ptr) {
    @autoreleasepool {
        // The Rust owner is not Send. Guard against ABI misuse rather than
        // touching an AppKit object on another thread.
        if (![NSThread isMainThread]) abort();
        PongSoftbufferLayer *surface = (__bridge_transfer PongSoftbufferLayer *)ptr;
        surface.pixels.contents = nil;
        [surface.pixels removeFromSuperlayer];
    }
}

typedef void (*ReleasePixels)(void *, const void *, size_t);
int pong_sb_surface_present(void *ptr, const void *data,
    size_t width, size_t height, size_t bytes, ReleasePixels release) {
    // This entry point always takes ownership of the incoming allocation.
    if (![NSThread isMainThread]) {
        release(NULL, data, bytes);
        return 0;
    }
    @autoreleasepool {
        PongSoftbufferLayer *surface = (__bridge PongSoftbufferLayer *)ptr;
        CGDataProviderRef provider = CGDataProviderCreateWithData(NULL, data, bytes, release);
        if (!provider) { release(NULL, data, bytes); return 0; }
        CGColorSpaceRef color = CGColorSpaceCreateDeviceRGB();
        CGImageRef image = color ? CGImageCreate(width, height, 8, 32,
            width * sizeof(uint32_t), color, kCGBitmapByteOrder32Little | kCGImageAlphaNoneSkipFirst,
            provider, NULL, false, kCGRenderingIntentDefault) : NULL;
        if (color) CGColorSpaceRelease(color);
        CGDataProviderRelease(provider);
        if (!image) return 0;

        [CATransaction begin];
        [CATransaction setDisableActions:YES];
        surface.pixels.contentsScale = surface.view.window.backingScaleFactor ?: 1;
        surface.pixels.frame = surface.view.layer.bounds;
        surface.pixels.contents = (__bridge id)image;
        [CATransaction commit];
        CGImageRelease(image);
        return 1;
    }
}
