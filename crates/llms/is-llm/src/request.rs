pub mod chat_completion;

use self::chat_completion::LlmChatCompletionRequest;
use crate::{response::LlmResponse, *};
use llm_cache::traits::IsLlmCacheRequest;

pub trait IsLlmRequest: IsLlmCacheRequest {
    type Response: IsLlmResponse;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LlmRequest {
    ChatCompletion(LlmChatCompletionRequest),
}

impl IsLlmRequest for LlmRequest {
    type Response = LlmResponse;
}
