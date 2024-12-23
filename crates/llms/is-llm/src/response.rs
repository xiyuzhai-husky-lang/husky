pub mod chat_completion;

use disk_cache::traits::IsDiskCacheResponse;

use crate::*;

pub trait IsLlmResponse: IsDiskCacheResponse {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LlmResponse {
    ChatCompletion(LlmChatCompletionResponse),
}

impl IsLlmResponse for LlmResponse {}
