use crate::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SglangResponse {
    TextGeneration { output: String },
}
