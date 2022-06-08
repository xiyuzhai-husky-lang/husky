use super::*;

// ts: { type: string; value: string; spaces_before?: number }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceTokenProps {
    pub kind: TraceTokenKind,
    pub value: String,
    pub opt_associated_trace_id: Option<TraceId>,
}

impl<T, E> From<Result<T, E>> for TraceTokenProps
where
    T: Into<TraceTokenProps>,
    E: Into<TraceTokenProps>,
{
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(t) => t.into(),
            Err(e) => e.into(),
        }
    }
}

// impl<'eval> From<EvalResult<'eval>> for TraceTokenProps {
//     fn from(result: EvalResult) -> Self {
//         match result {
//             Ok(value) => value.into(),
//             Err(e) => Self {
//                 value: e.message.into(),
//                 opt_associated_trace_id: None,
//                 kind: TraceTokenKind::Error,
//             },
//         }
//     }
// }

// impl<'eval> From<VMRuntimeResult<StackValueSnapshot<'eval>>> for TraceTokenProps {
//     fn from(_: VMRuntimeResult<StackValueSnapshot>) -> Self {
//         todo!()
//     }
// }

// impl From<VMRuntimeResult<CopyableValue>> for TraceTokenProps {
//     fn from(result: VMRuntimeResult<CopyableValue>) -> Self {
//         match result {
//             Ok(value) => value.into(),
//             Err(e) => Self {
//                 value: e.message.into(),
//                 opt_associated_trace_id: None,
//                 kind: TraceTokenKind::Error,
//             },
//         }
//     }
// }

// impl From<CopyableValue> for TraceTokenProps {
//     fn from(value: CopyableValue) -> Self {
//         fade!(value)
//     }
// }

// impl From<InitKind> for TraceTokenProps {
//     fn from(init_kind: InitKind) -> Self {
//         match init_kind {
//             InitKind::Let => keyword!("let "),
//             InitKind::Var => keyword!("var "),
//             InitKind::Decl => panic!(),
//         }
//     }
// }

// ts: string
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
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

#[macro_export]
macro_rules! keyword {
    ($value:expr) => {{
        TraceTokenProps {
            kind: TraceTokenKind::Keyword,
            value: $value.into(),
            opt_associated_trace_id: None,
        }
    }};
}

#[macro_export]
macro_rules! label {
    ($value:expr, $associated:expr) => {{
        TraceTokenProps {
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
        TraceTokenProps {
            kind: TraceTokenKind::Ident,
            value: $value.into(),
            opt_associated_trace_id: None,
        }
    }};
    ($value:expr, $opt_associated_trace_id: expr) => {{
        TraceTokenProps {
            kind: TraceTokenKind::Ident,
            value: $value.into(),
            opt_associated_trace_id: $opt_associated_trace_id,
        }
    }};
}

#[macro_export]
macro_rules! literal {
    ($value:expr) => {{
        TraceTokenProps {
            kind: TraceTokenKind::Literal,
            value: $value.into(),
            opt_associated_trace_id: None,
        }
    }};
}

#[macro_export]
macro_rules! special {
    ($value: expr) => {{
        TraceTokenProps {
            kind: TraceTokenKind::Special,
            value: $value.into(),
            opt_associated_trace_id: None,
        }
    }};

    ($value: expr, $opt_associated_trace_id: expr) => {{
        TraceTokenProps {
            kind: TraceTokenKind::Special,
            value: $value.into(),
            opt_associated_trace_id: $opt_associated_trace_id,
        }
    }};
}

#[macro_export]
macro_rules! route {
    ($value:expr) => {{
        TraceTokenProps {
            kind: TraceTokenKind::Scope,
            value: $value.into(),
            opt_associated_trace_id: None,
        }
    }};

    ($value:expr, $opt_associated_trace_id: expr) => {{
        TraceTokenProps {
            kind: TraceTokenKind::Scope,
            value: $value.into(),
            opt_associated_trace_id: $opt_associated_trace_id,
        }
    }};
}

#[macro_export]
macro_rules! fade {
    ($value:expr) => {{
        TraceTokenProps {
            kind: TraceTokenKind::Fade,
            value: $value.into(),
            opt_associated_trace_id: None,
        }
    }};
    ($value:expr, $associated:expr) => {{
        TraceTokenProps {
            kind: TraceTokenKind::Fade,
            value: $value.into(),
            opt_associated_trace_id: $associated,
        }
    }};
}
