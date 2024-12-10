pub mod error;

use self::error::{LlmError, LlmResult};

pub trait IsLlm {
    fn chat_completion(&self, prompt: &str) -> LlmResult<String>;
}
