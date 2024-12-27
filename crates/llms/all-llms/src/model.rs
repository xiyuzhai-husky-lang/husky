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
    pub const GPT_4O: Self = Self::Openai(OpenaiModel::Gpt4o);
    pub const GEMINI_1_5_FLASH: Self = Self::Gemini(GeminiModel::Gemini1_5Flash);
    pub const GEMINI_1_5_PRO: Self = Self::Gemini(GeminiModel::Gemini1_5Pro);
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

impl TryFrom<&str> for AllLlmModel {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "gpt-4o" => Ok(AllLlmModel::GPT_4O),
            "gemini-1.5-flash" => Ok(AllLlmModel::GEMINI_1_5_FLASH),
            "gemini-1.5-pro" => Ok(AllLlmModel::GEMINI_1_5_PRO),
            _ => Err(()),
        }
    }
}

impl serde::Serialize for AllLlmModel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for AllLlmModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::try_from(s.as_str())
            .map_err(|_| serde::de::Error::custom(format!("unknown model: {}", s)))
    }
}
