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
                15,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 15;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 154,\n            },\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 1,
                                    character: 13,
                                },
                                end: Position {
                                    line: 1,
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
                30,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 30;\n\ntoken = Punctuation(\n    ColonColon,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 33,
                                },
                                end: Position {
                                    line: 2,
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
                45,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 45;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 146,\n            },\n        ),\n    ),\n);\n\ntoken_info = UseExpr {\n    use_expr_idx: 22,\n    rule_idx: UseExprRuleIdx(\n        7,\n    ),\n    state: Resolved {\n        original_symbol: Submodule(\n            SubmoduleSymbol(\n                Id {\n                    value: 22,\n                },\n            ),\n        ),\n    },\n};\n\nuse\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 11,
                                },
                                end: Position {
                                    line: 4,
                                    character: 16,
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
                60,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 60;\n\ntoken = Punctuation(\n    ColonColon,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 49,
                                },
                                end: Position {
                                    line: 6,
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
                75,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 75;\n\ntoken = Punctuation(\n    Comma,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 10,
                                    character: 57,
                                },
                                end: Position {
                                    line: 10,
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
                90,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 90;\n\ntoken = Punctuation(\n    Ket(\n        Par,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 23,
                                    character: 42,
                                },
                                end: Position {
                                    line: 23,
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
                105,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 105;\n\ntoken = Punctuation(\n    Dot,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 25,
                                    character: 35,
                                },
                                end: Position {
                                    line: 25,
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
                120,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 120;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 447,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 2,\n    current_symbol_kind: LetVariable {\n        pattern_symbol_idx: 2,\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 201,\n        },\n    ),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        121,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                209,\n            ),\n        ),\n    ),\n    variant: LetVariable {\n        ident: Identifier(\n            Word(\n                Id {\n                    value: 447,\n                },\n            ),\n        ),\n        pattern_symbol_idx: 2,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 27,
                                    character: 8,
                                },
                                end: Position {
                                    line: 27,
                                    character: 12,
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
                135,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 135;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 300,\n            },\n        ),\n    ),\n);\n\ntoken_info = Field;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 29,
                                    character: 20,
                                },
                                end: Position {
                                    line: 29,
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
                150,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 150;\n\ntoken = Punctuation(\n    Dot,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 32,
                                    character: 34,
                                },
                                end: Position {
                                    line: 32,
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
                165,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 165;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 449,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 4,\n    current_symbol_kind: LetVariable {\n        pattern_symbol_idx: 6,\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 201,\n        },\n    ),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        166,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                209,\n            ),\n        ),\n    ),\n    variant: LetVariable {\n        ident: Identifier(\n            Word(\n                Id {\n                    value: 449,\n                },\n            ),\n        ),\n        pattern_symbol_idx: 6,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 34,
                                    character: 8,
                                },
                                end: Position {
                                    line: 34,
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
                180,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 180;\n\ntoken = Keyword(\n    Stmt(\n        Let,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 36,
                                    character: 4,
                                },
                                end: Position {
                                    line: 36,
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
                195,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 195;\n\ntoken = Punctuation(\n    Dot,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 40,
                                    character: 29,
                                },
                                end: Position {
                                    line: 40,
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
                210,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 210;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 446,\n            },\n        ),\n    ),\n);\n\ntoken_info = Entity(\n    Some(\n        ModuleItem(\n            Form(\n                FormPath(\n                    Id {\n                        value: 40,\n                    },\n                ),\n            ),\n        ),\n    ),\n    Some(\n        ModuleItem {\n            module_item_kind: Form(\n                Function,\n            ),\n            connection: Connected,\n        },\n    ),\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 46,
                                    character: 5,
                                },
                                end: Position {
                                    line: 46,
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
                225,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 225;\n\ntoken = Punctuation(\n    Dot,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 47,
                                    character: 15,
                                },
                                end: Position {
                                    line: 47,
                                    character: 16,
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
                240,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 240;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 232,\n            },\n        ),\n    ),\n);\n\ntoken_info = Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 49,
                                    character: 21,
                                },
                                end: Position {
                                    line: 49,
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
                255,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 255;\n\ntoken = Keyword(\n    Stmt(\n        Let,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 52,
                                    character: 4,
                                },
                                end: Position {
                                    line: 52,
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
                270,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 270;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 223,\n            },\n        ),\n    ),\n);\n\ntoken_info = InheritedSymbol {\n    inherited_symbol_idx: 0,\n    inherited_symbol_kind: RegularParameter {\n        ident: Identifier(\n            Word(\n                Id {\n                    value: 223,\n                },\n            ),\n        ),\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 203,\n        },\n    ),\n};\n\nInheritedSymbol {\n    parent_symbol_idx: Current(\n        0,\n    ),\n    kind: RegularParameter {\n        ident: Identifier(\n            Word(\n                Id {\n                    value: 223,\n                },\n            ),\n        ),\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 54,
                                    character: 5,
                                },
                                end: Position {
                                    line: 54,
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
                285,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 285;\n\ntoken = Punctuation(\n    Binary(\n        Curry,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 56,
                                    character: 33,
                                },
                                end: Position {
                                    line: 56,
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
                300,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 300;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 211,\n            },\n        ),\n    ),\n);\n\ntoken_info = Field;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 58,
                                    character: 15,
                                },
                                end: Position {
                                    line: 58,
                                    character: 16,
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