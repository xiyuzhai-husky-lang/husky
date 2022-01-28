use std::borrow::Cow;

use crate::*;

pub const ROOT_TRACES: &'static [Trace] = &[Trace {
    idx: 0,
    parent: None,
    tokens: Cow::Borrowed(&[
        TraceToken {
            kind: TraceTokenKind::Keyword,
            value: "let",
            spaces_before: None,
        },
        TraceToken {
            kind: TraceTokenKind::Ident,
            value: "a",
            spaces_before: None,
        },
        TraceToken {
            kind: TraceTokenKind::Special,
            value: "=",
            spaces_before: None,
        },
        TraceToken {
            kind: TraceTokenKind::Scope,
            value: "f",
            spaces_before: None,
        },
        TraceToken {
            kind: TraceTokenKind::Special,
            value: "(",
            spaces_before: Some(0),
        },
        TraceToken {
            kind: TraceTokenKind::Ident,
            value: "c",
            spaces_before: Some(0),
        },
        TraceToken {
            kind: TraceTokenKind::Special,
            value: "+",
            spaces_before: None,
        },
        TraceToken {
            kind: TraceTokenKind::Ident,
            value: "b",
            spaces_before: None,
        },
        TraceToken {
            kind: TraceTokenKind::Special,
            value: ")",
            spaces_before: Some(0),
        },
        TraceToken {
            kind: TraceTokenKind::Fade,
            value: "=",
            spaces_before: None,
        },
        TraceToken {
            kind: TraceTokenKind::Fade,
            value: "a plot",
            spaces_before: None,
        },
    ]),
}];
