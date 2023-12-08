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
                            value: "Other\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Keyword(\n    Keyword::Use,\n);\n\ntoken_info = None;\n\n\n",
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
                            value: "\ntoken_idx = 75;\n\ntoken_line_group_idx = 10\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                4,\n            ),\n        ),\n        data: TokenInfoData::CallPar,\n    },\n);\n\ncall par\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 16,
                                character: 40,
                            },
                            end: Position {
                                line: 16,
                                character: 41,
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
            151,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 150;\n\ntoken_line_group_idx = 20\n\ntoken = TokenData::Ident(\n    `xmax`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                28,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 3,\n            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {\n                pattern_symbol_idx: 3,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        20,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                116,\n            ),\n        ),\n    ),\n    data: CurrentSynSymbolData::LetVariable {\n        ident: `xmax`,\n        pattern_symbol_idx: 3,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 27,
                                character: 19,
                            },
                            end: Position {
                                line: 27,
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
            226,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 225;\n\ntoken_line_group_idx = 27\n\ntoken = TokenData::Ident(\n    `contour_len`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::PatternExpr(\n            1,\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 1,\n            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {\n                pattern_symbol_idx: 1,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        4,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                125,\n            ),\n        ),\n    ),\n    data: CurrentSynSymbolData::LetVariable {\n        ident: `contour_len`,\n        pattern_symbol_idx: 1,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 39,
                                character: 16,
                            },
                            end: Position {
                                line: 39,
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
            301,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 300;\n\ntoken_line_group_idx = 32\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Dot,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 44,
                                character: 39,
                            },
                            end: Position {
                                line: 44,
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
            376,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 375;\n\ntoken_line_group_idx = 38\n\ntoken = TokenData::Keyword(\n    Keyword::Pronoun(\n        SelfValue,\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                4,\n            ),\n        ),\n        data: TokenInfoData::SelfValue,\n    },\n);\n\nself value\n",
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
            451,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 450;\n\ntoken_line_group_idx = 49\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 65,
                                character: 28,
                            },
                            end: Position {
                                line: 65,
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
            526,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 525;\n\ntoken_line_group_idx = 55\n\ntoken = TokenData::Ident(\n    `row_below`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                6,\n            ),\n        ),\n        data: TokenInfoData::InheritedSynSymbol {\n            inherited_syn_symbol_idx: 2,\n            inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {\n                ident: `row_below`,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nInheritedSynSymbol {\n    parent_symbol_idx: Current(\n        2,\n    ),\n    modifier: Pure,\n    kind: InheritedSynSymbolKind::ParenateParameter {\n        ident: `row_below`,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 77,
                                character: 42,
                            },
                            end: Position {
                                line: 77,
                                character: 51,
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
            601,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 600;\n\ntoken_line_group_idx = 71\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Vertical,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 96,
                                character: 8,
                            },
                            end: Position {
                                line: 96,
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
            676,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 675;\n\ntoken_line_group_idx = 79\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Minus,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 107,
                                character: 11,
                            },
                            end: Position {
                                line: 107,
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
            751,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 750;\n\ntoken_line_group_idx = 90\n\ntoken = TokenData::Literal(\n    LiteralData::Integer(\n        UnspecifiedRegular(\n            1,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 125,
                                character: 6,
                            },
                            end: Position {
                                line: 125,
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
            826,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 825;\n\ntoken_line_group_idx = 106\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Vertical,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 143,
                                character: 12,
                            },
                            end: Position {
                                line: 143,
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
            901,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 900;\n\ntoken_line_group_idx = 117\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 161,
                                character: 57,
                            },
                            end: Position {
                                line: 161,
                                character: 58,
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
            976,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 975;\n\ntoken_line_group_idx = 123\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 171,
                                character: 33,
                            },
                            end: Position {
                                line: 171,
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
            1051,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 1050;\n\ntoken_line_group_idx = 130\n\ntoken = TokenData::Ident(\n    `r_dl`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                42,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 7,\n            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {\n                pattern_symbol_idx: 6,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: Pure,\n    access_start: RegionalTokenIdx(\n        55,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                86,\n            ),\n        ),\n    ),\n    data: CurrentSynSymbolData::LetVariable {\n        ident: `r_dl`,\n        pattern_symbol_idx: 6,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 178,
                                character: 75,
                            },
                            end: Position {
                                line: 178,
                                character: 79,
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
            1126,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 1125;\n\ntoken_line_group_idx = 138\n\ntoken = TokenData::Ident(\n    `j`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                77,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 11,\n            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {\n                pattern_symbol_idx: 9,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        117,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                536,\n            ),\n        ),\n    ),\n    data: CurrentSynSymbolData::LetVariable {\n        ident: `j`,\n        pattern_symbol_idx: 9,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 188,
                                character: 82,
                            },
                            end: Position {
                                line: 188,
                                character: 83,
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
            1201,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 1200;\n\ntoken_line_group_idx = 149\n\ntoken = TokenData::Ident(\n    `inward_direction`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                108,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 14,\n            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {\n                pattern_symbol_idx: 12,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        151,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                536,\n            ),\n        ),\n    ),\n    data: CurrentSynSymbolData::LetVariable {\n        ident: `inward_direction`,\n        pattern_symbol_idx: 12,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 207,
                                character: 20,
                            },
                            end: Position {
                                line: 207,
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
            1276,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 1275;\n\ntoken_line_group_idx = 155\n\ntoken = TokenData::Ident(\n    `i`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                160,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 10,\n            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {\n                pattern_symbol_idx: 8,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        112,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                536,\n            ),\n        ),\n    ),\n    data: CurrentSynSymbolData::LetVariable {\n        ident: `i`,\n        pattern_symbol_idx: 8,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 220,
                                character: 61,
                            },
                            end: Position {
                                line: 220,
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
            1351,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 1350;\n\ntoken_line_group_idx = 163\n\ntoken = TokenData::Ident(\n    `from_i_shift28`,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 233,
                                character: 51,
                            },
                            end: Position {
                                line: 233,
                                character: 65,
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
            1426,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 1425;\n\ntoken_line_group_idx = 179\n\ntoken = TokenData::Ident(\n    `i`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                268,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 10,\n            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {\n                pattern_symbol_idx: 8,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        112,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                536,\n            ),\n        ),\n    ),\n    data: CurrentSynSymbolData::LetVariable {\n        ident: `i`,\n        pattern_symbol_idx: 8,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 253,
                                character: 20,
                            },
                            end: Position {
                                line: 253,
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
            1501,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 1500;\n\ntoken_line_group_idx = 189\n\ntoken = TokenData::Ident(\n    `contour`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                318,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 9,\n            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {\n                pattern_symbol_idx: 7,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        102,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                536,\n            ),\n        ),\n    ),\n    data: CurrentSynSymbolData::LetVariable {\n        ident: `contour`,\n        pattern_symbol_idx: 7,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 265,
                                character: 39,
                            },
                            end: Position {
                                line: 265,
                                character: 46,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
]