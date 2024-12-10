pub mod error;

use self::error::{LlmError, LlmResult};

// TODO: another trait IsLlmDyn?
pub trait IsLlm {
    fn chat_completion(&self, prompt: &str) -> LlmResult<String>;
}
