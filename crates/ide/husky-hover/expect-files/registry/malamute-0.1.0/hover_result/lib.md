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
                            value: "\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::ColonHyphen,\n    ),\n);\n\ntoken_info = None;\n\n\n",
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
            18,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 17;\n\ntoken_line_group_idx = 1\n\ntoken = TokenData::Ident(\n    `Debug`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            0,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Trait(\n                    TraitPath(`core::fmt::Debug`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Trait(\n                    TraitPath(`core::fmt::Debug`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 2,
                                character: 8,
                            },
                            end: Position {
                                line: 2,
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
            35,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 34;\n\ntoken_line_group_idx = 4\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Vert,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 5,
                                character: 0,
                            },
                            end: Position {
                                line: 5,
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
            52,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 51;\n\ntoken_line_group_idx = 6\n\ntoken = TokenData::Ident(\n    `Label`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::TemplateParameter(\n            0,\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_variable_idx: 0,\n            current_variable_kind: CurrentVariableKind::TemplateParameter {\n                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {\n                    ident_token: IdentRegionalToken {\n                        ident: `Label`,\n                        regional_token_idx: RegionalTokenIdx(\n                            7,\n                        ),\n                    },\n                },\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Compterm,\n    access_start: RegionalTokenIdx(\n        8,\n    ),\n    access_end: None,\n    data: CurrentVariableData::TemplateParameter {\n        syn_attrs: TemplateParameterSynAttrs {\n            syn_attrs: [\n                Phantom(\n                    PoundRegionalToken(\n                        RegionalTokenIdx(\n                            5,\n                        ),\n                    ),\n                    PhanRegionalToken {\n                        token_idx: RegionalTokenIdx(\n                            6,\n                        ),\n                    },\n                ),\n            ],\n        },\n        annotated_variance_token: None,\n        data: CurrentTemplateVariableData::Type {\n            ident_token: IdentRegionalToken {\n                ident: `Label`,\n                regional_token_idx: RegionalTokenIdx(\n                    7,\n                ),\n            },\n            trai_syn_expr_idxs: [],\n        },\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 8,
                                character: 24,
                            },
                            end: Position {
                                line: 8,
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
            69,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 68;\n\ntoken_line_group_idx = 9\n\ntoken = TokenData::Ident(\n    `Label`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::TemplateParameter(\n            0,\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_variable_idx: 0,\n            current_variable_kind: CurrentVariableKind::TemplateParameter {\n                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {\n                    ident_token: IdentRegionalToken {\n                        ident: `Label`,\n                        regional_token_idx: RegionalTokenIdx(\n                            5,\n                        ),\n                    },\n                },\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentVariableEntry {\n    modifier: Compterm,\n    access_start: RegionalTokenIdx(\n        6,\n    ),\n    access_end: None,\n    data: CurrentVariableData::TemplateParameter {\n        syn_attrs: TemplateParameterSynAttrs {\n            syn_attrs: [\n                Phantom(\n                    PoundRegionalToken(\n                        RegionalTokenIdx(\n                            3,\n                        ),\n                    ),\n                    PhanRegionalToken {\n                        token_idx: RegionalTokenIdx(\n                            4,\n                        ),\n                    },\n                ),\n            ],\n        },\n        annotated_variance_token: None,\n        data: CurrentTemplateVariableData::Type {\n            ident_token: IdentRegionalToken {\n                ident: `Label`,\n                regional_token_idx: RegionalTokenIdx(\n                    5,\n                ),\n            },\n            trai_syn_expr_idxs: [],\n        },\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 12,
                                character: 11,
                            },
                            end: Position {
                                line: 12,
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
            86,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 85;\n\ntoken_line_group_idx = 10\n\ntoken = TokenData::Ident(\n    `default`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::AstIdentifiable,\n        data: TokenInfoData::EntityNode(\n            ItemSynNodePath::AssocItem(\n                AssocItemSynNodePath::TraitForTypeItem(\n                    TraitForTypeItemSynNodePath(\n                        ItemSynNodePathId {\n                            data: ItemSynNodePathData::AssocItem(\n                                AssocItemSynNodePathData::TraitForTypeItem(\n                                    TraitForTypeItemSynNodePathData {\n                                        disambiguated_item_path: DisambiguatedItemPath {\n                                            maybe_ambiguous_item_path: TraitForTypeItemPath(\n                                                `<malamute::OneVsAll as core::default::Default(0)>::default`,\n                                                TraitItemKind::AssocRitchie(\n                                                    RitchieItemKind::Fn,\n                                                ),\n                                            ),\n                                            disambiguator: 0,\n                                        },\n                                    },\n                                ),\n                            ),\n                        },\n                    ),\n                ),\n            ),\n            EntityKind::AssocItem {\n                assoc_item_kind: AssocItemKind::TraitForTypeItem(\n                    TraitItemKind::AssocRitchie(\n                        RitchieItemKind::Fn,\n                    ),\n                ),\n            },\n        ),\n    },\n);\n\n\n",
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
            103,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 102;\n\ntoken_line_group_idx = 12\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 16,
                                character: 32,
                            },
                            end: Position {
                                line: 16,
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
            120,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 119;\n\ntoken_line_group_idx = 13\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 17,
                                character: 16,
                            },
                            end: Position {
                                line: 17,
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
            137,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 136;\n\ntoken_line_group_idx = 14\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            SynBinaryOpr::ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 19,
                                character: 66,
                            },
                            end: Position {
                                line: 19,
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
            154,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 153;\n\ntoken_line_group_idx = 16\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            SynBinaryOpr::ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 21,
                                character: 36,
                            },
                            end: Position {
                                line: 21,
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
            171,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 170;\n\ntoken_line_group_idx = 17\n\ntoken = TokenData::Ident(\n    `core`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            10,\n            PrincipalEntityPath::Module(\n                `core`,\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::Module(\n                `core`,\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 22,
                                character: 26,
                            },
                            end: Position {
                                line: 22,
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
            188,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 187;\n\ntoken_line_group_idx = 18\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Comma,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 24,
                                character: 20,
                            },
                            end: Position {
                                line: 24,
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
            205,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 204;\n\ntoken_line_group_idx = 19\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::RaOrGt,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 25,
                                character: 64,
                            },
                            end: Position {
                                line: 25,
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
            222,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 221;\n\ntoken_line_group_idx = 23\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 30,
                                character: 38,
                            },
                            end: Position {
                                line: 30,
                                character: 39,
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
                            value: "\ntoken_idx = 238;\n\ntoken_line_group_idx = 24\n\ntoken = TokenData::Ident(\n    `Output`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::AstIdentifiable,\n        data: TokenInfoData::EntityNode(\n            ItemSynNodePath::AssocItem(\n                AssocItemSynNodePath::TraitForTypeItem(\n                    TraitForTypeItemSynNodePath(\n                        ItemSynNodePathId {\n                            data: ItemSynNodePathData::AssocItem(\n                                AssocItemSynNodePathData::TraitForTypeItem(\n                                    TraitForTypeItemSynNodePathData {\n                                        disambiguated_item_path: DisambiguatedItemPath {\n                                            maybe_ambiguous_item_path: TraitForTypeItemPath(\n                                                `<malamute::OneVsAll as core::ops::Unveil(0)>::Output`,\n                                                TraitItemKind::AssocType,\n                                            ),\n                                            disambiguator: 0,\n                                        },\n                                    },\n                                ),\n                            ),\n                        },\n                    ),\n                ),\n            ),\n            EntityKind::AssocItem {\n                assoc_item_kind: AssocItemKind::TraitForTypeItem(\n                    TraitItemKind::AssocType,\n                ),\n            },\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 31,
                                character: 9,
                            },
                            end: Position {
                                line: 31,
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
            256,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 255;\n\ntoken_line_group_idx = 25\n\ntoken = TokenData::Ident(\n    `ops`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            2,\n            PrincipalEntityPath::Module(\n                `core::ops`,\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::Module(\n                `core::ops`,\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 33,
                                character: 76,
                            },
                            end: Position {
                                line: 33,
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
            273,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 272;\n\ntoken_line_group_idx = 27\n\ntoken = TokenData::Ident(\n    `ops`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            3,\n            PrincipalEntityPath::Module(\n                `core::ops`,\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::Module(\n                `core::ops`,\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 35,
                                character: 48,
                            },
                            end: Position {
                                line: 35,
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
            290,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 289;\n\ntoken_line_group_idx = 28\n\ntoken = TokenData::Ident(\n    `ops`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            11,\n            PrincipalEntityPath::Module(\n                `core::ops`,\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::Module(\n                `core::ops`,\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 36,
                                character: 47,
                            },
                            end: Position {
                                line: 36,
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
            307,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 306;\n\ntoken_line_group_idx = 29\n\ntoken = TokenData::Ident(\n    `ops`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            19,\n            PrincipalEntityPath::Module(\n                `core::ops`,\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::Module(\n                `core::ops`,\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 37,
                                character: 47,
                            },
                            end: Position {
                                line: 37,
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
            324,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "This is a paradigm\ntoken_idx = 323;\n\ntoken_line_group_idx = 30\n\ntoken = TokenData::Keyword(\n    Keyword::Form(\n        Compterm,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 39,
                                character: 32,
                            },
                            end: Position {
                                line: 39,
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
            341,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 340;\n\ntoken_line_group_idx = 30\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Eq,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 41,
                                character: 20,
                            },
                            end: Position {
                                line: 41,
                                character: 21,
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