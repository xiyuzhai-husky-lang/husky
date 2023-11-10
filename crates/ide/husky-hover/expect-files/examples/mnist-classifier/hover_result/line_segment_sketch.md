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
                                value: "Other\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Keyword(\n    Keyword::Mod,\n);\n\ntoken_info = None;\n\n\n",
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
                                value: "\ntoken_idx = 75;\n\ntoken_line_group_idx = 10\n\ntoken = TokenData::Ident(\n    `clone`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                14,\n            ),\n        ),\n        data: TokenInfoData::Method,\n    },\n);\n\n\n",
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
                151,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 150;\n\ntoken_line_group_idx = 17\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                12,\n            ),\n        ),\n        data: TokenInfoData::CallPar,\n    },\n);\n\ncall par\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 26,
                                    character: 70,
                                },
                                end: Position {
                                    line: 26,
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
                226,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 225;\n\ntoken_line_group_idx = 28\n\ntoken = TokenData::Keyword(\n    Keyword::Pronoun(\n        SelfValue,\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                1,\n            ),\n        ),\n        data: TokenInfoData::SelfValue,\n    },\n);\n\nself value\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 45,
                                    character: 26,
                                },
                                end: Position {
                                    line: 45,
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
                301,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 300;\n\ntoken_line_group_idx = 36\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Dot,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 53,
                                    character: 33,
                                },
                                end: Position {
                                    line: 53,
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
                376,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 375;\n\ntoken_line_group_idx = 42\n\ntoken = TokenData::Ident(\n    `r`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::PatternExpr(\n            2,\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 2,\n            current_syn_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {\n                pattern_symbol_idx: 2,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: None,\n    access_start: RegionalTokenIdx(\n        9,\n    ),\n    access_end: None,\n    data: CurrentSynSymbolData::ParenateRegularParameter {\n        ident: `r`,\n        pattern_symbol_idx: 2,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 64,
                                    character: 25,
                                },
                                end: Position {
                                    line: 64,
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
                451,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 450;\n\ntoken_line_group_idx = 47\n\ntoken = TokenData::Ident(\n    `L`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                41,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 1,\n            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {\n                pattern_symbol_idx: 1,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: None,\n    access_start: RegionalTokenIdx(\n        3,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                84,\n            ),\n        ),\n    ),\n    data: CurrentSynSymbolData::LetVariable {\n        ident: `L`,\n        pattern_symbol_idx: 1,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 69,
                                    character: 21,
                                },
                                end: Position {
                                    line: 69,
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
                526,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 525;\n\ntoken_line_group_idx = 52\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
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
                601,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 600;\n\ntoken_line_group_idx = 59\n\ntoken = TokenData::Keyword(\n    Keyword::Stmt(\n        Let,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 83,
                                    character: 4,
                                },
                                end: Position {
                                    line: 83,
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
                676,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 675;\n\ntoken_line_group_idx = 68\n\ntoken = TokenData::Keyword(\n    Keyword::Modifier(\n        Mut,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 92,
                                    character: 8,
                                },
                                end: Position {
                                    line: 92,
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
                751,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 750;\n\ntoken_line_group_idx = 78\n\ntoken = TokenData::Ident(\n    `right_bound`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                85,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 5,\n            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {\n                pattern_symbol_idx: 5,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        77,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                215,\n            ),\n        ),\n    ),\n    data: CurrentSynSymbolData::LetVariable {\n        ident: `right_bound`,\n        pattern_symbol_idx: 5,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 104,
                                    character: 15,
                                },
                                end: Position {
                                    line: 104,
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
                826,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 825;\n\ntoken_line_group_idx = 88\n\ntoken = TokenData::Ident(\n    `dp0`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::PatternExpr(\n            2,\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 2,\n            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {\n                pattern_symbol_idx: 2,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        9,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                244,\n            ),\n        ),\n    ),\n    data: CurrentSynSymbolData::LetVariable {\n        ident: `dp0`,\n        pattern_symbol_idx: 2,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 115,
                                    character: 12,
                                },
                                end: Position {
                                    line: 115,
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
                901,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 900;\n\ntoken_line_group_idx = 95\n\ntoken = TokenData::Ident(\n    `r`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                40,\n            ),\n        ),\n        data: TokenInfoData::InheritedSynSymbol {\n            inherited_syn_symbol_idx: 4,\n            inherited_syn_symbol_kind: InheritedSynSymbolKind::ParenateParameter {\n                ident: `r`,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nInheritedSynSymbol {\n    parent_symbol_idx: Current(\n        4,\n    ),\n    modifier: None,\n    kind: InheritedSynSymbolKind::ParenateParameter {\n        ident: `r`,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 122,
                                    character: 40,
                                },
                                end: Position {
                                    line: 122,
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
                976,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 975;\n\ntoken_line_group_idx = 107\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 134,
                                    character: 24,
                                },
                                end: Position {
                                    line: 134,
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
                1051,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 1050;\n\ntoken_line_group_idx = 118\n\ntoken = TokenData::Keyword(\n    Keyword::Stmt(\n        If,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 150,
                                    character: 4,
                                },
                                end: Position {
                                    line: 150,
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
                1126,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1125;\n\ntoken_line_group_idx = 129\n\ntoken = TokenData::Ident(\n    `ls_extend_end`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::PatternExpr(\n            5,\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 5,\n            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {\n                pattern_symbol_idx: 5,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: None,\n    access_start: RegionalTokenIdx(\n        49,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                340,\n            ),\n        ),\n    ),\n    data: CurrentSynSymbolData::LetVariable {\n        ident: `ls_extend_end`,\n        pattern_symbol_idx: 5,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 162,
                                    character: 12,
                                },
                                end: Position {
                                    line: 162,
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
                1201,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1200;\n\ntoken_line_group_idx = 135\n\ntoken = TokenData::Ident(\n    `points`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                49,\n            ),\n        ),\n        data: TokenInfoData::Field,\n    },\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 169,
                                    character: 27,
                                },
                                end: Position {
                                    line: 169,
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
                1276,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1275;\n\ntoken_line_group_idx = 142\n\ntoken = TokenData::Ident(\n    `ls_last`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::PatternExpr(\n            11,\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 11,\n            current_syn_symbol_kind: CurrentSynSymbolKind::LetVariable {\n                pattern_symbol_idx: 11,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: None,\n    access_start: RegionalTokenIdx(\n        199,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                313,\n            ),\n        ),\n    ),\n    data: CurrentSynSymbolData::LetVariable {\n        ident: `ls_last`,\n        pattern_symbol_idx: 11,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 176,
                                    character: 20,
                                },
                                end: Position {
                                    line: 176,
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
                1351,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1350;\n\ntoken_line_group_idx = 146\n\ntoken = TokenData::Ident(\n    `dot`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                118,\n            ),\n        ),\n        data: TokenInfoData::Method,\n    },\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 183,
                                    character: 31,
                                },
                                end: Position {
                                    line: 183,
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
                1426,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1425;\n\ntoken_line_group_idx = 154\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
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
                                value: "\ntoken_idx = 1500;\n\ntoken_line_group_idx = 159\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 199,
                                    character: 32,
                                },
                                end: Position {
                                    line: 199,
                                    character: 33,
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