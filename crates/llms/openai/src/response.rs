use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OpenaiResponse {
    TextGeneration(String),
}
