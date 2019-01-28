use std::mem;
use std::sync::{Arc, Mutex, Once, ONCE_INIT};

#[derive(Clone)]
pub struct Counter {
    // Since we will be used in many threads, we need to protect
    // concurrent access
    pub inner: Arc<Mutex<u8>>,
}

pub fn singleton() -> Counter {
    // Initialize it to a null value
    static mut COUNTER: *const Counter = 0 as *const Counter;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = Counter {
                inner: Arc::new(Mutex::new(0)),
            };

            // Put it in the heap so it can outlive this call
            COUNTER = mem::transmute(Box::new(singleton));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*COUNTER).clone()
    }
}
