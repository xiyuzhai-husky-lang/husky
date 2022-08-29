use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceStalk {
    pub extra_tokens: Vec<TraceTokenData>,
    pub kind: TraceStalkKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceStalkKind {
    Value,
    Unarrived,
}

impl TraceStalk {
    pub fn unarrived() -> Self {
        Self {
            extra_tokens: vec![],
            kind: TraceStalkKind::Unarrived,
        }
    }
}

impl Default for TraceStalk {
    fn default() -> Self {
        Self {
            extra_tokens: Default::default(),
            kind: TraceStalkKind::Value,
        }
    }
}
