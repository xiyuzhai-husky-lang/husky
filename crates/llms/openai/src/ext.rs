pub use openai_api_rs::v1::api::OpenAIClient;
pub use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
pub use openai_api_rs::v1::common::GPT4_O;

use crate::*;
use tokio::runtime::Runtime;

impl<'db> OpenaiClient<'db> {
    pub(crate) async fn complete_chat_ext(
        &self,
        request: ChatCompletionRequest,
    ) -> OpenaiResult<String> {
        // Block on the async chat completion request using tokio
        let Some(ref client_ext) = self.client_ext else {
            return Err(OpenaiError::EnvApiKeyNotSet);
        };
        let response = client_ext
            .chat_completion(request)
            .await
            .map_err(|_| OpenaiError::ExtChatCompletion)?;

        // Extract the message content from the first choice
        Ok(response
            .choices
            .first()
            .ok_or(OpenaiError::NoChoicesReturned)?
            .message
            .content
            .clone()
            .unwrap_or_default())
    }
}
