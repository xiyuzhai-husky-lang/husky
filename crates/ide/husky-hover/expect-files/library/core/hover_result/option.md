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
                                value: "Other\ntoken_idx = 1;\n\ntoken = Keyword(\n    Type(\n        Enum,\n    ),\n);\n\ntoken_info = None;\n\n\n",
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
                                value: "\ntoken_idx = 2;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 96,\n            },\n        ),\n    ),\n);\n\ntoken_info = Entity(\n    Some(\n        ModuleItem(\n            Type(\n                TypePath(\n                    Id {\n                        value: 30,\n                    },\n                ),\n            ),\n        ),\n    ),\n    Some(\n        ModuleItem {\n            module_item_kind: Type(\n                Enum,\n            ),\n            connection: Connected,\n        },\n    ),\n);\n\n\n",
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
                                    character: 15,
                                },
                                end: Position {
                                    line: 2,
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
                4,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 4;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 97,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: ImplicitParameter {\n        implicit_parameter_kind: Type {\n            ident_token: IdentToken {\n                ident: Ident(\n                    Word(\n                        Id {\n                            value: 97,\n                        },\n                    ),\n                ),\n                token_idx: TokenIdx(\n                    4,\n                ),\n            },\n        },\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 93,\n        },\n    ),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        5,\n    ),\n    access_end: None,\n    variant: ImplicitParameter {\n        implicit_parameter_variant: Type {\n            ident_token: IdentToken {\n                ident: Ident(\n                    Word(\n                        Id {\n                            value: 97,\n                        },\n                    ),\n                ),\n                token_idx: TokenIdx(\n                    4,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 16,
                                },
                                end: Position {
                                    line: 2,
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
                5,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 5;\n\ntoken = Punctuation(\n    RaOrGt,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 17,
                                },
                                end: Position {
                                    line: 2,
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
                6,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 6;\n\ntoken = Punctuation(\n    Vertical,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 3,
                                    character: 0,
                                },
                                end: Position {
                                    line: 3,
                                    character: 1,
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
                                value: "\ntoken_idx = 7;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 98,\n            },\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 3,
                                    character: 2,
                                },
                                end: Position {
                                    line: 3,
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
                8,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 8;\n\ntoken = Punctuation(\n    Bra(\n        Par,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 3,
                                    character: 6,
                                },
                                end: Position {
                                    line: 3,
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
                9,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 9;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 97,\n            },\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 3,
                                    character: 7,
                                },
                                end: Position {
                                    line: 3,
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
                10,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 10;\n\ntoken = Punctuation(\n    Ket(\n        Par,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 3,
                                    character: 8,
                                },
                                end: Position {
                                    line: 3,
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
                11,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 11;\n\ntoken = Punctuation(\n    Vertical,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 0,
                                },
                                end: Position {
                                    line: 4,
                                    character: 1,
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
                12,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 12;\n\ntoken = Ident(\n    Ident(\n        Word(\n            Id {\n                value: 99,\n            },\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 2,
                                },
                                end: Position {
                                    line: 4,
                                    character: 6,
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