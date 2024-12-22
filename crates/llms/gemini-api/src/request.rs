use crate::common::Content;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug, Hash)]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
}
