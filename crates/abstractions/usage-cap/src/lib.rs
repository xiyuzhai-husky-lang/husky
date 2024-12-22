mod count_down;
pub mod error;

use self::{count_down::UsageCountDown, error::UsageCapError};
use error::LlmCapResult;
use lazy_static::lazy_static;
use std::sync::Mutex;

pub struct UsageCap {
    count_down: std::sync::Mutex<UsageCountDown>,
    post_call_usage_multiplier: usize,
}

impl UsageCap {
    pub fn new(
        entity_name: &'static str,
        var_name: &'static str,
        post_call_usage_multiplier: usize,
    ) -> Self {
        Self {
            count_down: std::sync::Mutex::new(UsageCountDown::new(entity_name, var_name)),
            post_call_usage_multiplier,
        }
    }

    fn remaining_count(&self) -> Option<usize> {
        self.count_down.lock().unwrap().remaining_count()
    }
}

impl UsageCap {
    pub fn try_use<R>(
        &self,
        min_usage: usize,
        f: impl FnOnce() -> (usize, R),
    ) -> LlmCapResult<Result<R, (R, UsageCapError)>> {
        use husky_print_utils::p;
        let mut count_down = self.count_down.lock().unwrap();
        count_down.try_count_down(min_usage)?;
        let (additional_usage, r) = f();
        let post_call =
            count_down.try_count_down(additional_usage * self.post_call_usage_multiplier);
        Ok(match post_call {
            Ok(_) => Ok(r),
            Err(e) => Err((r, e)),
        })
    }
}

#[test]
fn llm_cap_works() {
    let _lock = ENV_TEST_MUTEX.lock().unwrap();

    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    let entity_name = "MimicAI";
    let var_name = "ENABLE_MIMIC_AI_API_CALLING";

    // Create a counter to track number of function calls
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    // Set environment variable for testing
    let original_var = std::env::var(var_name).ok();
    std::env::set_var(var_name, "3");

    let cap = UsageCap::new(entity_name, var_name, 2);

    // Create a test function that increments the counter
    let test_fn = move || {
        let r = counter_clone.fetch_add(1, Ordering::SeqCst);
        (0, r)
    };

    let expected_remaining_count = vec![Some(2), Some(1), Some(0), Some(0), Some(0)];

    assert_eq!(cap.remaining_count(), Some(3));

    // Try calling the function 5 times (should only work 3 times)
    for i in 0..5 {
        let result = cap.try_use(1, &test_fn);
        match result {
            Ok(_) => (),
            Err(e) => {
                if !e.to_string().contains("final count down") {
                    panic!("Unexpected error: {}", e);
                }
            }
        }
        assert_eq!(cap.remaining_count(), expected_remaining_count[i]);
    }

    // Verify that the function was only called 3 times
    assert_eq!(counter.load(Ordering::SeqCst), 3);

    // Restore original environment state
    if let Some(original) = original_var {
        std::env::set_var(var_name, original);
    } else {
        std::env::remove_var(var_name);
    }
}

#[cfg(test)]
lazy_static! {
    static ref ENV_TEST_MUTEX: Mutex<()> = Mutex::new(());
}
