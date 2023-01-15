Ok(
    [
        (
            TokenIdx(
                0,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 0;\n\ntoken = Keyword(\n    Use,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 0,
                                    character: 0,
                                },
                                end: Position {
                                    line: 0,
                                    character: 3,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                11,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 11;\n\ntoken = Punctuation(\n    Star,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 1,
                                    character: 24,
                                },
                                end: Position {
                                    line: 1,
                                    character: 25,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                22,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 22;\n\ntoken = Punctuation(\n    Colon,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 34,
                                },
                                end: Position {
                                    line: 4,
                                    character: 35,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                33,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 33;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 278,\n            },\n        ),\n    ),\n);\n\ntoken_info = LocalSymbol {\n    local_symbol_idx: 0,\n    local_symbol_kind: LetVariable {\n        pattern_symbol: 0,\n    },\n    expr_sheet: ExprSheet(\n        Id {\n            value: 274,\n        },\n    ),\n};\n\nLocalSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 278,\n            },\n        ),\n    ),\n    access_start: TokenIdx(\n        34,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                214,\n            ),\n        ),\n    ),\n    kind: LetVariable {\n        pattern_symbol: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 5,
                                    character: 8,
                                },
                                end: Position {
                                    line: 5,
                                    character: 9,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                44,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 44;\n\ntoken = Punctuation(\n    Binary(\n        Assign(\n            None,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 29,
                                },
                                end: Position {
                                    line: 6,
                                    character: 30,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                55,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 55;\n\ntoken = Punctuation(\n    Bra(\n        Par,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 80,
                                },
                                end: Position {
                                    line: 6,
                                    character: 81,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                66,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 66;\n\ntoken = Punctuation(\n    Binary(\n        PureClosed(\n            Sub,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 66,
                                },
                                end: Position {
                                    line: 7,
                                    character: 67,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                77,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 77;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 322,\n            },\n        ),\n    ),\n);\n\ntoken_info = LocalSymbol {\n    local_symbol_idx: 3,\n    local_symbol_kind: LetVariable {\n        pattern_symbol: 3,\n    },\n    expr_sheet: ExprSheet(\n        Id {\n            value: 274,\n        },\n    ),\n};\n\nLocalSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 322,\n            },\n        ),\n    ),\n    access_start: TokenIdx(\n        78,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                214,\n            ),\n        ),\n    ),\n    kind: LetVariable {\n        pattern_symbol: 3,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 8,
                                    character: 8,
                                },
                                end: Position {
                                    line: 8,
                                    character: 43,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                88,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 88;\n\ntoken = Literal(\n    Integer(\n        Unspecified,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 10,
                                    character: 46,
                                },
                                end: Position {
                                    line: 10,
                                    character: 47,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                99,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 99;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 83,\n            },\n        ),\n    ),\n);\n\ntoken_info = InheritedSymbol {\n    inherited_symbol_idx: 1,\n    inherited_symbol_kind: Parameter {\n        original_local_symbol_idx: 1,\n    },\n    expr_sheet: ExprSheet(\n        Id {\n            value: 274,\n        },\n    ),\n};\n\nInheritedSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 319,\n            },\n        ),\n    ),\n    kind: Parameter {\n        original_local_symbol_idx: 1,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 12,
                                    character: 32,
                                },
                                end: Position {
                                    line: 12,
                                    character: 51,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                110,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 110;\n\ntoken = Punctuation(\n    Ket(\n        Box,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 12,
                                    character: 71,
                                },
                                end: Position {
                                    line: 12,
                                    character: 72,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                121,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 121;\n\ntoken = Punctuation(\n    Dot,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 13,
                                    character: 61,
                                },
                                end: Position {
                                    line: 13,
                                    character: 62,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                132,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 132;\n\ntoken = Punctuation(\n    Bra(\n        Par,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 14,
                                    character: 71,
                                },
                                end: Position {
                                    line: 14,
                                    character: 72,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                143,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 143;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 137,\n            },\n        ),\n    ),\n);\n\ntoken_info = Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 15,
                                    character: 52,
                                },
                                end: Position {
                                    line: 15,
                                    character: 55,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                154,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 154;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 325,\n            },\n        ),\n    ),\n);\n\ntoken_info = LocalSymbol {\n    local_symbol_idx: 8,\n    local_symbol_kind: LetVariable {\n        pattern_symbol: 7,\n    },\n    expr_sheet: ExprSheet(\n        Id {\n            value: 274,\n        },\n    ),\n};\n\nLocalSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 325,\n            },\n        ),\n    ),\n    access_start: TokenIdx(\n        155,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                214,\n            ),\n        ),\n    ),\n    kind: LetVariable {\n        pattern_symbol: 7,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 17,
                                    character: 16,
                                },
                                end: Position {
                                    line: 17,
                                    character: 33,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                165,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 165;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 319,\n            },\n        ),\n    ),\n);\n\ntoken_info = InheritedSymbol {\n    inherited_symbol_idx: 0,\n    inherited_symbol_kind: Parameter {\n        original_local_symbol_idx: 0,\n    },\n    expr_sheet: ExprSheet(\n        Id {\n            value: 274,\n        },\n    ),\n};\n\nInheritedSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 83,\n            },\n        ),\n    ),\n    kind: Parameter {\n        original_local_symbol_idx: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 18,
                                    character: 59,
                                },
                                end: Position {
                                    line: 18,
                                    character: 64,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                176,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 176;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 145,\n            },\n        ),\n    ),\n);\n\ntoken_info = LocalSymbol {\n    local_symbol_idx: 10,\n    local_symbol_kind: FrameVariable(\n        69,\n    ),\n    expr_sheet: ExprSheet(\n        Id {\n            value: 274,\n        },\n    ),\n};\n\nLocalSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 145,\n            },\n        ),\n    ),\n    access_start: TokenIdx(\n        182,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                210,\n            ),\n        ),\n    ),\n    kind: FrameVariable(\n        69,\n    ),\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 19,
                                    character: 38,
                                },
                                end: Position {
                                    line: 19,
                                    character: 40,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                187,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 187;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 225,\n            },\n        ),\n    ),\n);\n\ntoken_info = Field;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 20,
                                    character: 51,
                                },
                                end: Position {
                                    line: 20,
                                    character: 58,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                198,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 198;\n\ntoken = Punctuation(\n    Binary(\n        Assign(\n            None,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 21,
                                    character: 30,
                                },
                                end: Position {
                                    line: 21,
                                    character: 31,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
        (
            TokenIdx(
                209,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 209;\n\ntoken = Punctuation(\n    Ket(\n        Par,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 21,
                                    character: 94,
                                },
                                end: Position {
                                    line: 21,
                                    character: 95,
                                },
                            },
                        ),
                    },
                    actions: [],
                },
            ),
        ),
    ],
)