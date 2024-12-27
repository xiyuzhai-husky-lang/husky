use crate::*;
use openai_api_rs::v1::chat_completion::{
    ChatCompletionMessage, ChatCompletionRequest, Content, MessageRole,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OpenaiRequest {
    TextGeneration { input: String },
}

pub(crate) enum OaiRequestExt {
    ChatCompletion(ChatCompletionRequest),
}

impl OpenaiRequest {
    pub fn ext(&self, model: OpenaiModel) -> OaiRequestExt {
        match self {
            OpenaiRequest::TextGeneration { input } => {
                OaiRequestExt::ChatCompletion(ChatCompletionRequest::new(
                    model.as_str().to_string(),
                    vec![ChatCompletionMessage {
                        role: MessageRole::user,
                        content: Content::Text(input.clone()),
                        name: None,
                        tool_calls: None,
                        tool_call_id: None,
                    }],
                ))
            }
        }
    }
}
