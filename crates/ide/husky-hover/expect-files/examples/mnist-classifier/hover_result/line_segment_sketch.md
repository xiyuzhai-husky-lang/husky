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
                75,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 75;\n\ntoken_line_group_idx = 10\n\ntoken = Token::Ident(\n    `clone`,\n);\n\ntoken_info = TokenInfo::Method;\n\n\n",
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
                150,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 150;\n\ntoken_line_group_idx = 17\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                225,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 225;\n\ntoken_line_group_idx = 28\n\ntoken = Token::Keyword(\n    Keyword::Pronoun(\n        SelfValue,\n    ),\n);\n\ntoken_info = TokenInfo::SelfValue;\n\nself value\n",
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
                300,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 300;\n\ntoken_line_group_idx = 36\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Dot,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                375,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 375;\n\ntoken_line_group_idx = 42\n\ntoken = Token::Ident(\n    `r`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {\n        pattern_symbol_idx: 1,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: None,\n    access_start: TokenIdx(\n        376,\n    ),\n    access_end: None,\n    variant: CurrentSynSymbolVariant::ParenateRegularParameter {\n        ident: `r`,\n        pattern_symbol_idx: 1,\n    },\n}\n",
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
                450,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 450;\n\ntoken_line_group_idx = 47\n\ntoken = Token::Ident(\n    `L`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 0,\n    current_symbol_kind: CurrentSynSymbolKind::LetVariable {\n        pattern_symbol_idx: 0,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: None,\n    access_start: TokenIdx(\n        384,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                465,\n            ),\n        ),\n    ),\n    variant: CurrentSynSymbolVariant::LetVariable {\n        ident: `L`,\n        pattern_symbol_idx: 0,\n    },\n}\n",
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
                525,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 525;\n\ntoken_line_group_idx = 52\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                600,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 600;\n\ntoken_line_group_idx = 59\n\ntoken = Token::Keyword(\n    Keyword::Stmt(\n        Let,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                675,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 675;\n\ntoken_line_group_idx = 68\n\ntoken = Token::Keyword(\n    Keyword::Modifier(\n        Mut,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                750,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 750;\n\ntoken_line_group_idx = 78\n\ntoken = Token::Ident(\n    `right_bound`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 4,\n    current_symbol_kind: CurrentSynSymbolKind::LetVariable {\n        pattern_symbol_idx: 4,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Mut,\n    access_start: TokenIdx(\n        657,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                795,\n            ),\n        ),\n    ),\n    variant: CurrentSynSymbolVariant::LetVariable {\n        ident: `right_bound`,\n        pattern_symbol_idx: 4,\n    },\n}\n",
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
                825,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 825;\n\ntoken_line_group_idx = 88\n\ntoken = Token::Ident(\n    `dp0`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 1,\n    current_symbol_kind: CurrentSynSymbolKind::LetVariable {\n        pattern_symbol_idx: 1,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: Mut,\n    access_start: TokenIdx(\n        826,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                1061,\n            ),\n        ),\n    ),\n    variant: CurrentSynSymbolVariant::LetVariable {\n        ident: `dp0`,\n        pattern_symbol_idx: 1,\n    },\n}\n",
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
                900,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 900;\n\ntoken_line_group_idx = 95\n\ntoken = Token::Ident(\n    `r`,\n);\n\ntoken_info = TokenInfo::InheritedSymbol {\n    inherited_symbol_idx: 3,\n    inherited_symbol_kind: InheritedSynSymbolKind::ParenateParameter {\n        ident: `r`,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nInheritedSynSymbol {\n    parent_symbol_idx: Current(\n        3,\n    ),\n    modifier: None,\n    kind: InheritedSynSymbolKind::ParenateParameter {\n        ident: `r`,\n    },\n}\n",
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
                975,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 975;\n\ntoken_line_group_idx = 107\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                1050,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 1050;\n\ntoken_line_group_idx = 118\n\ntoken = Token::Keyword(\n    Keyword::Stmt(\n        If,\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                1125,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1125;\n\ntoken_line_group_idx = 129\n\ntoken = Token::Ident(\n    `ls_extend_end`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 4,\n    current_symbol_kind: CurrentSynSymbolKind::LetVariable {\n        pattern_symbol_idx: 4,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: None,\n    access_start: TokenIdx(\n        1126,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                1417,\n            ),\n        ),\n    ),\n    variant: CurrentSynSymbolVariant::LetVariable {\n        ident: `ls_extend_end`,\n        pattern_symbol_idx: 4,\n    },\n}\n",
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
                1200,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1200;\n\ntoken_line_group_idx = 135\n\ntoken = Token::Ident(\n    `points`,\n);\n\ntoken_info = TokenInfo::Field;\n\n\n",
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
                1275,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1275;\n\ntoken_line_group_idx = 142\n\ntoken = Token::Ident(\n    `ls_last`,\n);\n\ntoken_info = TokenInfo::CurrentSymbol {\n    current_symbol_idx: 10,\n    current_symbol_kind: CurrentSynSymbolKind::LetVariable {\n        pattern_symbol_idx: 10,\n    },\n    syn_expr_region: ExprRegionLeash(_),\n};\n\nCurrentSynSymbol {\n    modifier: None,\n    access_start: TokenIdx(\n        1276,\n    ),\n    access_end: Some(\n        TokenIdxRangeEnd(\n            TokenIdx(\n                1390,\n            ),\n        ),\n    ),\n    variant: CurrentSynSymbolVariant::LetVariable {\n        ident: `ls_last`,\n        pattern_symbol_idx: 10,\n    },\n}\n",
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
                1350,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1350;\n\ntoken_line_group_idx = 146\n\ntoken = Token::Ident(\n    `dot`,\n);\n\ntoken_info = TokenInfo::Method;\n\n\n",
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
                1425,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1425;\n\ntoken_line_group_idx = 154\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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
                1500,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 1500;\n\ntoken_line_group_idx = 159\n\ntoken = Token::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = TokenInfo::None;\n\n\n",
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