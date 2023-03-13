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
                43,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 43;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 181,\n            },\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 12,
                                    character: 4,
                                },
                                end: Position {
                                    line: 12,
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
                86,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 86;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 186,\n            },\n        ),\n    ),\n);\n\ntoken_info = Entity(\n    Some(\n        ModuleItem(\n            Type(\n                TypePath(\n                    Id {\n                        value: 42,\n                    },\n                ),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 25,
                                    character: 5,
                                },
                                end: Position {
                                    line: 25,
                                    character: 23,
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
                129,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 129;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 191,\n            },\n        ),\n    ),\n);\n\ntoken_info = Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 33,
                                    character: 21,
                                },
                                end: Position {
                                    line: 33,
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
                172,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 172;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 188,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: LetVariable {\n        pattern_symbol_idx: 1,\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 129,\n        },\n    ),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        173,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                213,\n            ),\n        ),\n    ),\n    variant: LetVariable {\n        ident: Ident(\n            Word(\n                Id {\n                    value: 188,\n                },\n            ),\n        ),\n        pattern_symbol_idx: 1,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 41,
                                    character: 12,
                                },
                                end: Position {
                                    line: 41,
                                    character: 24,
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
                215,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 215;\n\ntoken = Punctuation(\n    Binary(\n        Curry,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 48,
                                    character: 22,
                                },
                                end: Position {
                                    line: 48,
                                    character: 24,
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
                258,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 258;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 202,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: LetVariable {\n        pattern_symbol_idx: 0,\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 131,\n        },\n    ),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        259,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                284,\n            ),\n        ),\n    ),\n    variant: LetVariable {\n        ident: Ident(\n            Word(\n                Id {\n                    value: 202,\n                },\n            ),\n        ),\n        pattern_symbol_idx: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 57,
                                    character: 16,
                                },
                                end: Position {
                                    line: 57,
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
                301,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 301;\n\ntoken = Punctuation(\n    Dot,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 65,
                                    character: 19,
                                },
                                end: Position {
                                    line: 65,
                                    character: 20,
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
                344,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 344;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 178,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 4,\n    current_symbol_kind: LetVariable {\n        pattern_symbol_idx: 4,\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 132,\n        },\n    ),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        345,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                406,\n            ),\n        ),\n    ),\n    variant: LetVariable {\n        ident: Ident(\n            Word(\n                Id {\n                    value: 178,\n                },\n            ),\n        ),\n        pattern_symbol_idx: 4,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 73,
                                    character: 16,
                                },
                                end: Position {
                                    line: 73,
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
                387,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 387;\n\ntoken = Punctuation(\n    Bra(\n        Box,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 78,
                                    character: 35,
                                },
                                end: Position {
                                    line: 78,
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
                430,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "This is a paradigm\ntoken_idx = 430;\n\ntoken = Keyword(\n    Form(\n        Proc,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 92,
                                    character: 4,
                                },
                                end: Position {
                                    line: 92,
                                    character: 8,
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
                473,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 473;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 194,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: LetVariable {\n        pattern_symbol_idx: 1,\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 135,\n        },\n    ),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        452,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                493,\n            ),\n        ),\n    ),\n    variant: LetVariable {\n        ident: Ident(\n            Word(\n                Id {\n                    value: 194,\n                },\n            ),\n        ),\n        pattern_symbol_idx: 1,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 99,
                                    character: 21,
                                },
                                end: Position {
                                    line: 99,
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
                516,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 516;\n\ntoken = Literal(\n    Integer(\n        Unspecified,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 106,
                                    character: 20,
                                },
                                end: Position {
                                    line: 106,
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
                559,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "This is a paradigm\ntoken_idx = 559;\n\ntoken = Keyword(\n    Form(\n        Proc,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 117,
                                    character: 0,
                                },
                                end: Position {
                                    line: 117,
                                    character: 4,
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
                602,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 602;\n\ntoken = Punctuation(\n    Vertical,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 119,
                                    character: 23,
                                },
                                end: Position {
                                    line: 119,
                                    character: 24,
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
                645,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "This is a paradigm\ntoken_idx = 645;\n\ntoken = Keyword(\n    Form(\n        Proc,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 125,
                                    character: 4,
                                },
                                end: Position {
                                    line: 125,
                                    character: 8,
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
                688,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 688;\n\ntoken = Punctuation(\n    Eq,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 130,
                                    character: 18,
                                },
                                end: Position {
                                    line: 130,
                                    character: 19,
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
                731,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 731;\n\ntoken = Punctuation(\n    Eq,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 136,
                                    character: 21,
                                },
                                end: Position {
                                    line: 136,
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
                774,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 774;\n\ntoken = Punctuation(\n    Exclamation,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 141,
                                    character: 23,
                                },
                                end: Position {
                                    line: 141,
                                    character: 24,
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
                817,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 817;\n\ntoken = Punctuation(\n    Comma,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 148,
                                    character: 68,
                                },
                                end: Position {
                                    line: 148,
                                    character: 69,
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
                860,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 860;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 192,\n            },\n        ),\n    ),\n);\n\ntoken_info = Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 154,
                                    character: 19,
                                },
                                end: Position {
                                    line: 154,
                                    character: 23,
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