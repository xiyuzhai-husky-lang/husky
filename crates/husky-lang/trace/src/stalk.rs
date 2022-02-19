use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TraceStalk {
    pub extra_tokens: Vec<TokenProps>,
}
