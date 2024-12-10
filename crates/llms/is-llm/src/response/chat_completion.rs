use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LlmChatCompletionResponse {
    pub content: String,
}

impl IsLlmResponse for LlmChatCompletionResponse {}
