use crate::*;
use openai_api_rs::v1::chat_completion::{
    ChatCompletionMessage, ChatCompletionRequest, Content, MessageRole,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OpenaiRequest {
    ChatCompletion(String),
}

pub(crate) enum OaiRequestExt {
    ChatCompletion(ChatCompletionRequest),
}

impl OpenaiRequest {
    pub fn ext(&self) -> OaiRequestExt {
        match self {
            OpenaiRequest::ChatCompletion(s) => {
                OaiRequestExt::ChatCompletion(ChatCompletionRequest::new(
                    "gpt-4o".to_string(),
                    vec![ChatCompletionMessage {
                        role: MessageRole::user,
                        content: Content::Text(s.clone()),
                        name: None,
                        tool_calls: None,
                        tool_call_id: None,
                    }],
                ))
            }
        }
    }
}
