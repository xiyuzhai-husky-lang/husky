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
                33,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 33;\n\ntoken_line_group_idx = 5\n\ntoken = Token::Keyword(\n    Keyword::TypeEntity(\n        Struct,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 4,
                                },
                                end: Position {
                                    line: 6,
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
                66,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 66;\n\ntoken_line_group_idx = 9\n\ntoken = Token::Keyword(\n    Keyword::Impl,\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                99,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 99;\n\ntoken_line_group_idx = 14\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 23,
                                    character: 28,
                                },
                                end: Position {
                                    line: 23,
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
                132,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 132;\n\ntoken_line_group_idx = 18\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 27,
                                    character: 43,
                                },
                                end: Position {
                                    line: 27,
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
                165,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 165;\n\ntoken_line_group_idx = 21\n\ntoken = Token::Ident(\n    `point_dist`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 6,\n    current_symbol_kind: CurrentSynSymbolKind::LetVariable {\n        pattern_symbol_idx: 5,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: None,\n    access_start: TokenIdx(\n        166,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                181,\n            ),\n        ),\n    ),\n    variant: CurrentSynSymbolVariant::LetVariable {\n        ident: `point_dist`,\n        pattern_symbol_idx: 5,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 31,
                                    character: 16,
                                },
                                end: Position {
                                    line: 31,
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
                198,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 198;\n\ntoken_line_group_idx = 27\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Dot,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 38,
                                    character: 26,
                                },
                                end: Position {
                                    line: 38,
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
                231,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 231;\n\ntoken_line_group_idx = 29\n\ntoken = Token::Ident(\n    `strokes`,\n);\n\ntoken_info = TokenInfo::Field;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 41,
                                    character: 26,
                                },
                                end: Position {
                                    line: 41,
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
                264,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 264;\n\ntoken_line_group_idx = 34\n\ntoken = Token::Ident(\n    `strokes`,\n);\n\ntoken_info = TokenInfo::Field;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 47,
                                    character: 31,
                                },
                                end: Position {
                                    line: 47,
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
                297,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 297;\n\ntoken_line_group_idx = 38\n\ntoken = Token::Ident(\n    `start_point`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: CurrentSynSymbolKind::LetVariable {\n        pattern_symbol_idx: 0,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: None,\n    access_start: TokenIdx(\n        261,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                384,\n            ),\n        ),\n    ),\n    variant: CurrentSynSymbolVariant::LetVariable {\n        ident: `start_point`,\n        pattern_symbol_idx: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 51,
                                    character: 23,
                                },
                                end: Position {
                                    line: 51,
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
                330,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 330;\n\ntoken_line_group_idx = 41\n\ntoken = Token::Ident(\n    `min`,\n);\n\ntoken_info = TokenInfo::Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 54,
                                    character: 24,
                                },
                                end: Position {
                                    line: 54,
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
                363,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 363;\n\ntoken_line_group_idx = 44\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Dot,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 57,
                                    character: 33,
                                },
                                end: Position {
                                    line: 57,
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
                396,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 396;\n\ntoken_line_group_idx = 47\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 64,
                                    character: 67,
                                },
                                end: Position {
                                    line: 64,
                                    character: 68,
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
                429,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 429;\n\ntoken_line_group_idx = 49\n\ntoken = Token::Ident(\n    `last`,\n);\n\ntoken_info = TokenInfo::Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 69,
                                    character: 25,
                                },
                                end: Position {
                                    line: 69,
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
                462,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 462;\n\ntoken_line_group_idx = 52\n\ntoken = Token::Ident(\n    `end`,\n);\n\ntoken_info = TokenInfo::EntityNode(\n    EntitySynNodePath::AssociatedItem(\n        AssociatedItemSynNodePath::TypeItem(\n            TypeItemSynNodePath {\n                maybe_ambiguous_path: MaybeAmbiguousPath {\n                    path: TypeItemPath {\n                        impl_block: TypeImplBlockPath {\n                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,\n                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),\n                            disambiguator: 0,\n                        },\n                        ident: `end`,\n                        item_kind: MethodFn,\n                    },\n                    disambiguator: 0,\n                },\n            },\n        ),\n    ),\n    AssociatedItem {\n        associated_item_kind: TypeItem(\n            MethodFn,\n        ),\n    },\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 75,
                                    character: 7,
                                },
                                end: Position {
                                    line: 75,
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
                495,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 495;\n\ntoken_line_group_idx = 55\n\ntoken = Token::Ident(\n    `displacement`,\n);\n\ntoken_info = TokenInfo::Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 79,
                                    character: 28,
                                },
                                end: Position {
                                    line: 79,
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
                528,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 528;\n\ntoken_line_group_idx = 59\n\ntoken = Token::Ident(\n    `last`,\n);\n\ntoken_info = TokenInfo::Method;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 85,
                                    character: 21,
                                },
                                end: Position {
                                    line: 85,
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
                561,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 561;\n\ntoken_line_group_idx = 62\n\ntoken = Token::Ident(\n    `L`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: CurrentSynSymbolKind::LetVariable {\n        pattern_symbol_idx: 1,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: None,\n    access_start: TokenIdx(\n        562,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                661,\n            ),\n        ),\n    ),\n    variant: CurrentSynSymbolVariant::LetVariable {\n        ident: `L`,\n        pattern_symbol_idx: 1,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 90,
                                    character: 8,
                                },
                                end: Position {
                                    line: 90,
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
                594,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 594;\n\ntoken_line_group_idx = 66\n\ntoken = Token::Ident(\n    `start`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 2,\n    current_symbol_kind: CurrentSynSymbolKind::LetVariable {\n        pattern_symbol_idx: 2,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Mut,\n    access_start: TokenIdx(\n        573,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                661,\n            ),\n        ),\n    ),\n    variant: CurrentSynSymbolVariant::LetVariable {\n        ident: `start`,\n        pattern_symbol_idx: 2,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 94,
                                    character: 8,
                                },
                                end: Position {
                                    line: 94,
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
                627,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 627;\n\ntoken_line_group_idx = 71\n\ntoken = Token::Ident(\n    `start`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 2,\n    current_symbol_kind: CurrentSynSymbolKind::LetVariable {\n        pattern_symbol_idx: 2,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Mut,\n    access_start: TokenIdx(\n        573,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                661,\n            ),\n        ),\n    ),\n    variant: CurrentSynSymbolVariant::LetVariable {\n        ident: `start`,\n        pattern_symbol_idx: 2,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 99,
                                    character: 17,
                                },
                                end: Position {
                                    line: 99,
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
                660,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 660;\n\ntoken_line_group_idx = 75\n\ntoken = Token::Ident(\n    `concave_components`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: CurrentSynSymbolKind::LetVariable {\n        pattern_symbol_idx: 0,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Mut,\n    access_start: TokenIdx(\n        553,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                661,\n            ),\n        ),\n    ),\n    variant: CurrentSynSymbolVariant::LetVariable {\n        ident: `concave_components`,\n        pattern_symbol_idx: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 106,
                                    character: 11,
                                },
                                end: Position {
                                    line: 106,
                                    character: 29,
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