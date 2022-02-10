pub(crate) mod feature_trace;

use crate::*;

// ts: { type: string; value: string; spaces_before?: number }
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct TokenProps {
    pub kind: TraceTokenKind,
    pub value: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spaces_before: Option<u8>,
}

// ts: string
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TraceTokenKind {
    Keyword,
    Label,
    Ident,
    Literal,
    Special,
    Scope,
    Fade,
}

macro_rules! keyword {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Keyword,
            value: $value.into(),
            spaces_before: None,
        }
    }};
    ($value:expr, $spaces_before:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Keyword,
            value: $value.into(),
            spaces_before: Some($spaces_before),
        }
    }};
}
pub(crate) use keyword;

macro_rules! label {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Label,
            value: $value.into(),
            spaces_before: None,
        }
    }};
    ($value:expr, $spaces_before:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Label,
            value: $value.into(),
            spaces_before: Some($spaces_before),
        }
    }};
}
pub(crate) use label;

macro_rules! ident {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Ident,
            value: $value.into(),
            spaces_before: None,
        }
    }};
    ($value:expr, $spaces_before:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Ident,
            value: $value.into(),
            spaces_before: Some($spaces_before as u8),
        }
    }};
}
pub(crate) use ident;

macro_rules! literal {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Literal,
            value: $value.into(),
            spaces_before: None,
        }
    }};
    ($value:expr, $spaces_before:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Ident,
            value: $value,
            spaces_before: Some($spaces_before),
        }
    }};
}
pub(crate) use literal;

macro_rules! special {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Special,
            value: $value.into(),
            spaces_before: None,
        }
    }};
    ($value:expr, $spaces_before:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Special,
            value: $value.into(),
            spaces_before: Some($spaces_before),
        }
    }};
}
pub(crate) use special;

macro_rules! scp {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Scope,
            value: $value.into(),
            spaces_before: None,
        }
    }};
}
pub(crate) use scp;

macro_rules! fade {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Fade,
            value: $value.into(),
            spaces_before: None,
        }
    }};
}
pub(crate) use fade;
