use crate::*;
use openai_api_rs::v1::chat_completion::{
    ChatCompletionMessage, ChatCompletionRequest, Content, MessageRole,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OaiRequest {
    ChatCompletion(String),
}

pub(crate) enum OaiRequestExt {
    ChatCompletion(ChatCompletionRequest),
}

impl OaiRequest {
    pub fn ext(self) -> OaiRequestExt {
        match self {
            OaiRequest::ChatCompletion(s) => {
                OaiRequestExt::ChatCompletion(ChatCompletionRequest::new(
                    "gpt-4o".to_string(),
                    vec![ChatCompletionMessage {
                        role: MessageRole::user,
                        content: Content::Text(s),
                        name: None,
                        tool_calls: None,
                        tool_call_id: None,
                    }],
                ))
            }
        }
    }
}
