use super::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceStalkRawData {
    pub extra_tokens: Vec<TraceTokenData>,
}
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TraceStalkData {
    pub opt_extra_tokens: Option<Rc<Vec<Rc<TraceTokenData>>>>,
}

impl From<TraceStalkRawData> for TraceStalkData {
    fn from(raw_data: TraceStalkRawData) -> Self {
        Self {
            opt_extra_tokens: if raw_data.extra_tokens.len() > 0 {
                Some(Rc::new(
                    raw_data
                        .extra_tokens
                        .into_iter()
                        .map(|token| Rc::new(token.into()))
                        .collect(),
                ))
            } else {
                None
            },
        }
    }
}
