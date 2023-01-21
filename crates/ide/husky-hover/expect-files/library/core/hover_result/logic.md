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
                2,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 2;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 10,\n            },\n        ),\n    ),\n);\n\ntoken_info = Entity(\n    Some(\n        ModuleItem(\n            Type(\n                TypePath(\n                    Id {\n                        value: 4,\n                    },\n                ),\n            ),\n        ),\n    ),\n    Some(\n        ModuleItem {\n            module_item_kind: Type(\n                Alien,\n            ),\n            connection: Connected,\n        },\n    ),\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 0,
                                    character: 9,
                                },
                                end: Position {
                                    line: 0,
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
                4,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 4;\n\ntoken = Attr(\n    Pub,\n);\n\ntoken_info = None;\n\n\n",
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
                6,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 6;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 11,\n            },\n        ),\n    ),\n);\n\ntoken_info = Entity(\n    Some(\n        ModuleItem(\n            Type(\n                TypePath(\n                    Id {\n                        value: 5,\n                    },\n                ),\n            ),\n        ),\n    ),\n    Some(\n        ModuleItem {\n            module_item_kind: Type(\n                Structure,\n            ),\n            connection: Connected,\n        },\n    ),\n);\n\n\n",
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
                8,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 8;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 12,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: ImplicitParameter {\n        implicit_parameter_kind: Type {\n            ident_token: IdentifierToken {\n                ident: Identifier(\n                    Word(\n                        Id {\n                            value: 12,\n                        },\n                    ),\n                ),\n                token_idx: TokenIdx(\n                    8,\n                ),\n            },\n        },\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 5,\n        },\n    ),\n};\n\nCurrentSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 12,\n            },\n        ),\n    ),\n    access_start: TokenIdx(\n        9,\n    ),\n    access_end: None,\n    variant: ImplicitParameter {\n        implicit_parameter_variant: Type {\n            ident_token: IdentifierToken {\n                ident: Identifier(\n                    Word(\n                        Id {\n                            value: 12,\n                        },\n                    ),\n                ),\n                token_idx: TokenIdx(\n                    8,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 23,
                                },
                                end: Position {
                                    line: 2,
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
                10,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 10;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 10,\n            },\n        ),\n    ),\n);\n\ntoken_info = Entity(\n    Some(\n        ModuleItem(\n            Type(\n                TypePath(\n                    Id {\n                        value: 4,\n                    },\n                ),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
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
                12,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 12;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 13,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: ImplicitParameter {\n        implicit_parameter_kind: Type {\n            ident_token: IdentifierToken {\n                ident: Identifier(\n                    Word(\n                        Id {\n                            value: 13,\n                        },\n                    ),\n                ),\n                token_idx: TokenIdx(\n                    12,\n                ),\n            },\n        },\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 5,\n        },\n    ),\n};\n\nCurrentSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 13,\n            },\n        ),\n    ),\n    access_start: TokenIdx(\n        13,\n    ),\n    access_end: None,\n    variant: ImplicitParameter {\n        implicit_parameter_variant: Type {\n            ident_token: IdentifierToken {\n                ident: Identifier(\n                    Word(\n                        Id {\n                            value: 13,\n                        },\n                    ),\n                ),\n                token_idx: TokenIdx(\n                    12,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 32,
                                },
                                end: Position {
                                    line: 2,
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
                14,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 14;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 10,\n            },\n        ),\n    ),\n);\n\ntoken_info = Entity(\n    Some(\n        ModuleItem(\n            Type(\n                TypePath(\n                    Id {\n                        value: 4,\n                    },\n                ),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 35,
                                },
                                end: Position {
                                    line: 2,
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
                16,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 16;\n\ntoken = Punctuation(\n    Bra(\n        Curl,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 41,
                                },
                                end: Position {
                                    line: 2,
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
                18,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 18;\n\ntoken = Punctuation(\n    Colon,\n);\n\ntoken_info = None;\n\n\n",
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
                20,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 20;\n\ntoken = Punctuation(\n    Comma,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 3,
                                    character: 11,
                                },
                                end: Position {
                                    line: 3,
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
                                    character: 9,
                                },
                                end: Position {
                                    line: 4,
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
                24,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 24;\n\ntoken = Punctuation(\n    Comma,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 12,
                                },
                                end: Position {
                                    line: 4,
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
                26,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 26;\n\ntoken = Attr(\n    Pub,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 0,
                                },
                                end: Position {
                                    line: 7,
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
                28,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 28;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 16,\n            },\n        ),\n    ),\n);\n\ntoken_info = Entity(\n    Some(\n        ModuleItem(\n            Type(\n                TypePath(\n                    Id {\n                        value: 6,\n                    },\n                ),\n            ),\n        ),\n    ),\n    Some(\n        ModuleItem {\n            module_item_kind: Type(\n                Inductive,\n            ),\n            connection: Connected,\n        },\n    ),\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 14,
                                },
                                end: Position {
                                    line: 7,
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
                30,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 30;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 12,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: ImplicitParameter {\n        implicit_parameter_kind: Type {\n            ident_token: IdentifierToken {\n                ident: Identifier(\n                    Word(\n                        Id {\n                            value: 12,\n                        },\n                    ),\n                ),\n                token_idx: TokenIdx(\n                    30,\n                ),\n            },\n        },\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 6,\n        },\n    ),\n};\n\nCurrentSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 12,\n            },\n        ),\n    ),\n    access_start: TokenIdx(\n        31,\n    ),\n    access_end: None,\n    variant: ImplicitParameter {\n        implicit_parameter_variant: Type {\n            ident_token: IdentifierToken {\n                ident: Identifier(\n                    Word(\n                        Id {\n                            value: 12,\n                        },\n                    ),\n                ),\n                token_idx: TokenIdx(\n                    30,\n                ),\n            },\n        },\n    },\n}\n",
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
                32,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 32;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 10,\n            },\n        ),\n    ),\n);\n\ntoken_info = Entity(\n    Some(\n        ModuleItem(\n            Type(\n                TypePath(\n                    Id {\n                        value: 4,\n                    },\n                ),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 25,
                                },
                                end: Position {
                                    line: 7,
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
                34,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 34;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 13,\n            },\n        ),\n    ),\n);\n\ntoken_info = CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: ImplicitParameter {\n        implicit_parameter_kind: Type {\n            ident_token: IdentifierToken {\n                ident: Identifier(\n                    Word(\n                        Id {\n                            value: 13,\n                        },\n                    ),\n                ),\n                token_idx: TokenIdx(\n                    34,\n                ),\n            },\n        },\n    },\n    expr_region: ExprRegion(\n        Id {\n            value: 6,\n        },\n    ),\n};\n\nCurrentSymbol {\n    ident: Identifier(\n        Word(\n            Id {\n                value: 13,\n            },\n        ),\n    ),\n    access_start: TokenIdx(\n        35,\n    ),\n    access_end: None,\n    variant: ImplicitParameter {\n        implicit_parameter_variant: Type {\n            ident_token: IdentifierToken {\n                ident: Identifier(\n                    Word(\n                        Id {\n                            value: 13,\n                        },\n                    ),\n                ),\n                token_idx: TokenIdx(\n                    34,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 31,
                                },
                                end: Position {
                                    line: 7,
                                    character: 32,
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
                36,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 36;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 10,\n            },\n        ),\n    ),\n);\n\ntoken_info = Entity(\n    Some(\n        ModuleItem(\n            Type(\n                TypePath(\n                    Id {\n                        value: 4,\n                    },\n                ),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 34,
                                },
                                end: Position {
                                    line: 7,
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
                38,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 38;\n\ntoken = Punctuation(\n    Vertical,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 8,
                                    character: 0,
                                },
                                end: Position {
                                    line: 8,
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
                40,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 40;\n\ntoken = Punctuation(\n    Bra(\n        Par,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 8,
                                    character: 6,
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
                42,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 42;\n\ntoken = Punctuation(\n    Ket(\n        Par,\n    ),\n);\n\ntoken_info = None;\n\n\n",
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
                                value: "\ntoken_idx = 44;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 18,\n            },\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 2,
                                },
                                end: Position {
                                    line: 9,
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
                46,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 46;\n\ntoken = Identifier(\n    Identifier(\n        Word(\n            Id {\n                value: 13,\n            },\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 8,
                                },
                                end: Position {
                                    line: 9,
                                    character: 9,
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