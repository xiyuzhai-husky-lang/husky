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
                            value: "\ntoken_idx = 0;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Pound,\n    ),\n);\n\ntoken_info = None;\n\n\n",
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
            10,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 9;\n\ntoken_line_group_idx = 1\n\ntoken = TokenData::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 1,
                                character: 0,
                            },
                            end: Position {
                                line: 1,
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
            19,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 18;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Ident(\n    `Label`,\n);\n\ntoken_info = None;\n\n\n",
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
            28,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 27;\n\ntoken_line_group_idx = 4\n\ntoken = TokenData::Ident(\n    `Clone`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            2,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Trait(\n                    TraitPath(`core::clone::Clone`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Trait(\n                    TraitPath(`core::clone::Clone`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 5,
                                character: 15,
                            },
                            end: Position {
                                line: 5,
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
            37,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 36;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Ident(\n    `phantom`,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 6,
                                character: 19,
                            },
                            end: Position {
                                line: 6,
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
            46,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 45;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::RaOrGt,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 6,
                                character: 61,
                            },
                            end: Position {
                                line: 6,
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
            55,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 54;\n\ntoken_line_group_idx = 8\n\ntoken = TokenData::Ident(\n    `Label`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::TemplateParameter(\n            1,\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 1,\n            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {\n                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {\n                    ident_token: IdentRegionalToken {\n                        ident: `Label`,\n                        regional_token_idx: RegionalTokenIdx(\n                            5,\n                        ),\n                    },\n                },\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: Const,\n    access_start: RegionalTokenIdx(\n        6,\n    ),\n    access_end: None,\n    data: CurrentSynSymbolData::TemplateParameter {\n        syn_attrs: TemplateParameterSynAttrs {\n            syn_attrs: [\n                Phantom(\n                    PoundRegionalToken(\n                        RegionalTokenIdx(\n                            3,\n                        ),\n                    ),\n                    PhantomRegionalToken {\n                        token_idx: RegionalTokenIdx(\n                            4,\n                        ),\n                    },\n                ),\n            ],\n        },\n        annotated_variance_token: None,\n        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {\n            ident_token: IdentRegionalToken {\n                ident: `Label`,\n                regional_token_idx: RegionalTokenIdx(\n                    5,\n                ),\n            },\n        },\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 10,
                                character: 14,
                            },
                            end: Position {
                                line: 10,
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
            64,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 63;\n\ntoken_line_group_idx = 8\n\ntoken = TokenData::Ident(\n    `Default`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            1,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Trait(\n                    TraitPath(`core::default::Default`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Trait(\n                    TraitPath(`core::default::Default`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 10,
                                character: 50,
                            },
                            end: Position {
                                line: 10,
                                character: 57,
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
            73,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 72;\n\ntoken_line_group_idx = 9\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 11,
                                character: 15,
                            },
                            end: Position {
                                line: 11,
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
            82,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 81;\n\ntoken_line_group_idx = 11\n\ntoken = TokenData::Ident(\n    `Label`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::TemplateParameter(\n            1,\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 1,\n            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {\n                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {\n                    ident_token: IdentRegionalToken {\n                        ident: `Label`,\n                        regional_token_idx: RegionalTokenIdx(\n                            3,\n                        ),\n                    },\n                },\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: Const,\n    access_start: RegionalTokenIdx(\n        4,\n    ),\n    access_end: None,\n    data: CurrentSynSymbolData::TemplateParameter {\n        syn_attrs: TemplateParameterSynAttrs {\n            syn_attrs: [],\n        },\n        annotated_variance_token: None,\n        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {\n            ident_token: IdentRegionalToken {\n                ident: `Label`,\n                regional_token_idx: RegionalTokenIdx(\n                    3,\n                ),\n            },\n        },\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 14,
                                character: 5,
                            },
                            end: Position {
                                line: 14,
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
            91,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 90;\n\ntoken_line_group_idx = 11\n\ntoken = TokenData::Ident(\n    `ops`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            2,\n            PrincipalEntityPath::Module(\n                `core::ops`,\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::Module(\n                `core::ops`,\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 14,
                                character: 38,
                            },
                            end: Position {
                                line: 14,
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
            100,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 99;\n\ntoken_line_group_idx = 11\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 14,
                                character: 86,
                            },
                            end: Position {
                                line: 14,
                                character: 87,
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
            109,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 108;\n\ntoken_line_group_idx = 13\n\ntoken = TokenData::Ident(\n    `Debug`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            1,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Trait(\n                    TraitPath(`core::fmt::Debug`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Trait(\n                    TraitPath(`core::fmt::Debug`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 17,
                                character: 8,
                            },
                            end: Position {
                                line: 17,
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
            118,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 117;\n\ntoken_line_group_idx = 14\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::LaOrLt,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 18,
                                character: 23,
                            },
                            end: Position {
                                line: 18,
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
            127,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 126;\n\ntoken_line_group_idx = 14\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Colon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 18,
                                character: 60,
                            },
                            end: Position {
                                line: 18,
                                character: 61,
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
            136,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 135;\n\ntoken_line_group_idx = 18\n\ntoken = TokenData::Keyword(\n    Keyword::Impl,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 23,
                                character: 0,
                            },
                            end: Position {
                                line: 23,
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
            145,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 144;\n\ntoken_line_group_idx = 18\n\ntoken = TokenData::Ident(\n    `label`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::TemplateParameter(\n            2,\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 2,\n            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {\n                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {\n                    ident_token: IdentRegionalToken {\n                        ident: `label`,\n                        regional_token_idx: RegionalTokenIdx(\n                            10,\n                        ),\n                    },\n                },\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: Const,\n    access_start: RegionalTokenIdx(\n        13,\n    ),\n    access_end: None,\n    data: CurrentSynSymbolData::TemplateParameter {\n        syn_attrs: TemplateParameterSynAttrs {\n            syn_attrs: [\n                Phantom(\n                    PoundRegionalToken(\n                        RegionalTokenIdx(\n                            7,\n                        ),\n                    ),\n                    PhantomRegionalToken {\n                        token_idx: RegionalTokenIdx(\n                            8,\n                        ),\n                    },\n                ),\n            ],\n        },\n        annotated_variance_token: None,\n        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {\n            ident_token: IdentRegionalToken {\n                ident: `label`,\n                regional_token_idx: RegionalTokenIdx(\n                    10,\n                ),\n            },\n            ty_expr_idx: 1,\n        },\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 23,
                                character: 36,
                            },
                            end: Position {
                                line: 23,
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
            154,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 153;\n\ntoken_line_group_idx = 18\n\ntoken = TokenData::Ident(\n    `OneVsAllResult`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            4,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`malamute::OneVsAllResult`, `Enum`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`malamute::OneVsAllResult`, `Enum`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 23,
                                character: 68,
                            },
                            end: Position {
                                line: 23,
                                character: 82,
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
            163,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 162;\n\ntoken_line_group_idx = 19\n\ntoken = TokenData::Ident(\n    `Output`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::AstIdentifiable,\n        data: TokenInfoData::EntityNode(\n            ItemSynNodePath::AssociatedItem(\n                AssociatedItemSynNodePath::TraitForTypeItem(\n                    TraitForTypeItemSynNodePath(\n                        ItemSynNodePathId {\n                            data: ItemSynNodePathData::AssociatedItem(\n                                AssociatedItemSynNodePathData::TraitForTypeItem(\n                                    TraitForTypeItemSynNodePathData {\n                                        maybe_ambiguous_path: MaybeAmbiguousPath {\n                                            path: TraitForTypeItemPath(\n                                                ItemPathId {\n                                                    data: ItemPathData::AssociatedItem(\n                                                        AssociatedItemPathData::TraitForTypeItem(\n                                                            TraitForTypeItemPathData {\n                                                                impl_block: TraitForTypeImplBlock {\n                                                                    data: TraitForTypeImplBlockPathData {\n                                                                        module_path: `malamute`,\n                                                                        trai_path: TraitPath(`core::ops::Unveil`),\n                                                                        ty_sketch: TypeSketch::Path(\n                                                                            TypePath(`malamute::OneVsAll`, `Enum`),\n                                                                        ),\n                                                                        disambiguator: 0,\n                                                                    },\n                                                                },\n                                                                ident: `Output`,\n                                                                item_kind: AssociatedType,\n                                                            },\n                                                        ),\n                                                    ),\n                                                },\n                                            ),\n                                            disambiguator: 0,\n                                        },\n                                    },\n                                ),\n                            ),\n                        },\n                    ),\n                ),\n            ),\n            AssociatedItem {\n                associated_item_kind: TraitForTypeItem(\n                    AssociatedType,\n                ),\n            },\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 24,
                                character: 9,
                            },
                            end: Position {
                                line: 24,
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
            172,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 171;\n\ntoken_line_group_idx = 20\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Comma,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 26,
                                character: 24,
                            },
                            end: Position {
                                line: 26,
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
            181,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 180;\n\ntoken_line_group_idx = 20\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::DotDotDot,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 27,
                                character: 4,
                            },
                            end: Position {
                                line: 27,
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
            190,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 189;\n\ntoken_line_group_idx = 20\n\ntoken = TokenData::Ident(\n    `i32`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            2,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::num::i32`, `Extern`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::num::i32`, `Extern`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 28,
                                character: 10,
                            },
                            end: Position {
                                line: 28,
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
            199,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 198;\n\ntoken_line_group_idx = 20\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Semicolon,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 29,
                                character: 31,
                            },
                            end: Position {
                                line: 29,
                                character: 32,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
]