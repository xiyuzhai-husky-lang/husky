use super::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceStalk {
    pub extra_tokens: Vec<TraceTokenData>,
}
