#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeminiModel {
    Gemini1_5Flash,
    Gemini1_5Pro,
}

impl GeminiModel {
    pub fn as_str(&self) -> &str {
        match self {
            GeminiModel::Gemini1_5Flash => "gemini-1.5-flash",
            GeminiModel::Gemini1_5Pro => "gemini-1.5-pro",
        }
    }
}
