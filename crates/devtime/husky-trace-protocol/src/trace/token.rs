use husky_signal::Signalable;

use super::*;

// ts: { type: string; value: string; spaces_before?: number }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceTokenData {
    pub kind: TraceTokenKind,
    pub value: String,
    pub opt_associated_trace_id: Option<TraceId>,
}

impl Signalable for TraceTokenData {}

impl<T, E> From<Result<T, E>> for TraceTokenData
where
    T: Into<TraceTokenData>,
    E: Into<TraceTokenData>,
{
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(t) => t.into(),
            Err(e) => e.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TraceTokenKind {
    Keyword,
    Mod,
    Label,
    Ident,
    Literal,
    Special,
    Scope,
    Fade,
    Error,
}

impl TraceTokenKind {
    pub fn code(self) -> &'static str {
        match self {
            TraceTokenKind::Keyword => "keyword",
            TraceTokenKind::Mod => "mod",
            TraceTokenKind::Label => "literal",
            TraceTokenKind::Ident => "ident",
            TraceTokenKind::Literal => "literal",
            TraceTokenKind::Special => "special",
            TraceTokenKind::Scope => "visibility",
            TraceTokenKind::Fade => "fade",
            TraceTokenKind::Error => "error",
        }
    }
}

impl std::fmt::Display for TraceTokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.code().fmt(f)
    }
}

// #[macro_export]
// macro_rules! label {
//     ($value:expr, $associated:expr) => {{
//         TraceTokenData {
//             kind: TraceTokenKind::Label,
//             value: $value.into(),
//             spaces_before: None,
//             associated: $associated,
//             associated: vec![],
//         }
//     }};
// }
