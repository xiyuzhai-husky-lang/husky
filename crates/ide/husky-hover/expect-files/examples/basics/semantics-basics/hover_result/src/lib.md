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
                            value: "This is a paradigm\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Keyword(\n    Keyword::Form(\n        Gn,\n    ),\n);\n\ntoken_info = None;\n\n\n",
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
            2,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 1;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Ident(\n    `some_neural_network`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::AstIdentifiable,\n        data: TokenInfoData::EntityNode(\n            ItemSynNodePath::MajorItem(\n                MajorItemSynNodePath::Form(\n                    MajorFormSynNodePath(`semantics_basics::some_neural_network`, `Ritchie(\n                        Gn,\n                    )`, (0)),\n                ),\n            ),\n            EntityKind::MajorItem {\n                module_item_kind: MajorItemKind::Form(\n                    MajorFormKind::Ritchie(\n                        RitchieItemKind::Gn,\n                    ),\n                ),\n                connection: MajorItemConnectionKind::Connected,\n            },\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 3,
                            },
                            end: Position {
                                line: 0,
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
            3,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 2;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::LeftDelimiter(\n            Delimiter::Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 22,
                            },
                            end: Position {
                                line: 0,
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
            4,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 3;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Ident(\n    `input`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::Pattern(\n            0,\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_variable_idx: 0,\n            current_variable_kind: CurrentVariableKind::SimpleParenateParameter {\n                pattern_variable_idx: 0,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Pure,\n    access_start: RegionalTokenIdx(\n        5,\n    ),\n    access_end: None,\n    data: CurrentVariableData::SimpleParenateParameter {\n        ident: `input`,\n        pattern_variable_idx: 0,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 23,
                            },
                            end: Position {
                                line: 0,
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
            5,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 4;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 28,
                            },
                            end: Position {
                                line: 0,
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
            6,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 5;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::LeftDelimiter(\n            Delimiter::Box,\n        ),\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                1,\n            ),\n        ),\n        data: TokenInfoData::ArrayFunctorBoxPrefix,\n    },\n);\n\narray functor box prefix\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 30,
                            },
                            end: Position {
                                line: 0,
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
            7,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 6;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Literal(\n    LiteralTokenData::Integer(\n        UnspecifiedRegular(\n            3,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 31,
                            },
                            end: Position {
                                line: 0,
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
            8,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 7;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::RightDelimiter(\n            Delimiter::Box,\n        ),\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                1,\n            ),\n        ),\n        data: TokenInfoData::ArrayFunctorBoxPrefix,\n    },\n);\n\narray functor box prefix\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 32,
                            },
                            end: Position {
                                line: 0,
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
            9,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 8;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Ident(\n    `f32`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            0,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::num::f32`, `Extern`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::num::f32`, `Extern`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 33,
                            },
                            end: Position {
                                line: 0,
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
            10,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 9;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::RightDelimiter(\n            Delimiter::Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 36,
                            },
                            end: Position {
                                line: 0,
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
            11,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 10;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            SynBinaryOpr::CurryType,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 38,
                            },
                            end: Position {
                                line: 0,
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
            12,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 11;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::LeftDelimiter(\n            Delimiter::Box,\n        ),\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                5,\n            ),\n        ),\n        data: TokenInfoData::ArrayFunctorBoxPrefix,\n    },\n);\n\narray functor box prefix\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 41,
                            },
                            end: Position {
                                line: 0,
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
            13,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 12;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Literal(\n    LiteralTokenData::Integer(\n        UnspecifiedRegular(\n            3,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 42,
                            },
                            end: Position {
                                line: 0,
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
            14,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 13;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::RightDelimiter(\n            Delimiter::Box,\n        ),\n    ),\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                5,\n            ),\n        ),\n        data: TokenInfoData::ArrayFunctorBoxPrefix,\n    },\n);\n\narray functor box prefix\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 43,
                            },
                            end: Position {
                                line: 0,
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
            15,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 14;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Ident(\n    `f32`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            1,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::num::f32`, `Extern`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::num::f32`, `Extern`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 44,
                            },
                            end: Position {
                                line: 0,
                                character: 47,
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
            16,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 15;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 0,
                                character: 47,
                            },
                            end: Position {
                                line: 0,
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
            17,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 16;\n\ntoken_line_group_idx = 1\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::ColonHyphen,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 1,
                                character: 4,
                            },
                            end: Position {
                                line: 1,
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
            18,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 17;\n\ntoken_line_group_idx = 1\n\ntoken = TokenData::Ident(\n    `begin`,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 1,
                                character: 7,
                            },
                            end: Position {
                                line: 1,
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
            19,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 18;\n\ntoken_line_group_idx = 1\n\ntoken = TokenData::Ident(\n    `session`,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 1,
                                character: 13,
                            },
                            end: Position {
                                line: 1,
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
            20,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 19;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::ColonHyphen,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 5,
                                character: 4,
                            },
                            end: Position {
                                line: 5,
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
            21,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 20;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Ident(\n    `end`,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 5,
                                character: 7,
                            },
                            end: Position {
                                line: 5,
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
            22,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 21;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Ident(\n    `session`,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 5,
                                character: 11,
                            },
                            end: Position {
                                line: 5,
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
            23,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 22;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Ident(\n    `by`,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 5,
                                character: 19,
                            },
                            end: Position {
                                line: 5,
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
            24,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 23;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Ident(\n    `sgd`,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 5,
                                character: 22,
                            },
                            end: Position {
                                line: 5,
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
            25,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 24;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::LeftDelimiter(\n            Delimiter::Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 5,
                                character: 25,
                            },
                            end: Position {
                                line: 5,
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
            26,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 25;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Ident(\n    `loss`,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 5,
                                character: 26,
                            },
                            end: Position {
                                line: 5,
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
            27,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 26;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::RightDelimiter(\n            Delimiter::Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 5,
                                character: 30,
                            },
                            end: Position {
                                line: 5,
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
            28,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 27;\n\ntoken_line_group_idx = 3\n\ntoken = TokenData::Keyword(\n    Keyword::Todo,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemExpr(\n            SemExprIdx(\n                0,\n            ),\n        ),\n        data: TokenInfoData::Todo,\n    },\n);\n\ntodo\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 7,
                                character: 4,
                            },
                            end: Position {
                                line: 7,
                                character: 8,
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