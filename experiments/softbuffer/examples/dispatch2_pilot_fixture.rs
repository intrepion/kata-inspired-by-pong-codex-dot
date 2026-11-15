#[cfg(target_os = "macos")]
mod macos {
    use core_foundation::runloop::CFRunLoop;
    use objc2_foundation::{is_main_thread, run_on_main, MainThreadBound, MainThreadMarker};
    use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
    
    struct DropProbe(Arc<AtomicBool>);
    impl Drop for DropProbe {
        fn drop(&mut self) {
            assert!(is_main_thread());
            self.0.store(true, Ordering::SeqCst);
        }
    }
    
    pub fn run() {
        let main = MainThreadMarker::new().expect("fixture must start on the main thread");
        let direct = run_on_main(|_| { assert!(is_main_thread()); String::from("direct") });
        assert_eq!(direct, "direct");
        if std::env::args().any(|arg| arg == "panic") {
            std::thread::spawn(|| run_on_main(|_| panic!("intentional queued panic")));
            loop { CFRunLoop::run_current(); }
        }
        let destroyed = Arc::new(AtomicBool::new(false));
        let bound = MainThreadBound::new(DropProbe(destroyed.clone()), main);
        let done = Arc::new(AtomicBool::new(false));
        let finished = done.clone();
        let runloop = CFRunLoop::get_main();
        std::thread::spawn(move || {
            let mut borrowed = String::from("borrowed");
            let value = run_on_main(|_| {
                assert!(is_main_thread());
                borrowed.push_str("-changed");
                format!("noncopy:{borrowed}")
            });
            assert_eq!(borrowed, "borrowed-changed");
            assert_eq!(value, "noncopy:borrowed-changed");
            drop(bound);
            assert!(destroyed.load(Ordering::SeqCst));
            finished.store(true, Ordering::SeqCst);
            runloop.stop();
        });
        while !done.load(Ordering::SeqCst) { CFRunLoop::run_current(); }
        println!("PASS direct, queued borrowed return, marker, off-main drop");
    }
}

#[cfg(target_os = "macos")]
fn main() { macos::run(); }

#[cfg(not(target_os = "macos"))]
fn main() { eprintln!("This fixture requires macOS"); }
