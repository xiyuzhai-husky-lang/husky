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
                                value: "Other\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = Token::Keyword(\n    Keyword::Mod,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                74,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 74;\n\ntoken_line_group_idx = 10\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Exclamation,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 16,
                                    character: 33,
                                },
                                end: Position {
                                    line: 16,
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
                148,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 148;\n\ntoken_line_group_idx = 18\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 28,
                                    character: 33,
                                },
                                end: Position {
                                    line: 28,
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
                222,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 222;\n\ntoken_line_group_idx = 28\n\ntoken = Token::Literal(\n    Literal::Integer(\n        Unspecified,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 45,
                                    character: 39,
                                },
                                end: Position {
                                    line: 45,
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
                296,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 296;\n\ntoken_line_group_idx = 37\n\ntoken = Token::Ident(\n    `ymin`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 3,\n    current_symbol_kind: CurrentSymbolKind::LetVariable {\n        pattern_symbol_idx: 3,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    modifier: Mut,\n    access_start: TokenIdx(\n        243,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                334,\n            ),\n        ),\n    ),\n    variant: CurrentSymbolVariant::LetVariable {\n        ident: `ymin`,\n        pattern_symbol_idx: 3,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 54,
                                    character: 12,
                                },
                                end: Position {
                                    line: 54,
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
                370,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 370;\n\ntoken_line_group_idx = 42\n\ntoken = Token::Ident(\n    `f32`,\n);\n\ntoken_info = TokenInfo::Entity(\n    Some(\n        EntityPath::ModuleItem(\n            ModuleItemPath::Type(\n                TypePath(`core::num::f32`, `Extern`),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 64,
                                    character: 28,
                                },
                                end: Position {
                                    line: 64,
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
                444,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 444;\n\ntoken_line_group_idx = 48\n\ntoken = Token::Ident(\n    `Vector2d`,\n);\n\ntoken_info = TokenInfo::Entity(\n    Some(\n        EntityPath::ModuleItem(\n            ModuleItemPath::Type(\n                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),\n            ),\n        ),\n    ),\n    None,\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 70,
                                    character: 4,
                                },
                                end: Position {
                                    line: 70,
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
                518,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 518;\n\ntoken_line_group_idx = 52\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 75,
                                    character: 31,
                                },
                                end: Position {
                                    line: 75,
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
                592,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 592;\n\ntoken_line_group_idx = 58\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 82,
                                    character: 47,
                                },
                                end: Position {
                                    line: 82,
                                    character: 48,
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
                666,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 666;\n\ntoken_line_group_idx = 67\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 91,
                                    character: 38,
                                },
                                end: Position {
                                    line: 91,
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
                740,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 740;\n\ntoken_line_group_idx = 77\n\ntoken = Token::Ident(\n    `r`,\n);\n\ntoken_info = TokenInfo::InheritedSymbol {\n    inherited_symbol_idx: 2,\n    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {\n        ident: `r`,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nInheritedSymbol {\n    parent_symbol_idx: Current(\n        2,\n    ),\n    modifier: Pure,\n    kind: InheritedSymbolKind::ExplicitParameter {\n        ident: `r`,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 103,
                                    character: 38,
                                },
                                end: Position {
                                    line: 103,
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
                814,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 814;\n\ntoken_line_group_idx = 87\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 114,
                                    character: 18,
                                },
                                end: Position {
                                    line: 114,
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
                888,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 888;\n\ntoken_line_group_idx = 95\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 122,
                                    character: 24,
                                },
                                end: Position {
                                    line: 122,
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
                962,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 962;\n\ntoken_line_group_idx = 106\n\ntoken = Token::Ident(\n    `dp`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 6,\n    current_symbol_kind: CurrentSymbolKind::LetVariable {\n        pattern_symbol_idx: 6,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    modifier: Pure,\n    access_start: TokenIdx(\n        917,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                1043,\n            ),\n        ),\n    ),\n    variant: CurrentSymbolVariant::LetVariable {\n        ident: `dp`,\n        pattern_symbol_idx: 6,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 133,
                                    character: 36,
                                },
                                end: Position {
                                    line: 133,
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
                1036,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1036;\n\ntoken_line_group_idx = 113\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 144,
                                    character: 21,
                                },
                                end: Position {
                                    line: 144,
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
                1110,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1110;\n\ntoken_line_group_idx = 128\n\ntoken = Token::Ident(\n    `ct`,\n);\n\ntoken_info = TokenInfo::InheritedSymbol {\n    inherited_symbol_idx: 0,\n    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {\n        ident: `ct`,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nInheritedSymbol {\n    parent_symbol_idx: Current(\n        0,\n    ),\n    modifier: Pure,\n    kind: InheritedSymbolKind::ExplicitParameter {\n        ident: `ct`,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 161,
                                    character: 25,
                                },
                                end: Position {
                                    line: 161,
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
                1184,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1184;\n\ntoken_line_group_idx = 134\n\ntoken = Token::Literal(\n    Literal::Float(\n        Unspecified,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 168,
                                    character: 57,
                                },
                                end: Position {
                                    line: 168,
                                    character: 60,
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
                1258,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1258;\n\ntoken_line_group_idx = 141\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::RaOrGt,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 175,
                                    character: 36,
                                },
                                end: Position {
                                    line: 175,
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
                1332,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1332;\n\ntoken_line_group_idx = 146\n\ntoken = Token::Literal(\n    Literal::Float(\n        Unspecified,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 182,
                                    character: 48,
                                },
                                end: Position {
                                    line: 182,
                                    character: 53,
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
                1406,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1406;\n\ntoken_line_group_idx = 154\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 191,
                                    character: 26,
                                },
                                end: Position {
                                    line: 191,
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