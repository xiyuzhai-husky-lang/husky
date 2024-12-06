mod count_down;
pub mod error;

use self::{count_down::LlmCountDown, error::LlmLockError};
use lazy_static::lazy_static;

pub struct LlmLock {
    lock: std::sync::Mutex<LlmCountDown>,
}

impl LlmLock {
    pub fn new() -> Self {
        Self {
            lock: std::sync::Mutex::new(LlmCountDown::new()),
        }
    }
}

pub fn try_call_llm<R>(f: impl FnOnce() -> R) -> Result<R, Box<dyn std::error::Error>> {
    lazy_static! {
        static ref LLM_LOCK: LlmLock = LlmLock::new();
    }

    LLM_LOCK.try_call_llm(f)
}

impl LlmLock {
    pub fn try_call_llm<R>(&self, f: impl FnOnce() -> R) -> Result<R, Box<dyn std::error::Error>> {
        let mut lock = self.lock.lock().unwrap();
        lock.try_count_down()?;
        Ok(f())
    }
}

#[test]
fn llm_lock_works() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    // Create a counter to track number of function calls
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    // Set environment variable for testing
    let original_var = std::env::var("ENABLE_LLM_API_CALLING").ok();
    std::env::set_var("ENABLE_LLM_API_CALLING", "3");

    // Create a test function that increments the counter
    let test_fn = move || {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    };

    // Try calling the function 5 times (should only work 3 times)
    for _ in 0..5 {
        let result = try_call_llm(&test_fn);
        match result {
            Ok(_) => (),
            Err(e) => {
                if !e.to_string().contains("final count down") {
                    panic!("Unexpected error: {}", e);
                }
            }
        }
    }

    // Verify that the function was only called 3 times
    assert_eq!(counter.load(Ordering::SeqCst), 3);

    // Restore original environment state
    if let Some(original) = original_var {
        std::env::set_var("ENABLE_LLM_API_CALLING", original);
    } else {
        std::env::remove_var("ENABLE_LLM_API_CALLING");
    }
}
