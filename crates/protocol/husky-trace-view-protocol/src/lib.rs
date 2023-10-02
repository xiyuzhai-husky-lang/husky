mod storage;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceViewData {
    tokens: Vec<TraceViewToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceViewToken {
    text: String,
    // ranged_semantic_token: husky_semantic_token::token::SemanticToken,
}
