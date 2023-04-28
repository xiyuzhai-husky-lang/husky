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
                77,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 77;\n\ntoken_line_group_idx = 11\n\ntoken = Token::Ident(\n    `bounding_box`,\n);\n\ntoken_info = TokenInfo::Entity(\n    Some(\n        EntityPath::AssociatedItem(\n            AssociatedItemPath::TypeItem(\n                TypeItemPath {\n                    parent_ty: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),\n                    ident: `bounding_box`,\n                    item_kind: MemoizedField,\n                },\n            ),\n        ),\n    ),\n    Some(\n        AssociatedItem {\n            associated_item_kind: TypeItem(\n                MemoizedField,\n            ),\n        },\n    ),\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 18,
                                    character: 8,
                                },
                                end: Position {
                                    line: 18,
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
                154,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 154;\n\ntoken_line_group_idx = 20\n\ntoken = Token::Ident(\n    `point`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 6,\n    current_symbol_kind: CurrentSymbolKind::LetVariable {\n        pattern_symbol_idx: 5,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    modifier: Pure,\n    access_start: TokenIdx(\n        131,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                178,\n            ),\n        ),\n    ),\n    variant: CurrentSymbolVariant::LetVariable {\n        ident: `point`,\n        pattern_symbol_idx: 5,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 27,
                                    character: 28,
                                },
                                end: Position {
                                    line: 27,
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
                231,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 231;\n\ntoken_line_group_idx = 28\n\ntoken = Token::Ident(\n    `i`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: CurrentSymbolKind::FrameVariable(\n        2,\n    ),\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    modifier: Pure,\n    access_start: TokenIdx(\n        241,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                290,\n            ),\n        ),\n    ),\n    variant: CurrentSymbolVariant::FrameVariable {\n        ident: `i`,\n        expr_idx: 2,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 40,
                                    character: 16,
                                },
                                end: Position {
                                    line: 40,
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
                308,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 308;\n\ntoken_line_group_idx = 33\n\ntoken = Token::Ident(\n    `b`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 5,\n    current_symbol_kind: CurrentSymbolKind::LetVariable {\n        pattern_symbol_idx: 4,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    modifier: Pure,\n    access_start: TokenIdx(\n        309,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                347,\n            ),\n        ),\n    ),\n    variant: CurrentSymbolVariant::LetVariable {\n        ident: `b`,\n        pattern_symbol_idx: 4,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 45,
                                    character: 12,
                                },
                                end: Position {
                                    line: 45,
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
                385,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 385;\n\ntoken_line_group_idx = 39\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 52,
                                    character: 19,
                                },
                                end: Position {
                                    line: 52,
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
                462,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 462;\n\ntoken_line_group_idx = 50\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Comma,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 67,
                                    character: 34,
                                },
                                end: Position {
                                    line: 67,
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
                539,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 539;\n\ntoken_line_group_idx = 60\n\ntoken = Token::Literal(\n    Literal::Integer(\n        Unspecified,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 82,
                                    character: 10,
                                },
                                end: Position {
                                    line: 82,
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
                616,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 616;\n\ntoken_line_group_idx = 74\n\ntoken = Token::Ident(\n    `Up`,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 96,
                                    character: 26,
                                },
                                end: Position {
                                    line: 96,
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
                693,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 693;\n\ntoken_line_group_idx = 85\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 109,
                                    character: 24,
                                },
                                end: Position {
                                    line: 109,
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
                770,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 770;\n\ntoken_line_group_idx = 96\n\ntoken = Token::Ident(\n    `pixel_pair_below`,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 125,
                                    character: 14,
                                },
                                end: Position {
                                    line: 125,
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
                847,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 847;\n\ntoken_line_group_idx = 110\n\ntoken = Token::Ident(\n    `Up`,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 139,
                                    character: 26,
                                },
                                end: Position {
                                    line: 139,
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
                924,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 924;\n\ntoken_line_group_idx = 124\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Comma,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 156,
                                    character: 14,
                                },
                                end: Position {
                                    line: 156,
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
                1001,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1001;\n\ntoken_line_group_idx = 130\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ambersand,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 169,
                                    character: 29,
                                },
                                end: Position {
                                    line: 169,
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
                1078,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1078;\n\ntoken_line_group_idx = 138\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ambersand,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 177,
                                    character: 55,
                                },
                                end: Position {
                                    line: 177,
                                    character: 56,
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
                1155,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1155;\n\ntoken_line_group_idx = 146\n\ntoken = Token::Ident(\n    `inward_direction`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 13,\n    current_symbol_kind: CurrentSymbolKind::LetVariable {\n        pattern_symbol_idx: 11,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    modifier: Mut,\n    access_start: TokenIdx(\n        1156,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                1545,\n            ),\n        ),\n    ),\n    variant: CurrentSymbolVariant::LetVariable {\n        ident: `inward_direction`,\n        pattern_symbol_idx: 11,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 187,
                                    character: 20,
                                },
                                end: Position {
                                    line: 187,
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
                1232,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1232;\n\ntoken_line_group_idx = 157\n\ntoken = Token::Ident(\n    `row_above`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 11,\n    current_symbol_kind: CurrentSymbolKind::LetVariable {\n        pattern_symbol_idx: 9,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    modifier: Mut,\n    access_start: TokenIdx(\n        1134,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                1545,\n            ),\n        ),\n    ),\n    variant: CurrentSymbolVariant::LetVariable {\n        ident: `row_above`,\n        pattern_symbol_idx: 9,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 203,
                                    character: 20,
                                },
                                end: Position {
                                    line: 203,
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
                1309,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1309;\n\ntoken_line_group_idx = 163\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 219,
                                    character: 44,
                                },
                                end: Position {
                                    line: 219,
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
                1386,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1386;\n\ntoken_line_group_idx = 171\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 232,
                                    character: 65,
                                },
                                end: Position {
                                    line: 232,
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
                1463,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1463;\n\ntoken_line_group_idx = 186\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::RaOrGt,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 251,
                                    character: 35,
                                },
                                end: Position {
                                    line: 251,
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
                1540,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1540;\n\ntoken_line_group_idx = 197\n\ntoken = Token::Ident(\n    `cc`,\n);\n\ntoken_info = TokenInfo::InheritedSymbol {\n    inherited_symbol_idx: 0,\n    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {\n        ident: `cc`,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nInheritedSymbol {\n    parent_symbol_idx: Current(\n        0,\n    ),\n    modifier: Pure,\n    kind: InheritedSymbolKind::ExplicitParameter {\n        ident: `cc`,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 264,
                                    character: 35,
                                },
                                end: Position {
                                    line: 264,
                                    character: 37,
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