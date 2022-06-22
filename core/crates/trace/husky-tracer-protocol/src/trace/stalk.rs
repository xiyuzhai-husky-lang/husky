use super::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceStalkData {
    pub extra_tokens: Vec<TraceTokenData>,
}
