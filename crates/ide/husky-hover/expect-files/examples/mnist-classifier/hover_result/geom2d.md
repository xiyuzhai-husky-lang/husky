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
                                value: "Other\ntoken_idx = 0;\n\ntoken = Keyword(\n    Type(\n        Struct,\n    ),\n);\n\ntoken_info = None;\n\n\n",
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
                                    character: 6,
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
                35,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 35;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 158,\n            },\n        ),\n    ),\n);\n\ntoken_info = InheritedSymbol {\n    inherited_symbol_idx: 0,\n    inherited_symbol_kind: Parameter {\n        original_local_symbol_idx: 0,\n    },\n    expr_sheet: ExprSheet(\n        Id {\n            value: 218,\n        },\n    ),\n};\n\nInheritedSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 131,\n            },\n        ),\n    ),\n    kind: Parameter {\n        original_local_symbol_idx: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 22,
                                },
                                end: Position {
                                    line: 7,
                                    character: 27,
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
                70,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 70;\n\ntoken = Punctuation(\n    Colon,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 12,
                                    character: 39,
                                },
                                end: Position {
                                    line: 12,
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
                105,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "This is a paradigm\ntoken_idx = 105;\n\ntoken = Keyword(\n    Paradigm(\n        Func,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 18,
                                    character: 4,
                                },
                                end: Position {
                                    line: 18,
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
                140,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 140;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 30,\n            },\n        ),\n    ),\n);\n\ntoken_info = Entity(\n    Some(\n        ModuleItem(\n            Type(\n                TypePath(\n                    Id {\n                        value: 12,\n                    },\n                ),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 27,
                                    character: 7,
                                },
                                end: Position {
                                    line: 27,
                                    character: 10,
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
                175,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 175;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 24,\n            },\n        ),\n    ),\n);\n\ntoken_info = InheritedSymbol {\n    inherited_symbol_idx: 0,\n    inherited_symbol_kind: Parameter {\n        original_local_symbol_idx: 0,\n    },\n    expr_sheet: ExprSheet(\n        Id {\n            value: 224,\n        },\n    ),\n};\n\nInheritedSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 24,\n            },\n        ),\n    ),\n    kind: Parameter {\n        original_local_symbol_idx: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 36,
                                    character: 17,
                                },
                                end: Position {
                                    line: 36,
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
                210,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 210;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 24,\n            },\n        ),\n    ),\n);\n\ntoken_info = LocalSymbol {\n    local_symbol_idx: 0,\n    local_symbol_kind: Parameter {\n        pattern_symbol: 0,\n    },\n    expr_sheet: ExprSheet(\n        Id {\n            value: 199,\n        },\n    ),\n};\n\nLocalSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 24,\n            },\n        ),\n    ),\n    access_start: TokenIdx(\n        211,\n    ),\n    access_end: None,\n    kind: Parameter {\n        pattern_symbol: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 41,
                                    character: 13,
                                },
                                end: Position {
                                    line: 41,
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
                245,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 245;\n\ntoken = Punctuation(\n    Star,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 45,
                                    character: 24,
                                },
                                end: Position {
                                    line: 45,
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
                280,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 280;\n\ntoken = Punctuation(\n    Colon,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 49,
                                    character: 34,
                                },
                                end: Position {
                                    line: 49,
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
                315,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 315;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 185,\n            },\n        ),\n    ),\n);\n\ntoken_info = Entity(\n    Some(\n        ModuleItem(\n            Type(\n                TypePath(\n                    Id {\n                        value: 22,\n                    },\n                ),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 58,
                                    character: 38,
                                },
                                end: Position {
                                    line: 58,
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
                350,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 350;\n\ntoken = Keyword(\n    Stmt(\n        Assert,\n    ),\n);\n\ntoken_info = None;\n\n\n",
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
                                    character: 14,
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
                385,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 385;\n\ntoken = Punctuation(\n    Bra(\n        Par,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 69,
                                    character: 72,
                                },
                                end: Position {
                                    line: 69,
                                    character: 73,
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
                                value: "\ntoken_idx = 420;\n\ntoken = Punctuation(\n    Dot,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 76,
                                    character: 82,
                                },
                                end: Position {
                                    line: 76,
                                    character: 83,
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
                455,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 455;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 137,\n            },\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 86,
                                    character: 15,
                                },
                                end: Position {
                                    line: 86,
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
                490,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 490;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 261,\n            },\n        ),\n    ),\n);\n\ntoken_info = LocalSymbol {\n    local_symbol_idx: 1,\n    local_symbol_kind: LetVariable {\n        pattern_symbol: 1,\n    },\n    expr_sheet: ExprSheet(\n        Id {\n            value: 231,\n        },\n    ),\n};\n\nLocalSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 261,\n            },\n        ),\n    ),\n    access_start: TokenIdx(\n        466,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                495,\n            ),\n        ),\n    ),\n    kind: LetVariable {\n        pattern_symbol: 1,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 91,
                                    character: 12,
                                },
                                end: Position {
                                    line: 91,
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
                525,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 525;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 179,\n            },\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 100,
                                    character: 16,
                                },
                                end: Position {
                                    line: 100,
                                    character: 27,
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
                560,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 560;\n\ntoken = Punctuation(\n    Bra(\n        Par,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 108,
                                    character: 33,
                                },
                                end: Position {
                                    line: 108,
                                    character: 34,
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
                595,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 595;\n\ntoken = Punctuation(\n    Ket(\n        Par,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 114,
                                    character: 41,
                                },
                                end: Position {
                                    line: 114,
                                    character: 42,
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
                                value: "\ntoken_idx = 630;\n\ntoken = Punctuation(\n    Dot,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 124,
                                    character: 12,
                                },
                                end: Position {
                                    line: 124,
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
                665,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 665;\n\ntoken = Punctuation(\n    Binary(\n        Curry,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 135,
                                    character: 16,
                                },
                                end: Position {
                                    line: 135,
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
                700,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 700;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 137,\n            },\n        ),\n    ),\n);\n\ntoken_info = Field;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 145,
                                    character: 15,
                                },
                                end: Position {
                                    line: 145,
                                    character: 18,
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