use crate::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize)]
pub struct TraceStalk<'eval> {
    pub extra_tokens: Vec<TokenProps<'eval>>,
}
