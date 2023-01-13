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
                42,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 42;\n\ntoken = Punctuation(\n    Ambersand,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 11,
                                    character: 16,
                                },
                                end: Position {
                                    line: 11,
                                    character: 17,
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
                84,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 84;\n\ntoken = Punctuation(\n    Bra(\n        Box,\n    ),\n);\n\ntoken_info = BoxPrefix;\n\nbox prefix\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 25,
                                    character: 25,
                                },
                                end: Position {
                                    line: 25,
                                    character: 26,
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
                126,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 126;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 120,\n            },\n        ),\n    ),\n);\n\ntoken_info = LocalSymbol {\n    local_symbol_idx: 0,\n    local_symbol_kind: LetVariable {\n        pattern_symbol: 0,\n    },\n    expr_sheet: ExprSheet(\n        Id {\n            value: 87,\n        },\n    ),\n};\n\nLocalSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 127,\n            },\n        ),\n    ),\n    access_start: TokenIdx(\n        100,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                153,\n            ),\n        ),\n    ),\n    kind: LetVariable {\n        pattern_symbol: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 33,
                                    character: 8,
                                },
                                end: Position {
                                    line: 33,
                                    character: 15,
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
                168,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 168;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 127,\n            },\n        ),\n    ),\n);\n\ntoken_info = Field;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 40,
                                    character: 32,
                                },
                                end: Position {
                                    line: 40,
                                    character: 44,
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
                210,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 210;\n\ntoken = Keyword(\n    Stmt(\n        Let,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 49,
                                    character: 8,
                                },
                                end: Position {
                                    line: 49,
                                    character: 11,
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
                252,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 252;\n\ntoken = Literal(\n    Integer(\n        Unspecified,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 57,
                                    character: 12,
                                },
                                end: Position {
                                    line: 57,
                                    character: 13,
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
                294,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 294;\n\ntoken = Keyword(\n    Stmt(\n        Let,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 66,
                                    character: 8,
                                },
                                end: Position {
                                    line: 66,
                                    character: 11,
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
                336,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 336;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 115,\n            },\n        ),\n    ),\n);\n\ntoken_info = LocalSymbol {\n    local_symbol_idx: 4,\n    local_symbol_kind: LetVariable {\n        pattern_symbol: 4,\n    },\n    expr_sheet: ExprSheet(\n        Id {\n            value: 91,\n        },\n    ),\n};\n\nLocalSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 117,\n            },\n        ),\n    ),\n    access_start: TokenIdx(\n        329,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                386,\n            ),\n        ),\n    ),\n    kind: LetVariable {\n        pattern_symbol: 4,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 73,
                                    character: 30,
                                },
                                end: Position {
                                    line: 73,
                                    character: 39,
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
                378,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 378;\n\ntoken = Punctuation(\n    Comma,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 79,
                                    character: 21,
                                },
                                end: Position {
                                    line: 79,
                                    character: 22,
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
                420,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 420;\n\ntoken = Keyword(\n    Stmt(\n        Let,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 92,
                                    character: 8,
                                },
                                end: Position {
                                    line: 92,
                                    character: 11,
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
                462,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 462;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 141,\n            },\n        ),\n    ),\n);\n\ntoken_info = Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 99,
                                    character: 42,
                                },
                                end: Position {
                                    line: 99,
                                    character: 46,
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
                504,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 504;\n\ntoken = Keyword(\n    Stmt(\n        Break,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 108,
                                    character: 16,
                                },
                                end: Position {
                                    line: 108,
                                    character: 21,
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
                546,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 546;\n\ntoken = Keyword(\n    Liason(\n        Mut,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 117,
                                    character: 8,
                                },
                                end: Position {
                                    line: 117,
                                    character: 11,
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
                588,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 588;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 155,\n            },\n        ),\n    ),\n);\n\ntoken_info = LocalSymbol {\n    local_symbol_idx: 0,\n    local_symbol_kind: LetVariable {\n        pattern_symbol: 0,\n    },\n    expr_sheet: ExprSheet(\n        Id {\n            value: 84,\n        },\n    ),\n};\n\nLocalSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 59,\n            },\n        ),\n    ),\n    access_start: TokenIdx(\n        548,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                616,\n            ),\n        ),\n    ),\n    kind: LetVariable {\n        pattern_symbol: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 119,
                                    character: 10,
                                },
                                end: Position {
                                    line: 119,
                                    character: 11,
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
                630,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 630;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 158,\n            },\n        ),\n    ),\n);\n\ntoken_info = LocalSymbol {\n    local_symbol_idx: 0,\n    local_symbol_kind: LetVariable {\n        pattern_symbol: 0,\n    },\n    expr_sheet: ExprSheet(\n        Id {\n            value: 85,\n        },\n    ),\n};\n\nLocalSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 158,\n            },\n        ),\n    ),\n    access_start: TokenIdx(\n        631,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                840,\n            ),\n        ),\n    ),\n    kind: LetVariable {\n        pattern_symbol: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 125,
                                    character: 12,
                                },
                                end: Position {
                                    line: 125,
                                    character: 18,
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
                672,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 672;\n\ntoken = Keyword(\n    Stmt(\n        Let,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 131,
                                    character: 12,
                                },
                                end: Position {
                                    line: 131,
                                    character: 15,
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
                714,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 714;\n\ntoken = Literal(\n    Integer(\n        Unspecified,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 137,
                                    character: 32,
                                },
                                end: Position {
                                    line: 137,
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
                756,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 756;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 164,\n            },\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 143,
                                    character: 31,
                                },
                                end: Position {
                                    line: 143,
                                    character: 36,
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
                798,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 798;\n\ntoken = Punctuation(\n    Binary(\n        Comparison(\n            Neq,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 148,
                                    character: 31,
                                },
                                end: Position {
                                    line: 148,
                                    character: 33,
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