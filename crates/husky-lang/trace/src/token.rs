use crate::*;

// ts: { type: string; value: string; spaces_before?: number }
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct TraceToken {
    pub kind: TraceTokenKind,
    pub value: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spaces_before: Option<usize>,
}

// ts: string
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TraceTokenKind {
    Keyword,
    Ident,
    Special,
    Scope,
    Fade,
}

macro_rules! keyword {
    ($value:expr) => {{
        TraceToken {
            kind: TraceTokenKind::Keyword,
            value: $value,
            spaces_before: None,
        }
    }};
    ($value:expr, $spaces_before:expr) => {{
        TraceToken {
            kind: TraceTokenKind::Keyword,
            value: $value,
            spaces_before: Some($spaces_before),
        }
    }};
}
pub(crate) use keyword;

macro_rules! ident {
    ($value:expr) => {{
        TraceToken {
            kind: TraceTokenKind::Ident,
            value: $value.into(),
            spaces_before: None,
        }
    }};
    ($value:expr, $spaces_before:expr) => {{
        TraceToken {
            kind: TraceTokenKind::Ident,
            value: $value,
            spaces_before: Some($spaces_before),
        }
    }};
}
pub(crate) use ident;

macro_rules! special {
    ($value:expr) => {{
        TraceToken {
            kind: TraceTokenKind::Special,
            value: $value,
            spaces_before: None,
        }
    }};
    ($value:expr, $spaces_before:expr) => {{
        TraceToken {
            kind: TraceTokenKind::Special,
            value: $value,
            spaces_before: Some($spaces_before),
        }
    }};
}
pub(crate) use special;

macro_rules! scp {
    ($value:expr) => {{
        TraceToken {
            kind: TraceTokenKind::Scope,
            value: $value,
            spaces_before: None,
        }
    }};
}
pub(crate) use scp;

macro_rules! fade {
    ($value:expr) => {{
        TraceToken {
            kind: TraceTokenKind::Fade,
            value: $value,
            spaces_before: None,
        }
    }};
}
pub(crate) use fade;
