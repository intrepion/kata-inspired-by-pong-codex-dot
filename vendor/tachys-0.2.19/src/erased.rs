use std::any::Any;

macro_rules! erased {
    ([$($new_t_params:tt)*], $name:ident, $storage:ty) => {
        /// A type-erased item with checked recovery of its concrete type.
        pub struct $name {
            value: Box<$storage>,
        }

        impl $name {
            /// Create a new type-erased item.
            pub fn new<T: $($new_t_params)*>(item: T) -> Self {
                Self { value: Box::new(item) }
            }

            /// Get a reference to the inner value.
            ///
            /// Panics if `T` is not the original value's type.
            pub fn get_ref<T: 'static>(&self) -> &T {
                self.value.downcast_ref::<T>().unwrap_or_else(|| panic!("Erased: type mismatch"))
            }

            /// Get a mutable reference to the inner value.
            ///
            /// Panics if `T` is not the original value's type.
            pub fn get_mut<T: 'static>(&mut self) -> &mut T {
                self.value.downcast_mut::<T>().unwrap_or_else(|| panic!("Erased: type mismatch"))
            }

            /// Consume the item and return the inner value.
            ///
            /// Panics if `T` is not the original value's type.
            pub fn into_inner<T: 'static>(self) -> T {
                match self.value.downcast::<T>() {
                    Ok(value) => *value,
                    Err(_) => panic!("Erased: type mismatch"),
                }
            }
        }
    };
}

erased!([Send + 'static], Erased, dyn Any + Send);
erased!(['static], ErasedLocal, dyn Any);

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        cell::Cell,
        panic::{AssertUnwindSafe, catch_unwind},
        rc::Rc,
        sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        },
    };

    #[test]
    fn sendable_value_can_be_read_mutated_and_moved_between_threads() {
        let mut value = Erased::new(String::from("pong"));
        assert_eq!(value.get_ref::<String>(), "pong");
        value.get_mut::<String>().push('!');
        assert_eq!(
            std::thread::spawn(move || value.into_inner::<String>())
                .join()
                .unwrap(),
            "pong!",
        );
    }

    #[test]
    fn local_value_can_hold_non_send_state_and_be_retrieved() {
        let state = Rc::new(Cell::new(1));
        let mut value = ErasedLocal::new(Rc::clone(&state));
        assert!(Rc::ptr_eq(value.get_ref::<Rc<Cell<i32>>>(), &state));
        value.get_mut::<Rc<Cell<i32>>>().set(2);
        let extracted = value.into_inner::<Rc<Cell<i32>>>();
        assert_eq!(extracted.get(), 2);
        assert_eq!(Rc::strong_count(&state), 2);
    }

    macro_rules! mismatch_tests {
        ($test:ident, $ty:ident) => {
            #[test]
            fn $test() {
                let mut value = $ty::new(1_u8);
                let read = catch_unwind(AssertUnwindSafe(|| *value.get_ref::<i8>()));
                let write = catch_unwind(AssertUnwindSafe(|| *value.get_mut::<i8>() = 2));
                assert_eq!(*value.get_ref::<u8>(), 1);
                let take = catch_unwind(AssertUnwindSafe(|| value.into_inner::<i8>()));
                for result in [read.map(|_| ()), write, take.map(|_| ())] {
                    assert_eq!(
                        result.unwrap_err().downcast_ref::<&str>(),
                        Some(&"Erased: type mismatch"),
                    );
                }
            }
        };
    }
    mismatch_tests!(sendable_mismatch_panics, Erased);
    mismatch_tests!(local_mismatch_panics, ErasedLocal);

    struct CountDrop(Arc<AtomicUsize>);
    impl Drop for CountDrop {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    macro_rules! drop_tests {
        ($test:ident, $ty:ident) => {
            #[test]
            fn $test() {
                let count = Arc::new(AtomicUsize::new(0));
                let make = || $ty::new(CountDrop(Arc::clone(&count)));
                drop(make());
                assert_eq!(count.load(Ordering::Relaxed), 1);
                let inner = make().into_inner::<CountDrop>();
                assert_eq!(count.load(Ordering::Relaxed), 1);
                drop(inner);
                assert_eq!(count.load(Ordering::Relaxed), 2);
                assert!(catch_unwind(AssertUnwindSafe(|| make().into_inner::<()>())).is_err());
                assert_eq!(count.load(Ordering::Relaxed), 3);
            }
        };
    }
    drop_tests!(sendable_drops_exactly_once, Erased);
    drop_tests!(local_drops_exactly_once, ErasedLocal);
}
