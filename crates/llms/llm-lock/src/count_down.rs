use crate::error::LlmLockError;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[cfg(test)]
lazy_static! {
    static ref ENV_TEST_MUTEX: Mutex<()> = Mutex::new(());
}

pub struct LlmCountDown {
    /// `None` if the environment variable `ENABLE_LLM_API_CALLING` is not set.
    count: Option<u32>,
}

impl LlmCountDown {
    pub fn new() -> Self {
        let count = std::env::var("ENABLE_LLM_API_CALLING")
            .map(|v| {
                v.parse::<u32>()
                    .expect("Invalid value for ENABLE_LLM_API_CALLING, must be a number")
            })
            .ok();
        Self { count }
    }
}

impl LlmCountDown {
    pub fn try_count_down(&mut self) -> Result<(), LlmLockError> {
        match self.count {
            Some(ref mut count) => {
                if *count == 0 {
                    Err(LlmLockError::FinalCountDown)
                } else {
                    *count -= 1;
                    Ok(())
                }
            }
            None => Err(LlmLockError::LlmApiCallingDisabled),
        }
    }
}

#[test]
fn llm_count_down_works() {
    let _lock = ENV_TEST_MUTEX.lock().unwrap();

    // Save original env var state if it exists
    let original_var = std::env::var("ENABLE_LLM_API_CALLING").ok();

    // Test initialization without env var
    let count_down = LlmCountDown::new();
    assert!(count_down.count.is_none());

    // Set environment variable for remaining tests
    std::env::set_var("ENABLE_LLM_API_CALLING", "5");

    // Test initialization with env var
    let mut count_down = LlmCountDown::new();
    assert_eq!(count_down.count, Some(5));

    // Test successful countdown
    assert!(count_down.try_count_down().is_ok());
    assert_eq!(count_down.count, Some(4));

    // Count down to 0
    assert!(count_down.try_count_down().is_ok()); // 3
    assert!(count_down.try_count_down().is_ok()); // 2
    assert!(count_down.try_count_down().is_ok()); // 1
    assert!(count_down.try_count_down().is_ok()); // 0

    // Test error when count is 0
    match count_down.try_count_down() {
        Err(LlmLockError::FinalCountDown) => (),
        _ => panic!("Expected FinalCountDown error"),
    }

    // Restore original environment state
    if let Some(original) = original_var {
        std::env::set_var("ENABLE_LLM_API_CALLING", original);
    } else {
        std::env::remove_var("ENABLE_LLM_API_CALLING");
    }
}

#[test]
fn llm_count_down_invalid_env_var() {
    let _lock = ENV_TEST_MUTEX.lock().unwrap();

    // Save original env var state if it exists
    let original_var = std::env::var("ENABLE_LLM_API_CALLING").ok();

    std::env::set_var("ENABLE_LLM_API_CALLING", "not_a_number");
    let result = std::panic::catch_unwind(|| LlmCountDown::new());
    assert!(result.is_err());

    // Restore original environment state
    if let Some(original) = original_var {
        std::env::set_var("ENABLE_LLM_API_CALLING", original);
    } else {
        std::env::remove_var("ENABLE_LLM_API_CALLING");
    }
}
