```rust
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
                            value: "\ntoken_idx = 14;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                1,\n            ),\n        ),\n        data: TokenInfoData::BoxColon,\n    },\n);\n\nbox colon\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 40,
                            },
                            end: Position {
                                line: 0,
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
            29,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 28;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::LeftDelimiter(\n            Delimiter::Par,\n        ),\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                11,\n            ),\n        ),\n        data: TokenInfoData::CallPar,\n    },\n);\n\ncall par\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 2,
                                character: 18,
                            },
                            end: Position {
                                line: 2,
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
            43,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 42;\n\ntoken_line_group_idx = 3\n\ntoken = TokenData::Ident(\n    `quick_sort_aux`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::AstIdentifiable,\n        data: TokenInfoData::EntityNode(\n            ItemSynNodePath::MajorItem(\n                MajorItemSynNodePath::Form(\n                    FormSynNodePath(`quick_sort::quick_sort_aux`, `Ritchie(\n                        Fn,\n                    )`, (0)),\n                ),\n            ),\n            EntityKind::MajorItem {\n                module_item_kind: MajorItemKind::Form(\n                    MajorFormKind::Ritchie(\n                        RitchieItemKind::Fn,\n                    ),\n                ),\n                connection: MajorItemConnectionKind::Connected,\n            },\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 4,
                                character: 3,
                            },
                            end: Position {
                                line: 4,
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
            57,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 56;\n\ntoken_line_group_idx = 3\n\ntoken = TokenData::Ident(\n    `T`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                2,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_variable_idx: 0,\n            current_variable_kind: CurrentVariableKind::TemplateParameter {\n                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {\n                    ident_token: IdentRegionalToken {\n                        ident: `T`,\n                        regional_token_idx: RegionalTokenIdx(\n                            4,\n                        ),\n                    },\n                },\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Compterm,\n    access_start: RegionalTokenIdx(\n        5,\n    ),\n    access_end: None,\n    data: CurrentVariableData::TemplateParameter {\n        syn_attrs: TemplateParameterSynAttrs {\n            syn_attrs: [],\n        },\n        annotated_variance_token: None,\n        data: CurrentTemplateVariableData::Type {\n            ident_token: IdentRegionalToken {\n                ident: `T`,\n                regional_token_idx: RegionalTokenIdx(\n                    4,\n                ),\n            },\n            trai_syn_expr_idxs: [\n                0,\n            ],\n        },\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 4,
                                character: 42,
                            },
                            end: Position {
                                line: 4,
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
            71,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 70;\n\ntoken_line_group_idx = 4\n\ntoken = TokenData::Ident(\n    `high`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                1,\n            ),\n        ),\n        data: TokenInfoData::InheritedSynSymbol {\n            inherited_syn_symbol_idx: 3,\n            inherited_syn_symbol_kind: InheritedVariableKind::Parenate {\n                ident: `high`,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nInheritedVariable {\n    modifier: Pure,\n    kind: InheritedVariableKind::Parenate {\n        ident: `high`,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 5,
                                character: 13,
                            },
                            end: Position {
                                line: 5,
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
            85,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 84;\n\ntoken_line_group_idx = 6\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::LeftDelimiter(\n            Delimiter::Par,\n        ),\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                14,\n            ),\n        ),\n        data: TokenInfoData::CallPar,\n    },\n);\n\ncall par\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 7,
                                character: 22,
                            },
                            end: Position {
                                line: 7,
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
            99,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 98;\n\ntoken_line_group_idx = 7\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            SynBinaryOpr::Closed(\n                BinaryClosedOpr::Add,\n            ),\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 8,
                                character: 30,
                            },
                            end: Position {
                                line: 8,
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
            113,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 112;\n\ntoken_line_group_idx = 8\n\ntoken = TokenData::Keyword(\n    Keyword::Modifier(\n        Mut,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 10,
                                character: 25,
                            },
                            end: Position {
                                line: 10,
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
            127,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 126;\n\ntoken_line_group_idx = 8\n\ntoken = TokenData::Ident(\n    `isize`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            2,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::num::isize`, `Extern`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::num::isize`, `Extern`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 10,
                                character: 58,
                            },
                            end: Position {
                                line: 10,
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
            141,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 140;\n\ntoken_line_group_idx = 10\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 12,
                                character: 24,
                            },
                            end: Position {
                                line: 12,
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
            155,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 154;\n\ntoken_line_group_idx = 13\n\ntoken = TokenData::Literal(\n    LiteralTokenData::Integer(\n        UnspecifiedRegular(\n            1,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 16,
                                character: 23,
                            },
                            end: Position {
                                line: 16,
                                character: 24,
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
                            value: "\ntoken_idx = 168;\n\ntoken_line_group_idx = 15\n\ntoken = TokenData::Ident(\n    `store_index`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                20,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_variable_idx: 1,\n            current_variable_kind: CurrentVariableKind::LetVariable {\n                pattern_variable_idx: 1,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        10,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                97,\n            ),\n        ),\n    ),\n    data: CurrentVariableData::LetVariable {\n        ident: `store_index`,\n        pattern_variable_idx: 1,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 18,
                                character: 12,
                            },
                            end: Position {
                                line: 18,
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
            183,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 182;\n\ntoken_line_group_idx = 17\n\ntoken = TokenData::WordOpr(\n    WordOpr::As,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 20,
                                character: 48,
                            },
                            end: Position {
                                line: 20,
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
            197,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 196;\n\ntoken_line_group_idx = 19\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            SynBinaryOpr::Comparison(\n                BinaryComparisonOpr::Geq,\n            ),\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 22,
                                character: 23,
                            },
                            end: Position {
                                line: 22,
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
            211,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 210;\n\ntoken_line_group_idx = 22\n\ntoken = TokenData::Ident(\n    `last_index`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                50,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_variable_idx: 2,\n            current_variable_kind: CurrentVariableKind::LetVariable {\n                pattern_variable_idx: 2,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Mut,\n    access_start: RegionalTokenIdx(\n        17,\n    ),\n    access_end: Some(\n        RegionalTokenIdxRangeEnd(\n            RegionalTokenIdx(\n                97,\n            ),\n        ),\n    ),\n    data: CurrentVariableData::LetVariable {\n        ident: `last_index`,\n        pattern_variable_idx: 2,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 25,
                                character: 43,
                            },
                            end: Position {
                                line: 25,
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
            225,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 224;\n\ntoken_line_group_idx = 23\n\ntoken = TokenData::Ident(\n    `usize`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            6,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::num::usize`, `Extern`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::num::usize`, `Extern`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 26,
                                character: 44,
                            },
                            end: Position {
                                line: 26,
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
            239,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 238;\n\ntoken_line_group_idx = 27\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::LeftDelimiter(\n            Delimiter::Box,\n        ),\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                0,\n            ),\n        ),\n        data: TokenInfoData::VecFunctorBoxPrefix,\n    },\n);\n\nvec functor box prefix\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 31,
                                character: 15,
                            },
                            end: Position {
                                line: 31,
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
            253,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 252;\n\ntoken_line_group_idx = 27\n\ntoken = TokenData::Literal(\n    LiteralTokenData::Integer(\n        UnspecifiedRegular(\n            0,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 31,
                                character: 39,
                            },
                            end: Position {
                                line: 31,
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
            267,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "This is a paradigm\ntoken_idx = 266;\n\ntoken_line_group_idx = 29\n\ntoken = TokenData::Keyword(\n    Keyword::Form(\n        Fn,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 36,
                                character: 0,
                            },
                            end: Position {
                                line: 36,
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
            281,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 280;\n\ntoken_line_group_idx = 30\n\ntoken = TokenData::Literal(\n    LiteralTokenData::String(\n        StringLiteralTokenData {\n            data: \"airplane\",\n        },\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 37,
                                character: 38,
                            },
                            end: Position {
                                line: 37,
                                character: 48,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
]
```