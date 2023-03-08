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
                                value: "\ntoken_idx = 22;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 145,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: Parameter {\n        pattern_symbol_idx: 0,\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 317,\n        },\n    ),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        23,\n    ),\n    access_end: None,\n    variant: RegularParameter {\n        ident: Ident(\n            Word(\n                Id {\n                    value: 145,\n                },\n            ),\n        ),\n        pattern_symbol_idx: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 19,
                                },
                                end: Position {
                                    line: 4,
                                    character: 38,
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
                                value: "Other\ntoken_idx = 33;\n\ntoken = Keyword(\n    Stmt(\n        Let,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 5,
                                    character: 4,
                                },
                                end: Position {
                                    line: 5,
                                    character: 7,
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
                                value: "\ntoken_idx = 44;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 383,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: LetVariable {\n        pattern_symbol_idx: 1,\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 321,\n        },\n    ),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        45,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                221,\n            ),\n        ),\n    ),\n    variant: LetVariable {\n        ident: Ident(\n            Word(\n                Id {\n                    value: 383,\n                },\n            ),\n        ),\n        pattern_symbol_idx: 1,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 8,
                                },
                                end: Position {
                                    line: 6,
                                    character: 28,
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
                                value: "\ntoken_idx = 55;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 244,\n            },\n        ),\n    ),\n);\n\ntoken_info = Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 68,
                                },
                                end: Position {
                                    line: 6,
                                    character: 80,
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
                                value: "\ntoken_idx = 66;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 382,\n            },\n        ),\n    ),\n);\n\ntoken_info = InheritedSymbol {\n    inherited_symbol_idx: 1,\n    inherited_symbol_kind: RegularParameter {\n        ident: Ident(\n            Word(\n                Id {\n                    value: 382,\n                },\n            ),\n        ),\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 321,\n        },\n    ),\n};\n\nInheritedSymbol {\n    parent_symbol_idx: Current(\n        1,\n    ),\n    kind: RegularParameter {\n        ident: Ident(\n            Word(\n                Id {\n                    value: 382,\n                },\n            ),\n        ),\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 61,
                                },
                                end: Position {
                                    line: 7,
                                    character: 66,
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
                                value: "Other\ntoken_idx = 77;\n\ntoken = Keyword(\n    Stmt(\n        Let,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 8,
                                    character: 4,
                                },
                                end: Position {
                                    line: 8,
                                    character: 7,
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
                                value: "\ntoken_idx = 88;\n\ntoken = Punctuation(\n    Binary(\n        Comparison(\n            Eq,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 10,
                                    character: 43,
                                },
                                end: Position {
                                    line: 10,
                                    character: 45,
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
                                value: "\ntoken_idx = 99;\n\ntoken = Punctuation(\n    Eq,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 12,
                                    character: 30,
                                },
                                end: Position {
                                    line: 12,
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
                110,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 110;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 341,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: LetVariable {\n        pattern_symbol_idx: 0,\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 321,\n        },\n    ),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        35,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                221,\n            ),\n        ),\n    ),\n    variant: LetVariable {\n        ident: Ident(\n            Word(\n                Id {\n                    value: 341,\n                },\n            ),\n        ),\n        pattern_symbol_idx: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 12,
                                    character: 70,
                                },
                                end: Position {
                                    line: 12,
                                    character: 71,
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
                                value: "\ntoken_idx = 121;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 387,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 5,\n    current_symbol_kind: LetVariable {\n        pattern_symbol_idx: 5,\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 321,\n        },\n    ),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        99,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                215,\n            ),\n        ),\n    ),\n    variant: LetVariable {\n        ident: Ident(\n            Word(\n                Id {\n                    value: 387,\n                },\n            ),\n        ),\n        pattern_symbol_idx: 5,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 13,
                                    character: 44,
                                },
                                end: Position {
                                    line: 13,
                                    character: 61,
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
                                value: "\ntoken_idx = 132;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 244,\n            },\n        ),\n    ),\n);\n\ntoken_info = Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 14,
                                    character: 59,
                                },
                                end: Position {
                                    line: 14,
                                    character: 71,
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
                                value: "\ntoken_idx = 143;\n\ntoken = Punctuation(\n    Dot,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 15,
                                    character: 51,
                                },
                                end: Position {
                                    line: 15,
                                    character: 52,
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
                                value: "Other\ntoken_idx = 154;\n\ntoken = Keyword(\n    Pattern(\n        Mut,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 17,
                                    character: 12,
                                },
                                end: Position {
                                    line: 17,
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
                165,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 165;\n\ntoken = Punctuation(\n    Bra(\n        Box,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 18,
                                    character: 58,
                                },
                                end: Position {
                                    line: 18,
                                    character: 59,
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
                                value: "\ntoken_idx = 176;\n\ntoken = Punctuation(\n    Binary(\n        Comparison(\n            Leq,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 19,
                                    character: 35,
                                },
                                end: Position {
                                    line: 19,
                                    character: 37,
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
                                value: "\ntoken_idx = 187;\n\ntoken = Punctuation(\n    Dot,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 20,
                                    character: 50,
                                },
                                end: Position {
                                    line: 20,
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
                198,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 198;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 388,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 8,\n    current_symbol_kind: LetVariable {\n        pattern_symbol_idx: 7,\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 321,\n        },\n    ),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        156,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                215,\n            ),\n        ),\n    ),\n    variant: LetVariable {\n        ident: Ident(\n            Word(\n                Id {\n                    value: 388,\n                },\n            ),\n        ),\n        pattern_symbol_idx: 7,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 21,
                                    character: 12,
                                },
                                end: Position {
                                    line: 21,
                                    character: 29,
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
                                    character: 93,
                                },
                                end: Position {
                                    line: 21,
                                    character: 94,
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
                220,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 220;\n\ntoken = Literal(\n    Integer(\n        Unspecified,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 24,
                                    character: 53,
                                },
                                end: Position {
                                    line: 24,
                                    character: 54,
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