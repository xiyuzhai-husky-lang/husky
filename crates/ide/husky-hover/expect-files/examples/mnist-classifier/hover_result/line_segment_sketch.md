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
                                value: "\ntoken_idx = 74;\n\ntoken_line_group_idx = 10\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 16,
                                    character: 36,
                                },
                                end: Position {
                                    line: 16,
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
                148,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 148;\n\ntoken_line_group_idx = 17\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            Closed(\n                Add,\n            ),\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 26,
                                    character: 58,
                                },
                                end: Position {
                                    line: 26,
                                    character: 59,
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
                                value: "\ntoken_idx = 222;\n\ntoken_line_group_idx = 27\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 44,
                                    character: 34,
                                },
                                end: Position {
                                    line: 44,
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
                296,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 296;\n\ntoken_line_group_idx = 36\n\ntoken = Token::Ident(\n    `xmax`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 2,\n    current_symbol_kind: CurrentSynSymbolKind::LetVariable {\n        pattern_symbol_idx: 2,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Mut,\n    access_start: TokenIdx(\n        244,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                342,\n            ),\n        ),\n    ),\n    variant: CurrentSynSymbolVariant::LetVariable {\n        ident: `xmax`,\n        pattern_symbol_idx: 2,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 53,
                                    character: 19,
                                },
                                end: Position {
                                    line: 53,
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
                370,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 370;\n\ntoken_line_group_idx = 42\n\ntoken = Token::Ident(\n    `go_right`,\n);\n\ntoken_info = TokenInfo::EntityNode(\n    EntitySynNodePath::ModuleItem(\n        ModuleItemSynNodePath::Fugitive(\n            FugitiveSynNodePath {\n                maybe_ambiguous_path: MaybeAmbiguousPath {\n                    path: FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `Fn`),\n                    disambiguator: 0,\n                },\n            },\n        ),\n    ),\n    ModuleItem {\n        module_item_kind: Fugitive(\n            Fn,\n        ),\n        connection: Connected,\n    },\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 64,
                                    character: 3,
                                },
                                end: Position {
                                    line: 64,
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
                444,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 444;\n\ntoken_line_group_idx = 47\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Minus,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 69,
                                    character: 13,
                                },
                                end: Position {
                                    line: 69,
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
                518,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 518;\n\ntoken_line_group_idx = 52\n\ntoken = Token::Ident(\n    `L`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: CurrentSynSymbolKind::LetVariable {\n        pattern_symbol_idx: 0,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: None,\n    access_start: TokenIdx(\n        482,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                563,\n            ),\n        ),\n    ),\n    variant: CurrentSynSymbolVariant::LetVariable {\n        ident: `L`,\n        pattern_symbol_idx: 0,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 75,
                                    character: 20,
                                },
                                end: Position {
                                    line: 75,
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
                592,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 592;\n\ntoken_line_group_idx = 58\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Dot,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 82,
                                    character: 19,
                                },
                                end: Position {
                                    line: 82,
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
                666,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 666;\n\ntoken_line_group_idx = 67\n\ntoken = Token::Keyword(\n    Keyword::Modifier(\n        Mut,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 91,
                                    character: 8,
                                },
                                end: Position {
                                    line: 91,
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
                740,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 740;\n\ntoken_line_group_idx = 76\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 102,
                                    character: 41,
                                },
                                end: Position {
                                    line: 102,
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
                814,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 814;\n\ntoken_line_group_idx = 86\n\ntoken = Token::Ident(\n    `f32`,\n);\n\ntoken_info = TokenInfo::Entity(\n    EntityPath::ModuleItem(\n        ModuleItemPath::Type(\n            TypePath(`core::num::f32`, `Extern`),\n        ),\n    ),\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 113,
                                    character: 59,
                                },
                                end: Position {
                                    line: 113,
                                    character: 62,
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
                                value: "\ntoken_idx = 888;\n\ntoken_line_group_idx = 94\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Dot,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 121,
                                    character: 20,
                                },
                                end: Position {
                                    line: 121,
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
                962,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 962;\n\ntoken_line_group_idx = 105\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::RaOrGt,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 132,
                                    character: 19,
                                },
                                end: Position {
                                    line: 132,
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
                1036,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1036;\n\ntoken_line_group_idx = 113\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Dot,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 143,
                                    character: 30,
                                },
                                end: Position {
                                    line: 143,
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
                1110,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 1110;\n\ntoken_line_group_idx = 127\n\ntoken = Token::Keyword(\n    Keyword::Stmt(\n        While,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 160,
                                    character: 4,
                                },
                                end: Position {
                                    line: 160,
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
                1184,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1184;\n\ntoken_line_group_idx = 134\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::LaOrLt,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 167,
                                    character: 54,
                                },
                                end: Position {
                                    line: 167,
                                    character: 55,
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
                                value: "\ntoken_idx = 1258;\n\ntoken_line_group_idx = 140\n\ntoken = Token::Ident(\n    `ct`,\n);\n\ntoken_info = TokenInfo::InheritedSymbol {\n    inherited_symbol_idx: 0,\n    inherited_symbol_kind: InheritedSynSymbolKind::ExplicitParameter {\n        ident: `ct`,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nInheritedSynSymbol {\n    parent_symbol_idx: Current(\n        0,\n    ),\n    modifier: None,\n    kind: InheritedSynSymbolKind::ExplicitParameter {\n        ident: `ct`,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 174,
                                    character: 48,
                                },
                                end: Position {
                                    line: 174,
                                    character: 50,
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
                                    line: 181,
                                    character: 46,
                                },
                                end: Position {
                                    line: 181,
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
                1406,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1406;\n\ntoken_line_group_idx = 153\n\ntoken = Token::Ident(\n    `end`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 2,\n    current_symbol_kind: CurrentSynSymbolKind::LetVariable {\n        pattern_symbol_idx: 2,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Mut,\n    access_start: TokenIdx(\n        1097,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                1497,\n            ),\n        ),\n    ),\n    variant: CurrentSynSymbolVariant::LetVariable {\n        ident: `end`,\n        pattern_symbol_idx: 2,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 190,
                                    character: 8,
                                },
                                end: Position {
                                    line: 190,
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
                1480,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1480;\n\ntoken_line_group_idx = 159\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            Closed(\n                Sub,\n            ),\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 198,
                                    character: 43,
                                },
                                end: Position {
                                    line: 198,
                                    character: 44,
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