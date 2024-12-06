use crate::error::{LlmCapError, LlmCapResult};
use crate::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

#[derive(Debug)]
pub struct LlmCountDown {
    entity_name: &'static str,
    /// `None` if the environment variable `ENABLE_LLM_API_CALLING` is not set.
    remaining_count: Option<usize>,
}

impl LlmCountDown {
    pub fn new(entity_name: &'static str, var_name: &'static str) -> Self {
        let maximal_count = std::env::var(var_name)
            .map(|v| {
                v.parse::<usize>()
                    .expect(&format!("Invalid value for {var_name}, must be a number"))
            })
            .ok();
        Self {
            entity_name,
            remaining_count: maximal_count,
        }
    }
}

impl LlmCountDown {
    pub fn remaining_count(&self) -> Option<usize> {
        self.remaining_count
    }
}

impl LlmCountDown {
    pub fn try_count_down(&mut self, usage: usize) -> LlmCapResult<()> {
        match self.remaining_count {
            Some(ref mut count) => {
                if *count == 0 {
                    if usage > 0 {
                        Err(LlmCapError::FinalCountDown)
                    } else {
                        Ok(())
                    }
                } else {
                    *count -= usage;
                    Ok(())
                }
            }
            None => Err(LlmCapError::LlmApiCallingDisabled(self.entity_name)),
        }
    }
}

#[test]
fn llm_count_down_works() {
    let _lock = ENV_TEST_MUTEX.lock().unwrap();

    let name = "MIMIC_AI";

    let var_name = "ENABLE_LLM_API_CALLING";

    // Save original env var state if it exists
    let original_var = std::env::var(var_name).ok();

    // Test initialization without env var
    let count_down = LlmCountDown::new("MIMIC_AI", var_name);
    assert!(count_down.remaining_count.is_none());

    // Set environment variable for remaining tests
    std::env::set_var(var_name, "5");

    // Test initialization with env var
    let mut count_down = LlmCountDown::new("MIMIC_AI", var_name);
    assert_eq!(count_down.remaining_count, Some(5));

    // Test successful countdown
    assert!(count_down.try_count_down(1).is_ok());
    assert_eq!(count_down.remaining_count, Some(4));

    // Count down to 0
    assert!(count_down.try_count_down(1).is_ok()); // 3
    assert!(count_down.try_count_down(1).is_ok()); // 2
    assert!(count_down.try_count_down(1).is_ok()); // 1
    assert!(count_down.try_count_down(1).is_ok()); // 0

    // Test error when count is 0
    match count_down.try_count_down(1) {
        Err(LlmCapError::FinalCountDown) => (),
        _ => panic!("Expected FinalCountDown error"),
    }

    // Restore original environment state
    if let Some(original) = original_var {
        std::env::set_var(var_name, original);
    } else {
        std::env::remove_var(var_name);
    }
}

#[test]
fn llm_count_down_invalid_env_var() {
    let _lock = ENV_TEST_MUTEX.lock().unwrap();

    let entity_name = "MimicAI";
    let var_name = "ENABLE_MIMIC_AI_API_CALLING";

    // Save original env var state if it exists
    let original_var = std::env::var(var_name).ok();

    std::env::set_var(var_name, "not_a_number");
    let result = std::panic::catch_unwind(|| LlmCountDown::new(entity_name, var_name));
    assert!(result.is_err());

    // Restore original environment state
    if let Some(original) = original_var {
        std::env::set_var(var_name, original);
    } else {
        std::env::remove_var(var_name);
    }
}
