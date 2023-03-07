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
                                value: "\ntoken_idx = 0;\n\ntoken = Attr(\n    Pub,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 0,
                                },
                                end: Position {
                                    line: 2,
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
                1,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 1;\n\ntoken = Keyword(\n    Type(\n        Type,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 4,
                                },
                                end: Position {
                                    line: 2,
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
                2,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 2;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 91,\n            },\n        ),\n    ),\n);\n\ntoken_info = Entity(\n    Some(\n        ModuleItem(\n            Type(\n                TypePath(\n                    Id {\n                        value: 27,\n                    },\n                ),\n            ),\n        ),\n    ),\n    Some(\n        ModuleItem {\n            module_item_kind: Type(\n                Extern,\n            ),\n            connection: Connected,\n        },\n    ),\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 9,
                                },
                                end: Position {
                                    line: 2,
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
                3,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 3;\n\ntoken = Punctuation(\n    LaOrLt,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 13,
                                },
                                end: Position {
                                    line: 2,
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
                4,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 4;\n\ntoken = Keyword(\n    Pattern(\n        Covariant,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 14,
                                },
                                end: Position {
                                    line: 2,
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
                5,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 5;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 30,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: ImplicitParameter {\n        implicit_parameter_kind: Type {\n            ident_token: IdentifierToken {\n                ident: Identifier(\n                    Word(\n                        Id {\n                            value: 30,\n                        },\n                    ),\n                ),\n                token_idx: TokenIdx(\n                    5,\n                ),\n            },\n        },\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 14,\n        },\n    ),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        6,\n    ),\n    access_end: None,\n    variant: ImplicitParameter {\n        implicit_parameter_variant: Type {\n            ident_token: IdentifierToken {\n                ident: Identifier(\n                    Word(\n                        Id {\n                            value: 30,\n                        },\n                    ),\n                ),\n                token_idx: TokenIdx(\n                    5,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 24,
                                },
                                end: Position {
                                    line: 2,
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
                6,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 6;\n\ntoken = Punctuation(\n    RaOrGt,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 25,
                                },
                                end: Position {
                                    line: 2,
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
                7,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 7;\n\ntoken = Punctuation(\n    Semicolon,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 26,
                                },
                                end: Position {
                                    line: 2,
                                    character: 27,
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