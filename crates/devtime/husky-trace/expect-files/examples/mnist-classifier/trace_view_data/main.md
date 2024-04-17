```rust
[
    (
        Trace(
            Id {
                value: 1,
            },
        ),
        TraceViewData {
            trace_kind: Submodule,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "mod",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "digits",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 4,
            },
        ),
        TraceViewData {
            trace_kind: Submodule,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "mod",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "zero",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 14,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "open_one_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "FermiMatchResult",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 16,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "fermi_match",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "almost_closed",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 15,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "pub",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "super",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "is_zero",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Zero",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 17,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "raw_contours",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ilen",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "==",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 30,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "n",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "open_one_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 31,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "n",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1.5",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 32,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "open_one_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 33,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "connected_components",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ilen",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "==",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 34,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "c",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "open_one_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "displacement",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 35,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "c",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "5.5",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 36,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "return",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 18,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "simp_zero_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "fermi_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 19,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "narrow_down",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "simp_zero_match",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "simp_zero_match",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "rel_norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "simp_zero_match",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "angle_change_norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "skip",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "5",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 20,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "simp_zero_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "3.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 21,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "None",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 22,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 23,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_hole",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 24,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "a",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_hole",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "bounding_box",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ymax",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_hole",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "bounding_box",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ymin",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 25,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "b",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_line_segment_sketch",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "bounding_box",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ymax",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_line_segment_sketch",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "bounding_box",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ymin",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 26,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ratio",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "a",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "/",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "b",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 27,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ratio",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0.4",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 28,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "a",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "simp_zero_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 29,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 5,
            },
        ),
        TraceViewData {
            trace_kind: Submodule,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "mod",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "one",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 37,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "one_fermi_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "FermiMatchResult",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 39,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "fermi_match",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upmost",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "hat",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 38,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "pub",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "is_one",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "One",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 40,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "narrow_down",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "max_hole_ilen",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "ignored_connected_components_row_span_sum_sum",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "skip",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "5",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 41,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "simp_one_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "fermi_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 42,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "simp_one_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "3.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 44,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "narrow_down",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "max_row_span",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "skip",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "5",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 45,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "max_row_span",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "6.5",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 46,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 43,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "else",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 47,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "max_hole_ilen",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "==",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 48,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ignored_connected_components_row_span_sum_sum",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "==",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 49,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "one_fermi_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 50,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upmost",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "one_fermi_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 51,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "hat",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "one_fermi_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "2",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 52,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "None",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 53,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost_number_of_strokes",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "strokes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "end",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "strokes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "start",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 54,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "one_fermi_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 55,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "narrow_down",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "top_k_row_span_sum",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "3",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "one_fermi_match",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "one_fermi_match",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "rel_norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "one_fermi_match",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "angle_change_norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "skip",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "12",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 56,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "narrow_down",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "one_fermi_match",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "rel_norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "one_fermi_match",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "angle_change",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "abs",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "skip",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "2",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 57,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upmost",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 58,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "hat",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 59,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost_number_of_strokes",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "==",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "3",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 60,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost_hat",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "strokes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "strokes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "start",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 61,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost_hat_dp",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost_hat",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "displacement",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 62,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost_feet",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "strokes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "strokes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "start",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "+",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "2",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 63,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost_feet_dp",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost_feet",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "displacement",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 64,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "narrow_down",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "downmost_hat_dp",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "y",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "downmost_feet_dp",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "y",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "skip",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "5",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 65,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost_number_of_strokes",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "==",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "3",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 66,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "a",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "lower_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 67,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "b",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 68,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "c",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "a",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "b",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 69,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "d",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "c",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "+",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "a",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 70,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "c",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "10.0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 71,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "c",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "20.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 72,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 6,
            },
        ),
        TraceViewData {
            trace_kind: Submodule,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "mod",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "six",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 73,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "six_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "FermiMatchResult",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 76,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "fermi_match",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upmost",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 74,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "six_match_refined1",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "FermiMatchResult",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 77,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "fermi_match",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upmost",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "bottom1",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 75,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "pub",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "super",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "is_six",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Six",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 78,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upmost_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "six_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 79,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upmost_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 80,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 81,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "lower_excess",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "lower_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 82,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "None",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 89,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "narrow_down",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "six_match",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "six_match_refined1",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "rel_norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "skip",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "5",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 90,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "bottom1_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "six_match_refined1",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 91,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "bottom1_match_dp",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "bottom1_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "displacement",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 92,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "bottom1_match_dp_y",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "bottom1_match_dp",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "y",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 93,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upmost_match_dp_y",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upmost_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "displacement",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "y",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 94,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "others",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "six_match_refined1",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "others",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 95,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "six_match_refined1",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1.8",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 96,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "bottom1_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 97,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "rel_upmost_match_end",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_line_segment_sketch",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "bounding_box",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "relative_point",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "upmost_match",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "end",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 98,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "narrow_down",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "upmost_match_dp_y",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "ignored_connected_components_row_span_sum_sum",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "lower_excess",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "top_k_row_span_sum",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "6",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "skip",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "15",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 99,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "rel_upmost_match_end",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "x",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0.7",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 100,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "return",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 83,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "narrow_down",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "six_match",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "skip",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "5",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 84,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "six_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1.8",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 101,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "six_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1.8",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 85,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "relative_bounding_box",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ymax",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0.75",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 86,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 102,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "relative_bounding_box",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ymax",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0.75",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 87,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "lower_excess",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "15.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 88,
            },
        ),
        TraceViewData {
            trace_kind: LazyStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 7,
            },
        ),
        TraceViewData {
            trace_kind: Submodule,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "mod",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "three",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 103,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "three_fermi_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "FermiMatchResult",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 105,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "fermi_match",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downarc",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "uparc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "back",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 104,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "pub",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "super",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "is_three",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Three",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 106,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ilen",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "2",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 107,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ilen",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "4",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 108,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downarc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "three_fermi_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 109,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "uparc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "three_fermi_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 110,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "back",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "three_fermi_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "2",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 111,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downarc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 112,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downarc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "3.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 113,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "uparc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 114,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "de",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downarc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "end_tangent",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "angle",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "true",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 115,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "de",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "or",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "de",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "100.0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 116,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downarc_enpoint",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downarc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "end",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 117,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "uparc_startpoint",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "uparc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "start",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 118,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "distance",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downarc_enpoint",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "dist",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "uparc_startpoint",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 119,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "distance",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "20.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 120,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "three_fermi_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "2.5",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 121,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downarc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "angle_change",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "100.0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 122,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 8,
            },
        ),
        TraceViewData {
            trace_kind: Submodule,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "mod",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "four",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 123,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_components",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "FermiMatchResult",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 127,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "fermi_match",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_coordinate_max",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_coordinate_max",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 124,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "components_max_downwards",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "FermiMatchResult",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 128,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "fermi_match",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "displacement_downwards",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 125,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "components_max_heights",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "FermiMatchResult",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 129,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "fermi_match",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "cc_box_heights",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 126,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "is_four",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Four",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 130,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_components",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 131,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_components",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 132,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 133,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "None",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 134,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "down_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "components_max_downwards",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 135,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "down_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 136,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "down_match_dp_y",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "down_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "displacement",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "y",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 137,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "higher_excess",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "lower_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 138,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "higher_excess",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "7.0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 139,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "None",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 141,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ilen",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">=",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "2",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 142,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "four_match_refine_result",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "components_max_heights",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 143,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "four_match_refine_result",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 144,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "components_max_heights",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1.0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 145,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "higher_excess",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "lower_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 146,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_arc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "components_max_heights",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 147,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_arc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 148,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_arc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "displacement",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "y",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 149,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_arc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "angle_change",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "110.0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 150,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "components_max_heights",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "9.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 151,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "a",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "top_k_row_right_mass_sum",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "3",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 152,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "a",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "22.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 153,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "a",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "9.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 154,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "return",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 140,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 9,
            },
        ),
        TraceViewData {
            trace_kind: Submodule,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "mod",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "five",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 155,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "pub",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "super",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "is_five",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Five",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 156,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 10,
            },
        ),
        TraceViewData {
            trace_kind: Submodule,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "mod",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "seven",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 157,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "simple_seven_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "FermiMatchResult",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 160,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "fermi_match",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "simple_leftdown_pattern",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 158,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "special_seven_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "FermiMatchResult",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 161,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "fermi_match",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "leftupcc_pattern",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "leftdowncc_pattern",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 159,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "pub",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "super",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "is_seven",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Seven",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 162,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "max_hole_ilen",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "==",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0.",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 163,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "simple_match_norm",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "simple_seven_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 164,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "simple_match_norm",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 170,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "simple_seven_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 171,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_excess",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "lower_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 172,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_excess",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "10.",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 173,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "return",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 165,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "simple_match_norm",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "4.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 174,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_excess",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "lower_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 175,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_excess",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "10.",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 176,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "return",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 166,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "special_seven_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 167,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "others",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "special_seven_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "others",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 168,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "false",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 169,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 11,
            },
        ),
        TraceViewData {
            trace_kind: Submodule,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "mod",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eight",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 177,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_mouth_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "FermiMatchResult",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 179,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "fermi_match",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "big_mouth",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 178,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "pub",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "super",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "is_eight",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Eight",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 2,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 180,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_excess",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "lower_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 181,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "None",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 183,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "None",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 184,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "false",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 182,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 12,
            },
        ),
        TraceViewData {
            trace_kind: Submodule,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "mod",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "nine",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 185,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "nine_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "FermiMatchResult",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 188,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "fermi_match",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "downmost",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 186,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "nine_match_refine",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "FermiMatchResult",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 189,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "fermi_match",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "big_cc",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 187,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "pub",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "super",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "is_nine",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Nine",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 190,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 191,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "None",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 192,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "down_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "nine_match",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 193,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "down_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 194,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "down_match_dp_y",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "down_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "displacement",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "y",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 195,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "higher_excess",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "lower_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 196,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "higher_excess",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "7.0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 197,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "None",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 199,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ilen",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">=",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "2",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 200,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "nine_match_refine_result",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "nine_match_refine",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 201,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "nine_match_refine_result",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 202,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "nine_match_refine",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1.0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 203,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "higher_excess",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "lower_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 204,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_arc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "nine_match_refine",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 205,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_arc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 206,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_arc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "displacement",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "y",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 207,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_arc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "angle_change",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "110.0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 208,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "nine_match_refine",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "norm",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "9.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 209,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "a",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "top_k_row_right_mass_sum",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "3",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 210,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "a",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "22.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 211,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "a",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "9.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 212,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "return",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 198,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 13,
            },
        ),
        TraceViewData {
            trace_kind: Submodule,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "mod",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "two",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 213,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "two_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "FermiMatchResult",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 215,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "fermi_match",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_cc_pattern",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "right_cc_pattern",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ",",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "down_cc_pattern",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 214,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "pub",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "super",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "is_two",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Two",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 216,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "cc_num",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ilen",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 217,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 218,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "eff_holes",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "None",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 219,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "two_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 220,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "right_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "two_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "1",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 221,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "down_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "two_match",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "matches",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "2",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 222,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "cc_num",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<=",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "3",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 223,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "lower_excess",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "lower_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "upper_mass",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 224,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "lower_excess",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "10.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 225,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "cc_num",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "==",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "2",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 228,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 229,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "right_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 230,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "a",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "right_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "angle_change",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 231,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "a",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "180.0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 232,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "end_tan",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "end_tangent",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "angle",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "true",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 233,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "x",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "end_tangent",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "x",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 234,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "y",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "end_tangent",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "y",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 235,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_ymax",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "relative_bounding_box",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ymax",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 236,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_ymin",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "relative_bounding_box",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ymin",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 237,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_mid_y",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_ymax",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "+",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_ymin",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "/",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "2.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 238,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "right_ymax",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "right_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "relative_bounding_box",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ymax",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 239,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "right_ymin",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "right_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "relative_bounding_box",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ymin",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 240,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "right_mid_y",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "right_ymax",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "+",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "right_ymin",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "/",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "2.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 241,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_mid_y",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "right_mid_y",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 226,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "cc_num",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "==",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "3",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 242,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "left_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 243,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "right_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 244,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "down_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "be",
                            token_class: WordOpr,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Some",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "_",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 245,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "require",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "down_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "relative_bounding_box",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ymin",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0.4",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 246,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "a",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "down_cc",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "!",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "angle_change",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 227,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "OneVsAll",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Yes",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 2,
            },
        ),
        TraceViewData {
            trace_kind: Submodule,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "mod",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 247,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "pub",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "connected_components",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ConnectedComponent",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 254,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "find_connected_components",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "input",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 248,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "pub",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "~",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ConnectedComponent",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 255,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "mut",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "i0",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 256,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "mut",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "max_row_span_sum",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 257,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "for",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "i",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "connected_components",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ilen",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 259,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "row_span_sum",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "connected_components",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "i",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "row_span_sum",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 260,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "if",
                            token_class: ControlFlowKeyword,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "row_span_sum",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ">",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "max_row_span_sum",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 261,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "max_row_span_sum",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "row_span_sum",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 262,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "i0",
                            token_class: Ident,
                            spaces_before: 12,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "i",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 258,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "return",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "connected_components",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "i0",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 249,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "pub",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ignored_connected_components_row_span_sum_sum",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "f32",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 263,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "let",
                            token_class: OtherKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "mut",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "sum",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0.0",
                            token_class: Literal,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 264,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "for",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "i",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "<",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "connected_components",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ilen",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "(",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ")",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 266,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "sum",
                            token_class: Ident,
                            spaces_before: 8,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "+=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "connected_components",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "i",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "row_span_sum",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 265,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "return",
                            token_class: ControlFlowKeyword,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "sum",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "-",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "row_span_sum",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 250,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "pub",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_raw_contours",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "~",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "RawContour",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 267,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "raw_contours",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 251,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "pub",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_raw_contour",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "~",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "RawContour",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 268,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "major_connected_component",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "raw_contours",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "0",
                            token_class: Literal,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 252,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "pub",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_line_segment_sketch",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "~",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "LineSegmentSketch",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 269,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "major_raw_contour",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "line_segment_sketch",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 253,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "pub",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "major_concave_components",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "~",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "[",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "]",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "ConcaveComponent",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 270,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "major_line_segment_sketch",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ".",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "concave_components",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 3,
            },
        ),
        TraceViewData {
            trace_kind: ValItem,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "val",
                            token_class: OtherKeyword,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "main",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: ":",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Class",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "MnistLabel",
                            token_class: Ident,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "=",
                            token_class: Punctuation,
                            spaces_before: 1,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: true,
        },
    ),
    (
        Trace(
            Id {
                value: 271,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "is_one",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 272,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "is_six",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 273,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "is_zero",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 274,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "is_seven",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 275,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "is_eight",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 276,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "is_three",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 277,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "is_nine",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 278,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "is_five",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 279,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "is_two",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "?",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
    (
        Trace(
            Id {
                value: 280,
            },
        ),
        TraceViewData {
            trace_kind: EagerStmt,
            lines_data: [
                TraceViewLineData {
                    tokens_data: [
                        TraceViewTokenData {
                            text: "Class",
                            token_class: Ident,
                            spaces_before: 4,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "::",
                            token_class: Punctuation,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                        TraceViewTokenData {
                            text: "Unknown",
                            token_class: Ident,
                            spaces_before: 0,
                            assoc_trace_id: None,
                        },
                    ],
                },
            ],
            have_subtraces: false,
        },
    ),
]
```