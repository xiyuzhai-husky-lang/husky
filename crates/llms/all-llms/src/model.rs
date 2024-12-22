use gemini::model::GeminiModel;
use openai::model::OpenaiModel;
use sglang::model::SglangModel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllLlmModel {
    Openai(OpenaiModel),
    Gemini(GeminiModel),
    Sglang(SglangModel),
}

impl AllLlmModel {
    pub fn as_str(&self) -> &str {
        match self {
            AllLlmModel::Openai(model) => model.as_str(),
            AllLlmModel::Gemini(model) => model.as_str(),
            AllLlmModel::Sglang(model) => model.as_str(),
        }
    }
}
