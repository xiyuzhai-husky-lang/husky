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
                                value: "Other\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = Token::Keyword(\n    Keyword::Use,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                76,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 76;\n\ntoken_line_group_idx = 10\n\ntoken = Token::Ident(\n    `x`,\n);\n\ntoken_info = TokenInfo::Field;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 16,
                                    character: 35,
                                },
                                end: Position {
                                    line: 16,
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
                152,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 152;\n\ntoken_line_group_idx = 19\n\ntoken = Token::Ident(\n    `max`,\n);\n\ntoken_info = TokenInfo::Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 25,
                                    character: 24,
                                },
                                end: Position {
                                    line: 25,
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
                228,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 228;\n\ntoken_line_group_idx = 26\n\ntoken = Token::Ident(\n    `points`,\n);\n\ntoken_info = TokenInfo::Field;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 37,
                                    character: 25,
                                },
                                end: Position {
                                    line: 37,
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
                304,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 304;\n\ntoken_line_group_idx = 31\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            Closed(\n                Sub,\n            ),\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 42,
                                    character: 28,
                                },
                                end: Position {
                                    line: 42,
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
                380,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 380;\n\ntoken_line_group_idx = 37\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 49,
                                    character: 26,
                                },
                                end: Position {
                                    line: 49,
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
                456,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 456;\n\ntoken_line_group_idx = 49\n\ntoken = Token::Ident(\n    `j`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: CurrentSymbolKind::Parameter {\n        pattern_symbol_idx: 1,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        457,\n    ),\n    access_end: None,\n    variant: CurrentSymbolVariant::ExplicitParameter {\n        ident: `j`,\n        pattern_symbol_idx: 1,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 66,
                                    character: 36,
                                },
                                end: Position {
                                    line: 66,
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
                532,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 532;\n\ntoken_line_group_idx = 59\n\ntoken = Token::Literal(\n    Literal::Integer(\n        Unspecified,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 81,
                                    character: 10,
                                },
                                end: Position {
                                    line: 81,
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
                608,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 608;\n\ntoken_line_group_idx = 73\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 95,
                                    character: 24,
                                },
                                end: Position {
                                    line: 95,
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
                684,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "This is a paradigm\ntoken_idx = 684;\n\ntoken_line_group_idx = 84\n\ntoken = Token::Keyword(\n    Keyword::Form(\n        Fn,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 108,
                                    character: 0,
                                },
                                end: Position {
                                    line: 108,
                                    character: 2,
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
                760,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 760;\n\ntoken_line_group_idx = 94\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::RaOrGt,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 123,
                                    character: 9,
                                },
                                end: Position {
                                    line: 123,
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
                836,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 836;\n\ntoken_line_group_idx = 109\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 138,
                                    character: 12,
                                },
                                end: Position {
                                    line: 138,
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
                912,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 912;\n\ntoken_line_group_idx = 123\n\ntoken = Token::Ident(\n    `StreakCache`,\n);\n\ntoken_info = TokenInfo::Entity(\n    Some(\n        EntityPath::ModuleItem(\n            ModuleItemPath::Type(\n                TypePath(`mnist_classifier::raw_contour::StreakCache`, `Struct`),\n            ),\n        ),\n    ),\n    Some(\n        ModuleItem {\n            module_item_kind: Type(\n                Struct,\n            ),\n            connection: Connected,\n        },\n    ),\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 154,
                                    character: 7,
                                },
                                end: Position {
                                    line: 154,
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
                988,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "This is a paradigm\ntoken_idx = 988;\n\ntoken_line_group_idx = 129\n\ntoken = Token::Keyword(\n    Keyword::Form(\n        Fn,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 168,
                                    character: 0,
                                },
                                end: Position {
                                    line: 168,
                                    character: 2,
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
                1064,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1064;\n\ntoken_line_group_idx = 137\n\ntoken = Token::Ident(\n    `r_dr`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 4,\n    current_symbol_kind: CurrentSymbolKind::LetVariable {\n        pattern_symbol_idx: 3,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        1037,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                1083,\n            ),\n        ),\n    ),\n    variant: CurrentSymbolVariant::LetVariable {\n        ident: `r_dr`,\n        pattern_symbol_idx: 3,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 176,
                                    character: 39,
                                },
                                end: Position {
                                    line: 176,
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
                1140,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1140;\n\ntoken_line_group_idx = 144\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Dot,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 184,
                                    character: 34,
                                },
                                end: Position {
                                    line: 184,
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
                1216,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1216;\n\ntoken_line_group_idx = 155\n\ntoken = Token::Ident(\n    `dir0`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 16,\n    current_symbol_kind: CurrentSymbolKind::LetVariable {\n        pattern_symbol_idx: 14,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        1167,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                1537,\n            ),\n        ),\n    ),\n    variant: CurrentSymbolVariant::LetVariable {\n        ident: `dir0`,\n        pattern_symbol_idx: 14,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 200,
                                    character: 67,
                                },
                                end: Position {
                                    line: 200,
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
                1292,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1292;\n\ntoken_line_group_idx = 161\n\ntoken = Token::Ident(\n    `get_concave_middle_point`,\n);\n\ntoken_info = TokenInfo::Entity(\n    Some(\n        EntityPath::ModuleItem(\n            ModuleItemPath::Form(\n                FormPath(`mnist_classifier::raw_contour::get_concave_middle_point`, `Fn`),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 217,
                                    character: 42,
                                },
                                end: Position {
                                    line: 217,
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
                1368,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1368;\n\ntoken_line_group_idx = 169\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 230,
                                    character: 48,
                                },
                                end: Position {
                                    line: 230,
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
                1444,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1444;\n\ntoken_line_group_idx = 184\n\ntoken = Token::Ident(\n    `mask`,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 249,
                                    character: 35,
                                },
                                end: Position {
                                    line: 249,
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
                1520,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1520;\n\ntoken_line_group_idx = 195\n\ntoken = Token::Ident(\n    `contour`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 8,\n    current_symbol_kind: CurrentSymbolKind::LetVariable {\n        pattern_symbol_idx: 6,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    access_start: TokenIdx(\n        1099,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                1537,\n            ),\n        ),\n    ),\n    variant: CurrentSymbolVariant::LetVariable {\n        ident: `contour`,\n        pattern_symbol_idx: 6,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 262,
                                    character: 16,
                                },
                                end: Position {
                                    line: 262,
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