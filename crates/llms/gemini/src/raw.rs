use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Hash)]
pub struct GeminiRawContent {
    pub parts: Vec<GeminiRawPart>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Hash)]
pub struct GeminiRawPart {
    pub text: String,
}
