//! Mac-only public AppKit/Core Graphics shim. Keeps Softbuffer's public
//! dispatch, borrowed buffer and 0x00RRGGBB pixel contract.
use crate::{backend_interface::*, error::InitError, util, Rect, SoftBufferError};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawWindowHandle};
use std::{
    ffi::{c_int, c_void},
    marker::PhantomData,
    num::NonZeroU32,
    ptr::{self, NonNull},
    rc::Rc,
};

extern "C" {
    fn pong_sb_surface_new(view: *mut c_void) -> *mut c_void;
    fn pong_sb_surface_free(surface: *mut c_void);
    fn pong_sb_surface_present(
        surface: *mut c_void,
        data: *const c_void,
        w: usize,
        h: usize,
        bytes: usize,
        release: unsafe extern "C" fn(*mut c_void, *const c_void, usize),
    ) -> c_int;
}

fn error(reason: &str) -> SoftBufferError {
    SoftBufferError::PlatformError(Some(reason.into()), None)
}

fn pixel_count(w: NonZeroU32, h: NonZeroU32) -> Result<usize, SoftBufferError> {
    let count = (w.get() as usize).checked_mul(h.get() as usize);
    count
        .filter(|n| *n <= isize::MAX as usize / size_of::<u32>())
        .ok_or_else(|| error("AppKit buffer exceeds addressable memory"))
}

#[derive(Debug)]
pub struct CGImpl<D, W> {
    native: NonNull<c_void>,
    size: Option<(NonZeroU32, NonZeroU32)>,
    window: W,
    _main_thread: PhantomData<(D, Rc<()>)>,
}
impl<D, W> Drop for CGImpl<D, W> {
    fn drop(&mut self) {
        // A valid native owner is only constructed on the main thread, is
        // not Send, and is freed before the owned W handle is dropped.
        unsafe { pong_sb_surface_free(self.native.as_ptr()) }
    }
}
impl<D: HasDisplayHandle, W: HasWindowHandle> SurfaceInterface<D, W> for CGImpl<D, W> {
    type Context = D;
    type Buffer<'a>
        = BufferImpl<'a, D, W>
    where
        Self: 'a;

    fn new(window: W, _display: &D) -> Result<Self, InitError<W>> {
        let raw = window.window_handle()?.as_raw();
        let RawWindowHandle::AppKit(view) = raw else {
            return Err(InitError::Unsupported(window));
        };
        let native = NonNull::new(unsafe { pong_sb_surface_new(view.ns_view.as_ptr()) })
            .ok_or_else(|| error("could not create a main-thread AppKit pixel layer"))?;
        Ok(Self {
            native,
            size: None,
            window,
            _main_thread: PhantomData,
        })
    }
    fn window(&self) -> &W {
        &self.window
    }
    fn resize(&mut self, w: NonZeroU32, h: NonZeroU32) -> Result<(), SoftBufferError> {
        pixel_count(w, h)?;
        self.size = Some((w, h));
        Ok(())
    }
    fn buffer_mut(&mut self) -> Result<Self::Buffer<'_>, SoftBufferError> {
        let (w, h) = self
            .size
            .ok_or_else(|| error("resize the AppKit surface first"))?;
        let count = pixel_count(w, h)?;
        let mut pixels = Vec::new();
        pixels
            .try_reserve_exact(count)
            .map_err(|_| error("could not allocate AppKit pixels"))?;
        pixels.resize(count, 0);
        Ok(BufferImpl {
            native: self.native,
            width: w,
            height: h,
            buffer: util::PixelBuffer(pixels),
            _borrow: PhantomData,
        })
    }
}

#[derive(Debug)]
pub struct BufferImpl<'a, D, W> {
    native: NonNull<c_void>,
    width: NonZeroU32,
    height: NonZeroU32,
    buffer: util::PixelBuffer,
    _borrow: PhantomData<&'a mut CGImpl<D, W>>,
}
impl<D: HasDisplayHandle, W: HasWindowHandle> BufferInterface for BufferImpl<'_, D, W> {
    fn width(&self) -> NonZeroU32 {
        self.width
    }
    fn height(&self) -> NonZeroU32 {
        self.height
    }
    fn pixels(&self) -> &[u32] {
        &self.buffer
    }
    fn pixels_mut(&mut self) -> &mut [u32] {
        &mut self.buffer
    }
    fn age(&self) -> u8 {
        0
    }
    fn present(self) -> Result<(), SoftBufferError> {
        unsafe extern "C" fn release(_cookie: *mut c_void, data: *const c_void, bytes: usize) {
            let buffer = ptr::slice_from_raw_parts_mut(
                data.cast_mut().cast::<u32>(),
                bytes / size_of::<u32>(),
            );
            // Only called with the original nonempty boxed slice.
            unsafe {
                drop(Box::from_raw(buffer));
            }
        }
        let pixels = self.buffer.0.into_boxed_slice();
        let bytes = pixels.len() * size_of::<u32>();
        let data = Box::into_raw(pixels).cast::<c_void>();
        if unsafe {
            pong_sb_surface_present(
                self.native.as_ptr(),
                data,
                self.width.get() as usize,
                self.height.get() as usize,
                bytes,
                release,
            )
        } != 1
        {
            return Err(error("AppKit rejected the pixel frame"));
        }
        Ok(())
    }
    fn present_with_damage(self, _damage: &[Rect]) -> Result<(), SoftBufferError> {
        self.present()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_unaddressable_frame() {
        assert!(pixel_count(NonZeroU32::MAX, NonZeroU32::MAX).is_err());
    }
}
