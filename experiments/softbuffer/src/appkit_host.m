// Handwritten bridge to the documented AppKit runtime; no SDK source vendored.
#import <AppKit/AppKit.h>
#ifdef PONG_APPKIT_QA
#import <QuartzCore/QuartzCore.h>
#endif
#include <stdint.h>

@interface PongSoftbufferHost : NSObject <NSWindowDelegate>
@property(strong) NSWindow *window;
@property BOOL closing;
@property BOOL blurred;
@end
@implementation PongSoftbufferHost
- (BOOL)windowShouldClose:(NSWindow *)sender {
    self.closing = YES;
    return NO; // Rust disposes the surface before we dispose its native view.
}
- (void)windowDidResignKey:(NSNotification *)note { self.blurred = YES; }
@end

void *pong_host_new(void) {
    if (![NSThread isMainThread]) return NULL;
    @autoreleasepool {
        [NSApplication sharedApplication];
        [NSApp setActivationPolicy:NSApplicationActivationPolicyRegular];
        [NSApp finishLaunching];
        PongSoftbufferHost *host = [PongSoftbufferHost new];
        host.window = [[NSWindow alloc] initWithContentRect:NSMakeRect(0, 0, 900, 500)
            styleMask:NSWindowStyleMaskTitled | NSWindowStyleMaskClosable |
                      NSWindowStyleMaskResizable | NSWindowStyleMaskMiniaturizable
            backing:NSBackingStoreBuffered defer:NO];
        host.window.releasedWhenClosed = NO;
        host.window.delegate = host;
        host.window.title = @"Pong Softbuffer (AppKit)";
        [host.window center];
        [host.window makeKeyAndOrderFront:nil];
        [NSApp activateIgnoringOtherApps:YES];
        return (__bridge_retained void *)host;
    }
}
void pong_host_free(void *ptr) {
    if (![NSThread isMainThread]) abort();
    @autoreleasepool {
        PongSoftbufferHost *host = (__bridge_transfer PongSoftbufferHost *)ptr;
        host.window.delegate = nil;
        [host.window close];
    }
}
void *pong_host_view(void *ptr) {
    @autoreleasepool {
        PongSoftbufferHost *host = (__bridge PongSoftbufferHost *)ptr;
        return (__bridge void *)host.window.contentView;
    }
}
void pong_host_title(void *ptr, const char *title) {
    @autoreleasepool {
        PongSoftbufferHost *host = (__bridge PongSoftbufferHost *)ptr;
        host.window.title = [NSString stringWithUTF8String:title];
    }
}
void pong_host_metrics(void *ptr, uint32_t *width, uint32_t *height, double *scale) {
    @autoreleasepool {
        PongSoftbufferHost *host = (__bridge PongSoftbufferHost *)ptr;
        NSView *view = host.window.contentView;
        NSRect px = [view convertRectToBacking:view.bounds];
        *width = (uint32_t)fmax(0, round(px.size.width));
        *height = (uint32_t)fmax(0, round(px.size.height));
        *scale = host.window.backingScaleFactor;
    }
}

typedef struct {
    uint32_t action; // 0 idle, 1 press, 2 release, 3 close, 4 events forwarded
    uint32_t key;
    uint32_t repeat;
    uint32_t focused;
    uint32_t lost_focus;
} PongHostEvent;

static uint32_t pong_key(NSEvent *e) {
    // macOS virtual hardware key identifiers, not locale-dependent text.
    switch (e.keyCode) {
        case 13: return 1; // W
        case 126: return 2; // Up
        case 1: return 3; // S
        case 125: return 4; // Down
        case 49: return 5; // Space
        case 15: return 6; // R
        case 53: return 7; // Escape
        default: return 0;
    }
}

void pong_host_poll(void *ptr, double seconds, PongHostEvent *result) {
    PongSoftbufferHost *host = (__bridge PongSoftbufferHost *)ptr;
    @autoreleasepool {
        *result = (PongHostEvent){0};
        NSDate *deadline = [NSDate dateWithTimeIntervalSinceNow:seconds];
        NSEvent *event = [NSApp nextEventMatchingMask:NSEventMaskAny
             untilDate:deadline inMode:NSDefaultRunLoopMode dequeue:YES];
        if (event && !host.closing) {
            BOOL isDown = event.type == NSEventTypeKeyDown;
            BOOL isUp = event.type == NSEventTypeKeyUp;
            BOOL shortcut = (event.modifierFlags &
                (NSEventModifierFlagCommand | NSEventModifierFlagControl)) != 0;
            if ((isDown || isUp) && event.window == host.window && host.window.isKeyWindow) {
                uint32_t mapped = pong_key(event);
                // Preserve AppKit shortcuts. A modified movement key-up
                // must still clear the matching physical hold, but a
                // shortcut key-down never starts or repeats gameplay.
                if (!shortcut || (isUp && mapped >= 1 && mapped <= 4)) {
                    result->key = mapped;
                }
            }
            if (!result->key || shortcut) [NSApp sendEvent:event];
            if (result->key) {
                result->action = isDown ? 1 : 2;
                result->repeat = isDown && event.isARepeat;
            } else {
                result->action = 4;
            }
        }
        if (host.closing) result->action = 3;
        result->focused = host.window.isKeyWindow;
        result->lost_focus = host.blurred;
        host.blurred = NO;
        [NSApp updateWindows];
    }
}

#ifdef PONG_APPKIT_QA
/* App-owned native checks; never compiled into the release artifact. */
void pong_host_qa_resize(void *ptr, unsigned width, unsigned height) {
    @autoreleasepool {
        PongSoftbufferHost *host = (__bridge PongSoftbufferHost *)ptr;
        [host.window setContentSize:NSMakeSize(width, height)];
        [host.window.contentView layoutSubtreeIfNeeded];
    }
}
void pong_host_qa_focus(void *ptr, int focused) {
    @autoreleasepool {
        PongSoftbufferHost *host = (__bridge PongSoftbufferHost *)ptr;
        if (focused) [host.window makeKeyAndOrderFront:nil];
        else [host.window orderOut:nil];
    }
}
void pong_host_qa_key(void *ptr, unsigned key, int down, int repeat, unsigned modifiers) {
    @autoreleasepool {
        PongSoftbufferHost *host = (__bridge PongSoftbufferHost *)ptr;
        NSEvent *event = [NSEvent keyEventWithType:down ? NSEventTypeKeyDown : NSEventTypeKeyUp
            location:NSZeroPoint
            modifierFlags:(modifiers & 1 ? NSEventModifierFlagCommand : 0) |
                          (modifiers & 2 ? NSEventModifierFlagControl : 0)
            timestamp:NSProcessInfo.processInfo.systemUptime
            windowNumber:host.window.windowNumber context:nil characters:@"w"
            charactersIgnoringModifiers:@"w" isARepeat:repeat keyCode:key];
        [NSApp postEvent:event atStart:YES];
    }
}
int pong_host_qa_pixels(void *ptr, unsigned width, unsigned height, uint32_t *samples,
                        double *layerScale) {
    @autoreleasepool {
        PongSoftbufferHost *host = (__bridge PongSoftbufferHost *)ptr;
        [CATransaction flush];
        CALayer *layer = host.window.contentView.layer.sublayers.lastObject;
        *layerScale = layer.contentsScale;
        CGImageRef image = (__bridge CGImageRef)layer.contents;
        if (!image || CGImageGetWidth(image) != width || CGImageGetHeight(image) != height ||
            layer.bounds.size.width <= 0 || layer.bounds.size.height <= 0) return 0;
        // Real app-owned layer readback, not the Rust buffer before present.
        // Quartz bitmap rows are top-to-bottom; draw the full layer scaled to
        // four pixels each way and inspect the four asymmetric corner samples.
        uint32_t pixels[16] = {0};
        CGColorSpaceRef space = CGColorSpaceCreateDeviceRGB();
        CGContextRef ctx = CGBitmapContextCreate(pixels, 4, 4, 8, 16, space,
                             kCGBitmapByteOrder32Little | kCGImageAlphaPremultipliedFirst);
        CGColorSpaceRelease(space);
        if (!ctx) return 0;
        CGContextSetInterpolationQuality(ctx, kCGInterpolationNone);
        CGContextScaleCTM(ctx, 4.0 / layer.bounds.size.width, 4.0 / layer.bounds.size.height);
        [layer renderInContext:ctx];
        CGContextRelease(ctx);
        samples[0] = pixels[0] & 0x00ffffff;
        samples[1] = pixels[3] & 0x00ffffff;
        samples[2] = pixels[12] & 0x00ffffff;
        samples[3] = pixels[15] & 0x00ffffff;
        return 1;
    }
}
void pong_host_qa_close(void *ptr) {
    @autoreleasepool {
        PongSoftbufferHost *host = (__bridge PongSoftbufferHost *)ptr;
        [host.window performClose:nil];
    }
}
#endif
