//! Owned buffer produced by the Objective-C C runtime, not Rust's allocator.
use std::ffi::c_void;
use std::mem;
use std::ops::Deref;
use std::ptr::NonNull;
use std::slice;

extern "C" {
    fn free(ptr: *mut c_void);
}

/// An owned C allocation containing plain copyable values.
pub struct COwnedSlice<T: Copy> {
    ptr: *mut T,
    len: usize,
}

impl<T: Copy> COwnedSlice<T> {
    /// Takes ownership of a buffer from the C allocator.
    ///
    /// # Safety
    /// A non-null pointer must be the original live C allocation, which is
    /// freed once by this owner, even if validation panics. For a nonempty
    /// slice the first `len` values must be initialized, valid for `T`, and
    /// not mutated for the lifetime of a borrow. Bounds checks do not prove
    /// the actual allocation size or the validity of its elements.
    pub(crate) unsafe fn from_raw(ptr: *mut T, len: usize) -> Self {
        let value = Self { ptr, len };
        Self::validate(ptr, len);
        value
    }

    fn validate(ptr: *mut T, len: usize) {
        if len != 0 {
            assert!(!ptr.is_null(), "null C buffer with nonzero length");
            assert_eq!(
                (ptr as usize) % mem::align_of::<T>(),
                0,
                "unaligned C buffer"
            );
            let bytes = len.checked_mul(mem::size_of::<T>());
            assert!(
                matches!(bytes, Some(n) if n <= isize::MAX as usize),
                "C buffer length exceeds slice limits"
            );
        }
    }
}

impl<T: Copy> Deref for COwnedSlice<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        // Rust requires a non-null aligned pointer even for an empty slice.
        let ptr = if self.len == 0 {
            NonNull::dangling()
        } else {
            // from_raw validated the nonempty pointer before returning.
            unsafe { NonNull::new_unchecked(self.ptr) }
        };
        unsafe { slice::from_raw_parts(ptr.as_ptr(), self.len) }
    }
}

impl<T: Copy> Drop for COwnedSlice<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { free(self.ptr.cast()) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::COwnedSlice;
    use std::{ffi::c_void, mem, ptr};

    extern "C" {
        fn malloc(n: usize) -> *mut c_void;
    }

    #[test]
    fn null_empty_slice_is_aligned() {
        let value = unsafe { COwnedSlice::<u64>::from_raw(ptr::null_mut(), 0) };
        assert!(value.is_empty());
        assert_eq!(value.as_ptr() as usize % mem::align_of::<u64>(), 0);
    }

    #[test]
    fn reads_aligned_c_allocated_array() {
        let p = unsafe { malloc(mem::size_of::<[u64; 2]>()).cast::<[u64; 2]>() };
        assert!(!p.is_null());
        unsafe {
            p.write([12, 34]);
        }
        let value = unsafe { COwnedSlice::from_raw(p.cast::<u64>(), 2) };
        assert_eq!(&*value, &[12, 34]);
    }

    #[test]
    #[should_panic(expected = "null C buffer with nonzero length")]
    fn null_nonempty_rejected() {
        let _ = unsafe { COwnedSlice::<u64>::from_raw(ptr::null_mut(), 1) };
    }

    #[test]
    #[should_panic(expected = "unaligned C buffer")]
    fn unaligned_nonempty_rejected() {
        // No ownership of the intentionally invalid pointer is assumed.
        COwnedSlice::<u64>::validate(1_usize as *mut u64, 1);
    }

    #[test]
    #[should_panic(expected = "C buffer length exceeds slice limits")]
    fn oversized_nonempty_rejected() {
        COwnedSlice::<u64>::validate(
            std::ptr::NonNull::<u64>::dangling().as_ptr(),
            (isize::MAX as usize / mem::size_of::<u64>()) + 1,
        );
    }

    #[test]
    #[should_panic(expected = "C buffer length exceeds slice limits")]
    fn byte_length_overflow_rejected() {
        COwnedSlice::<u64>::validate(std::ptr::NonNull::<u64>::dangling().as_ptr(), usize::MAX);
    }
}
