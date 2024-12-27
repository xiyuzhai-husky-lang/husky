use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SglangRequest {
    TextGeneration { input: String },
}
