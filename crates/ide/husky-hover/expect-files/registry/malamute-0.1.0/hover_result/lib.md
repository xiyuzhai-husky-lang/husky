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
                                value: "Other\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = Token::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                4,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 4;\n\ntoken_line_group_idx = 0\n\ntoken = Token::Ident(\n    `Label`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {\n        implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {\n            ident_token: IdentToken {\n                ident: `Label`,\n                token_idx: TokenIdx(\n                    4,\n                ),\n            },\n        },\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Const,\n    access_start: TokenIdx(\n        5,\n    ),\n    access_end: None,\n    variant: CurrentSynSymbolVariant::ImplicitParameter {\n        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {\n            ident_token: IdentToken {\n                ident: `Label`,\n                token_idx: TokenIdx(\n                    4,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 0,
                                    character: 18,
                                },
                                end: Position {
                                    line: 0,
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
                8,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 8;\n\ntoken_line_group_idx = 0\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 0,
                                    character: 36,
                                },
                                end: Position {
                                    line: 0,
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
                12,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 12;\n\ntoken_line_group_idx = 1\n\ntoken = Token::Ident(\n    `Yes`,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 1,
                                    character: 2,
                                },
                                end: Position {
                                    line: 1,
                                    character: 5,
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
                                value: "Other\ntoken_idx = 16;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Keyword(\n    Keyword::TypeEntity(\n        Enum,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 4,
                                },
                                end: Position {
                                    line: 4,
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
                20,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 20;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Comma,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 29,
                                },
                                end: Position {
                                    line: 4,
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
                24,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 24;\n\ntoken_line_group_idx = 3\n\ntoken = Token::Ident(\n    `Label`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {\n        implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {\n            ident_token: IdentToken {\n                ident: `Label`,\n                token_idx: TokenIdx(\n                    19,\n                ),\n            },\n        },\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Const,\n    access_start: TokenIdx(\n        20,\n    ),\n    access_end: None,\n    variant: CurrentSynSymbolVariant::ImplicitParameter {\n        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {\n            ident_token: IdentToken {\n                ident: `Label`,\n                token_idx: TokenIdx(\n                    19,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 44,
                                },
                                end: Position {
                                    line: 4,
                                    character: 49,
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
                                value: "\ntoken_idx = 28;\n\ntoken_line_group_idx = 5\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Vertical,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 0,
                                },
                                end: Position {
                                    line: 6,
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
                32,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 32;\n\ntoken_line_group_idx = 7\n\ntoken = Token::Keyword(\n    Keyword::Impl,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 0,
                                },
                                end: Position {
                                    line: 9,
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
                36,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 36;\n\ntoken_line_group_idx = 7\n\ntoken = Token::Keyword(\n    Keyword::Const,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 12,
                                },
                                end: Position {
                                    line: 9,
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
                40,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 40;\n\ntoken_line_group_idx = 7\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::RaOrGt,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 30,
                                },
                                end: Position {
                                    line: 9,
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
                44,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 44;\n\ntoken_line_group_idx = 7\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 41,
                                },
                                end: Position {
                                    line: 9,
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
                48,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 48;\n\ntoken_line_group_idx = 7\n\ntoken = Token::Ident(\n    `label`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {\n        implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {\n            ident_token: IdentToken {\n                ident: `label`,\n                token_idx: TokenIdx(\n                    37,\n                ),\n            },\n        },\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Const,\n    access_start: TokenIdx(\n        40,\n    ),\n    access_end: None,\n    variant: CurrentSynSymbolVariant::ImplicitParameter {\n        implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {\n            ident_token: IdentToken {\n                ident: `label`,\n                token_idx: TokenIdx(\n                    37,\n                ),\n            },\n            ty_expr_idx: 0,\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 71,
                                },
                                end: Position {
                                    line: 9,
                                    character: 76,
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
                52,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 52;\n\ntoken_line_group_idx = 7\n\ntoken = Token::Ident(\n    `label`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {\n        implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {\n            ident_token: IdentToken {\n                ident: `label`,\n                token_idx: TokenIdx(\n                    37,\n                ),\n            },\n        },\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Const,\n    access_start: TokenIdx(\n        40,\n    ),\n    access_end: None,\n    variant: CurrentSynSymbolVariant::ImplicitParameter {\n        implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {\n            ident_token: IdentToken {\n                ident: `label`,\n                token_idx: TokenIdx(\n                    37,\n                ),\n            },\n            ty_expr_idx: 0,\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 96,
                                },
                                end: Position {
                                    line: 9,
                                    character: 101,
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
                56,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 56;\n\ntoken_line_group_idx = 8\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 10,
                                    character: 16,
                                },
                                end: Position {
                                    line: 10,
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
                60,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "This is a paradigm\ntoken_idx = 60;\n\ntoken_line_group_idx = 9\n\ntoken = Token::Keyword(\n    Keyword::Fugitive(\n        Gn,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                64,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 64;\n\ntoken_line_group_idx = 9\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Comma,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 12,
                                    character: 24,
                                },
                                end: Position {
                                    line: 12,
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
                68,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 68;\n\ntoken_line_group_idx = 9\n\ntoken = Token::Ident(\n    `Label`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {\n        implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {\n            ident_token: IdentToken {\n                ident: `Label`,\n                token_idx: TokenIdx(\n                    63,\n                ),\n            },\n        },\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Const,\n    access_start: TokenIdx(\n        64,\n    ),\n    access_end: None,\n    variant: CurrentSynSymbolVariant::ImplicitParameter {\n        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {\n            ident_token: IdentToken {\n                ident: `Label`,\n                token_idx: TokenIdx(\n                    63,\n                ),\n            },\n        },\n    },\n}\n",
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
                72,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 72;\n\ntoken_line_group_idx = 9\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Box,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 13,
                                    character: 7,
                                },
                                end: Position {
                                    line: 13,
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
                76,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 76;\n\ntoken_line_group_idx = 9\n\ntoken = Token::Ident(\n    `f32`,\n);\n\ntoken_info = TokenInfo::Entity(\n    EntityPath::ModuleItem(\n        ModuleItemPath::Type(\n            TypePath(`core::num::f32`, `Extern`),\n        ),\n    ),\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 13,
                                    character: 13,
                                },
                                end: Position {
                                    line: 13,
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
                80,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 80;\n\ntoken_line_group_idx = 9\n\ntoken = Token::Ident(\n    `i32`,\n);\n\ntoken_info = TokenInfo::Entity(\n    EntityPath::ModuleItem(\n        ModuleItemPath::Type(\n            TypePath(`core::num::i32`, `Extern`),\n        ),\n    ),\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 14,
                                    character: 10,
                                },
                                end: Position {
                                    line: 14,
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
                84,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 84;\n\ntoken_line_group_idx = 9\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 15,
                                    character: 0,
                                },
                                end: Position {
                                    line: 15,
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
                88,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 88;\n\ntoken_line_group_idx = 9\n\ntoken = Token::Ident(\n    `label`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {\n        implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {\n            ident_token: IdentToken {\n                ident: `label`,\n                token_idx: TokenIdx(\n                    66,\n                ),\n            },\n        },\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Const,\n    access_start: TokenIdx(\n        69,\n    ),\n    access_end: None,\n    variant: CurrentSynSymbolVariant::ImplicitParameter {\n        implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {\n            ident_token: IdentToken {\n                ident: `label`,\n                token_idx: TokenIdx(\n                    66,\n                ),\n            },\n            ty_expr_idx: 0,\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 15,
                                    character: 26,
                                },
                                end: Position {
                                    line: 15,
                                    character: 31,
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