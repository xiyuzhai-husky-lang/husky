#[derive(Debug, Clone)]
pub struct TraceViewProtocol {
    tokens: Vec<TraceViewToken>,
}

#[derive(Debug, Clone)]
pub struct TraceViewToken {
    text: String,
    // ranged_semantic_token: husky_semantic_token::token::SemanticToken,
}
