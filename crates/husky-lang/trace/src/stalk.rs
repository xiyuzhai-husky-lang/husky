use crate::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
pub struct TraceStalk {
    pub extra_tokens: Vec<TokenProps>,
}
