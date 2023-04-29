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
                32,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 32;\n\ntoken_line_group_idx = 5\n\ntoken = Token::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                64,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 64;\n\ntoken_line_group_idx = 8\n\ntoken = Token::Ident(\n    `strokes`,\n);\n\ntoken_info = TokenInfo::Field;\n\n\n",
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
                96,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 96;\n\ntoken_line_group_idx = 13\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 21,
                                    character: 44,
                                },
                                end: Position {
                                    line: 21,
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
                128,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 128;\n\ntoken_line_group_idx = 18\n\ntoken = Token::Keyword(\n    Keyword::Stmt(\n        Let,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                160,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 160;\n\ntoken_line_group_idx = 20\n\ntoken = Token::Ident(\n    `strokes`,\n);\n\ntoken_info = TokenInfo::Field;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 30,
                                    character: 29,
                                },
                                end: Position {
                                    line: 30,
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
                192,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 192;\n\ntoken_line_group_idx = 26\n\ntoken = Token::Ident(\n    `angle_change`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: CurrentSymbolKind::LetVariable {\n        pattern_symbol_idx: 0,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    modifier: Mut,\n    access_start: TokenIdx(\n        193,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                256,\n            ),\n        ),\n    ),\n    variant: CurrentSymbolVariant::LetVariable {\n        ident: `angle_change`,\n        pattern_symbol_idx: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 37,
                                    character: 16,
                                },
                                end: Position {
                                    line: 37,
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
                224,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 224;\n\ntoken_line_group_idx = 28\n\ntoken = Token::Ident(\n    `strokes`,\n);\n\ntoken_info = TokenInfo::Field;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 40,
                                    character: 42,
                                },
                                end: Position {
                                    line: 40,
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
                256,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "This is a paradigm\ntoken_idx = 256;\n\ntoken_line_group_idx = 33\n\ntoken = Token::Keyword(\n    Keyword::Form(\n        Val,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 46,
                                    character: 4,
                                },
                                end: Position {
                                    line: 46,
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
                288,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 288;\n\ntoken_line_group_idx = 37\n\ntoken = Token::Keyword(\n    Keyword::Modifier(\n        Mut,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 50,
                                    character: 12,
                                },
                                end: Position {
                                    line: 50,
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
                320,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 320;\n\ntoken_line_group_idx = 40\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Dot,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 53,
                                    character: 28,
                                },
                                end: Position {
                                    line: 53,
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
                352,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 352;\n\ntoken_line_group_idx = 43\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 56,
                                    character: 27,
                                },
                                end: Position {
                                    line: 56,
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
                384,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 384;\n\ntoken_line_group_idx = 45\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 61,
                                    character: 8,
                                },
                                end: Position {
                                    line: 61,
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
                416,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 416;\n\ntoken_line_group_idx = 49\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 68,
                                    character: 31,
                                },
                                end: Position {
                                    line: 68,
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
                448,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 448;\n\ntoken_line_group_idx = 51\n\ntoken = Token::Ident(\n    `strokes`,\n);\n\ntoken_info = TokenInfo::Field;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 73,
                                    character: 13,
                                },
                                end: Position {
                                    line: 73,
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
                480,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 480;\n\ntoken_line_group_idx = 54\n\ntoken = Token::Ident(\n    `displacement`,\n);\n\ntoken_info = TokenInfo::Entity(\n    Some(\n        EntityPath::AssociatedItem(\n            AssociatedItemPath::TypeItem(\n                TypeItemPath {\n                    parent_ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),\n                    ident: `displacement`,\n                    item_kind: MethodFn,\n                },\n            ),\n        ),\n    ),\n    Some(\n        AssociatedItem {\n            associated_item_kind: TypeItem(\n                MethodFn,\n            ),\n        },\n    ),\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 78,
                                    character: 7,
                                },
                                end: Position {
                                    line: 78,
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
                512,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 512;\n\ntoken_line_group_idx = 57\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 82,
                                    character: 43,
                                },
                                end: Position {
                                    line: 82,
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
                544,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 544;\n\ntoken_line_group_idx = 60\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 88,
                                    character: 93,
                                },
                                end: Position {
                                    line: 88,
                                    character: 94,
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
                576,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 576;\n\ntoken_line_group_idx = 65\n\ntoken = Token::Ident(\n    `start`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 2,\n    current_symbol_kind: CurrentSymbolKind::LetVariable {\n        pattern_symbol_idx: 2,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    modifier: Mut,\n    access_start: TokenIdx(\n        568,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                656,\n            ),\n        ),\n    ),\n    variant: CurrentSymbolVariant::LetVariable {\n        ident: `start`,\n        pattern_symbol_idx: 2,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 93,
                                    character: 10,
                                },
                                end: Position {
                                    line: 93,
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
                608,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 608;\n\ntoken_line_group_idx = 69\n\ntoken = Token::WordOpr(\n    WordOpr::And,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 97,
                                    character: 29,
                                },
                                end: Position {
                                    line: 97,
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
                640,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 640;\n\ntoken_line_group_idx = 72\n\ntoken = Token::Ident(\n    `start`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 2,\n    current_symbol_kind: CurrentSymbolKind::LetVariable {\n        pattern_symbol_idx: 2,\n    },\n    expr_region: ExprRegionLeash(_),\n};\n\nCurrentSymbol {\n    modifier: Mut,\n    access_start: TokenIdx(\n        568,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                656,\n            ),\n        ),\n    ),\n    variant: CurrentSymbolVariant::LetVariable {\n        ident: `start`,\n        pattern_symbol_idx: 2,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 102,
                                    character: 57,
                                },
                                end: Position {
                                    line: 102,
                                    character: 62,
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