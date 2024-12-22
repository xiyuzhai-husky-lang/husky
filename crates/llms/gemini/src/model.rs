use enum_index::IsEnumIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, IsEnumIndex)]
pub enum GeminiModel {
    Gemini1_5Flash,
    Gemini1_5Pro,
}

impl GeminiModel {
    pub fn as_str(&self) -> &'static str {
        match self {
            GeminiModel::Gemini1_5Flash => "gemini-1.5-flash",
            GeminiModel::Gemini1_5Pro => "gemini-1.5-pro",
        }
    }

    pub fn url_name(&self) -> &'static str {
        match self {
            GeminiModel::Gemini1_5Flash => "gemini-1.5-flash",
            GeminiModel::Gemini1_5Pro => "gemini-1.5-pro",
        }
    }
}
