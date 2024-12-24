use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LlmChatCompletionRequest {
    pub prompt: String,
}

impl IsLlmRequest for LlmChatCompletionRequest {
    type Response = LlmChatCompletionResponse;
}
