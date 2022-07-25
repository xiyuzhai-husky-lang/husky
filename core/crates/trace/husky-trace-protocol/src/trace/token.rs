use husky_signal::Signalable;
use husky_vm_interface::__VMError;

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
    Label,
    Ident,
    Literal,
    Special,
    Scope,
    Fade,
    Error,
}

impl TraceTokenKind {
    pub fn as_str(self) -> &'static str {
        match self {
            TraceTokenKind::Keyword => "keyword",
            TraceTokenKind::Label => "literal",
            TraceTokenKind::Ident => "ident",
            TraceTokenKind::Literal => "literal",
            TraceTokenKind::Special => "special",
            TraceTokenKind::Scope => "scope",
            TraceTokenKind::Fade => "fade",
            TraceTokenKind::Error => "error",
        }
    }
}

impl From<__VMError> for TraceTokenData {
    fn from(_: __VMError) -> Self {
        todo!()
    }
}

impl<'eval> From<__Register<'eval>> for TraceTokenData {
    fn from(_: __Register<'eval>) -> Self {
        todo!()
    }
}

impl std::fmt::Display for TraceTokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

#[macro_export]
macro_rules! keyword {
    ($value:expr) => {{
        TraceTokenData {
            kind: TraceTokenKind::Keyword,
            value: $value.into(),
            opt_associated_trace_id: None,
        }
    }};
}

#[macro_export]
macro_rules! label {
    ($value:expr, $associated:expr) => {{
        TraceTokenData {
            kind: TraceTokenKind::Label,
            value: $value.into(),
            spaces_before: None,
            associated: $associated,
            associated: vec![],
        }
    }};
}

#[macro_export]
macro_rules! ident {
    ($value:expr) => {{
        TraceTokenData {
            kind: TraceTokenKind::Ident,
            value: $value.into(),
            opt_associated_trace_id: None,
        }
    }};
    ($value:expr, $opt_associated_trace_id: expr) => {{
        TraceTokenData {
            kind: TraceTokenKind::Ident,
            value: $value.into(),
            opt_associated_trace_id: $opt_associated_trace_id,
        }
    }};
}

#[macro_export]
macro_rules! literal {
    ($value:expr) => {{
        TraceTokenData {
            kind: TraceTokenKind::Literal,
            value: $value.into(),
            opt_associated_trace_id: None,
        }
    }};
}

#[macro_export]
macro_rules! special {
    ($value: expr) => {{
        TraceTokenData {
            kind: TraceTokenKind::Special,
            value: $value.into(),
            opt_associated_trace_id: None,
        }
    }};

    ($value: expr, $opt_associated_trace_id: expr) => {{
        TraceTokenData {
            kind: TraceTokenKind::Special,
            value: $value.into(),
            opt_associated_trace_id: $opt_associated_trace_id,
        }
    }};
}

#[macro_export]
macro_rules! route {
    ($value:expr) => {{
        TraceTokenData {
            kind: TraceTokenKind::Scope,
            value: $value.into(),
            opt_associated_trace_id: None,
        }
    }};

    ($value:expr, $opt_associated_trace_id: expr) => {{
        TraceTokenData {
            kind: TraceTokenKind::Scope,
            value: $value.into(),
            opt_associated_trace_id: $opt_associated_trace_id,
        }
    }};
}

#[macro_export]
macro_rules! fade {
    ($value:expr) => {{
        TraceTokenData {
            kind: TraceTokenKind::Fade,
            value: $value.into(),
            opt_associated_trace_id: None,
        }
    }};
    ($value:expr, $associated:expr) => {{
        TraceTokenData {
            kind: TraceTokenKind::Fade,
            value: $value.into(),
            opt_associated_trace_id: $associated,
        }
    }};
}
