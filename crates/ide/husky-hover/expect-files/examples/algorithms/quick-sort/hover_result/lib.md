Ok(
    [
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
                                value: "Other\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = None;\n\n\n",
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
                15,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 14;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Box,\n        ),\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                1,\n            ),\n        ),\n        data: TokenInfoData::BoxColon,\n    },\n);\n\nbox colon\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 0,
                                    character: 37,
                                },
                                end: Position {
                                    line: 0,
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
                29,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 28;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Ident(\n    `arr`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                4,\n            ),\n        ),\n        data: TokenInfoData::InheritedSymbol {\n            inherited_symbol_idx: 2,\n            inherited_symbol_kind: SynInheritedSymbolKind::ParenateParameter {\n                ident: `arr`,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nSynInheritedSymbol {\n    parent_symbol_idx: Current(\n        2,\n    ),\n    modifier: Mut,\n    kind: SynInheritedSymbolKind::ParenateParameter {\n        ident: `arr`,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 2,
                                    character: 19,
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
                43,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 42;\n\ntoken_line_group_idx = 3\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::LaOrLt,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 17,
                                },
                                end: Position {
                                    line: 4,
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
                57,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 56;\n\ntoken_line_group_idx = 3\n\ntoken = TokenData::Ident(\n    `low`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::PatternExpr(\n            2,\n        ),\n        data: TokenInfoData::CurrentSymbol {\n            current_symbol_idx: 3,\n            current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {\n                pattern_symbol_idx: 2,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nSynCurrentSymbol {\n    modifier: None,\n    access_start: RegionalTokenIdx(\n        18,\n    ),\n    access_end: None,\n    variant: SynCurrentSymbolVariant::ParenateRegularParameter {\n        ident: `low`,\n        pattern_symbol_idx: 2,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 41,
                                },
                                end: Position {
                                    line: 4,
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
                71,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 70;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Keyword(\n    Keyword::Stmt(\n        Let,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 8,
                                },
                                end: Position {
                                    line: 6,
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
                85,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 84;\n\ntoken_line_group_idx = 6\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Comma,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 7,
                                    character: 26,
                                },
                                end: Position {
                                    line: 7,
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
                99,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 98;\n\ntoken_line_group_idx = 7\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Comma,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 8,
                                    character: 33,
                                },
                                end: Position {
                                    line: 8,
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
                113,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 112;\n\ntoken_line_group_idx = 8\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Box,\n        ),\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                1,\n            ),\n        ),\n        data: TokenInfoData::BoxColon,\n    },\n);\n\nbox colon\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 10,
                                    character: 30,
                                },
                                end: Position {
                                    line: 10,
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
                127,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 126;\n\ntoken_line_group_idx = 8\n\ntoken = TokenData::Ident(\n    `isize`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            4,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::num::isize`, `Extern`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::num::isize`, `Extern`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 10,
                                    character: 64,
                                },
                                end: Position {
                                    line: 10,
                                    character: 69,
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
                141,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 140;\n\ntoken_line_group_idx = 10\n\ntoken = TokenData::Literal(\n    LiteralData::Integer(\n        UnspecifiedRegular(\n            1,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 12,
                                    character: 32,
                                },
                                end: Position {
                                    line: 12,
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
                155,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 154;\n\ntoken_line_group_idx = 14\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Box,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 17,
                                    character: 17,
                                },
                                end: Position {
                                    line: 17,
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
                169,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 168;\n\ntoken_line_group_idx = 16\n\ntoken = TokenData::Ident(\n    `last_index`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                24,\n            ),\n        ),\n        data: TokenInfoData::CurrentSymbol {\n            current_symbol_idx: 3,\n            current_symbol_kind: SynCurrentSymbolKind::LetVariable {\n                pattern_symbol_idx: 3,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nSynCurrentSymbol {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        17,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                97,\n            ),\n        ),\n    ),\n    variant: SynCurrentSymbolVariant::LetVariable {\n        ident: `last_index`,\n        pattern_symbol_idx: 3,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 19,
                                    character: 8,
                                },
                                end: Position {
                                    line: 19,
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
                183,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 182;\n\ntoken_line_group_idx = 17\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::RaOrGt,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 20,
                                    character: 58,
                                },
                                end: Position {
                                    line: 20,
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
                197,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 196;\n\ntoken_line_group_idx = 20\n\ntoken = TokenData::Keyword(\n    Keyword::Stmt(\n        Break,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 23,
                                    character: 12,
                                },
                                end: Position {
                                    line: 23,
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
                211,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 210;\n\ntoken_line_group_idx = 22\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 25,
                                    character: 62,
                                },
                                end: Position {
                                    line: 25,
                                    character: 63,
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
                225,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 224;\n\ntoken_line_group_idx = 25\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Pound,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 29,
                                    character: 0,
                                },
                                end: Position {
                                    line: 29,
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
                239,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 238;\n\ntoken_line_group_idx = 27\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 31,
                                    character: 21,
                                },
                                end: Position {
                                    line: 31,
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
                253,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 252;\n\ntoken_line_group_idx = 27\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Comma,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 31,
                                    character: 44,
                                },
                                end: Position {
                                    line: 31,
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
                267,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 266;\n\ntoken_line_group_idx = 29\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 36,
                                    character: 29,
                                },
                                end: Position {
                                    line: 36,
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
                281,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 280;\n\ntoken_line_group_idx = 30\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Comma,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 37,
                                    character: 55,
                                },
                                end: Position {
                                    line: 37,
                                    character: 56,
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