use crate::*;

// ts: { type: string; value: string; spaces_before?: number }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenProps {
    pub kind: TraceTokenKind,
    pub value: Cow<'static, str>,
    pub associated_trace: Option<Arc<Trace>>,
}

impl Serialize for TokenProps {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("TokenProps", 3)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field(
            "associated_trace",
            &self.associated_trace.as_ref().map(|trace| trace.id),
        )?;
        state.end()
    }
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
            associated_trace: None,
        }
    }};
}
pub(crate) use keyword;

// macro_rules! label {
//     ($value:expr, $associated:expr) => {{
//         TokenProps {
//             kind: TraceTokenKind::Label,
//             value: $value.into(),
//             spaces_before: None,
//             associated: $associated,
//             associated: vec![],
//         }
//     }};
// }
// pub(crate) use label;

macro_rules! ident {
    ($value:expr, $associated_trace: expr) => {{
        TokenProps {
            kind: TraceTokenKind::Ident,
            value: $value.into(),
            associated_trace: $associated_trace,
        }
    }};
}
pub(crate) use ident;

macro_rules! literal {
    ($value:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Literal,
            value: $value.into(),
            associated_trace: None,
        }
    }};
}
pub(crate) use literal;

macro_rules! special {
    ($value: expr, $associated_trace: expr) => {{
        TokenProps {
            kind: TraceTokenKind::Special,
            value: $value.into(),
            associated_trace: $associated_trace,
        }
    }};
}
pub(crate) use special;

macro_rules! scope {
    ($value:expr, $associated_trace: expr, $associated_subtraces: expr) => {{
        TokenProps {
            kind: TraceTokenKind::Scope,
            value: $value.into(),
            associated_trace: $associated_trace,
            associated_subtraces: $associated_subtraces,
        }
    }};
}
pub(crate) use scope;

macro_rules! fade {
    ($value:expr, $associated:expr) => {{
        TokenProps {
            kind: TraceTokenKind::Fade,
            value: $value.into(),
            associated: $associated,
        }
    }};
}
pub(crate) use fade;
