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
            17,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 16;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Ident(\n    `Known`,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 2,
                                character: 2,
                            },
                            end: Position {
                                line: 2,
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
            33,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 32;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Keyword(\n    Keyword::TypeEntity(\n        Enum,\n    ),\n);\n\ntoken_info = None;\n\n\n",
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
                                character: 8,
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
            49,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 48;\n\ntoken_line_group_idx = 7\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Vertical,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 8,
                                character: 0,
                            },
                            end: Position {
                                line: 8,
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
            65,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 64;\n\ntoken_line_group_idx = 8\n\ntoken = TokenData::Keyword(\n    Keyword::Connection(\n        For,\n    ),\n);\n\ntoken_info = None;\n\n\n",
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
            81,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 80;\n\ntoken_line_group_idx = 11\n\ntoken = TokenData::Keyword(\n    Keyword::Impl,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 14,
                                character: 0,
                            },
                            end: Position {
                                line: 14,
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
            97,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 96;\n\ntoken_line_group_idx = 11\n\ntoken = TokenData::Ident(\n    `OneVsAll`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            4,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`malamute::OneVsAll`, `Enum`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`malamute::OneVsAll`, `Enum`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 14,
                                character: 59,
                            },
                            end: Position {
                                line: 14,
                                character: 67,
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
                            value: "\ntoken_idx = 112;\n\ntoken_line_group_idx = 13\n\ntoken = TokenData::Ident(\n    `one_vs_all`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::PatternExpr(\n            1,\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 1,\n            current_syn_symbol_kind: CurrentSynSymbolKind::ExplicitRegularParameter {\n                pattern_symbol_idx: 1,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: Pure,\n    access_start: RegionalTokenIdx(\n        6,\n    ),\n    access_end: None,\n    data: CurrentSynSymbolData::ParenateRegularParameter {\n        ident: `one_vs_all`,\n        pattern_symbol_idx: 1,\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 17,
                                character: 21,
                            },
                            end: Position {
                                line: 17,
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
            129,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 128;\n\ntoken_line_group_idx = 14\n\ntoken = TokenData::Keyword(\n    Keyword::Stmt(\n        Match,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 18,
                                character: 8,
                            },
                            end: Position {
                                line: 18,
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
            145,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 144;\n\ntoken_line_group_idx = 15\n\ntoken = TokenData::Ident(\n    `Class`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            7,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`malamute::Class`, `Enum`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`malamute::Class`, `Enum`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 19,
                                character: 57,
                            },
                            end: Position {
                                line: 19,
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
            161,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 160;\n\ntoken_line_group_idx = 16\n\ntoken = TokenData::Ident(\n    `ControlFlow`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            13,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::ops::ControlFlow`, `Enum`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::ops::ControlFlow`, `Enum`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 20,
                                character: 37,
                            },
                            end: Position {
                                line: 20,
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
            177,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "Other\ntoken_idx = 176;\n\ntoken_line_group_idx = 18\n\ntoken = TokenData::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = None;\n\n\n",
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
            193,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 192;\n\ntoken_line_group_idx = 19\n\ntoken = TokenData::Ident(\n    `ConfidentYes`,\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 24,
                                character: 2,
                            },
                            end: Position {
                                line: 24,
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
            209,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 208;\n\ntoken_line_group_idx = 22\n\ntoken = TokenData::Ident(\n    `Label`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                1,\n            ),\n        ),\n        data: TokenInfoData::CurrentSynSymbol {\n            current_syn_symbol_idx: 1,\n            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {\n                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {\n                    ident_token: IdentRegionalToken {\n                        ident: `Label`,\n                        regional_token_idx: RegionalTokenIdx(\n                            5,\n                        ),\n                    },\n                },\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nCurrentSynSymbol {\n    modifier: Const,\n    access_start: RegionalTokenIdx(\n        6,\n    ),\n    access_end: None,\n    data: CurrentSynSymbolData::TemplateParameter {\n        syn_attrs: TemplateParameterSynAttrs {\n            syn_attrs: [\n                Phantom(\n                    PoundRegionalToken(\n                        RegionalTokenIdx(\n                            3,\n                        ),\n                    ),\n                    PhantomRegionalToken {\n                        token_idx: RegionalTokenIdx(\n                            4,\n                        ),\n                    },\n                ),\n            ],\n        },\n        annotated_variance_token: None,\n        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {\n            ident_token: IdentRegionalToken {\n                ident: `Label`,\n                regional_token_idx: RegionalTokenIdx(\n                    5,\n                ),\n            },\n        },\n    },\n}\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 28,
                                character: 43,
                            },
                            end: Position {
                                line: 28,
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
            225,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 224;\n\ntoken_line_group_idx = 23\n\ntoken = TokenData::Ident(\n    `Output`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::AstIdentifiable,\n        data: TokenInfoData::EntityNode(\n            ItemSynNodePath::AssociatedItem(\n                AssociatedItemSynNodePath::TraitForTypeItem(\n                    TraitForTypeItemSynNodePath(\n                        ItemSynNodePathId {\n                            data: ItemSynNodePathData::AssociatedItem(\n                                AssociatedItemSynNodePathData::TraitForTypeItem(\n                                    TraitForTypeItemSynNodePathData {\n                                        maybe_ambiguous_path: MaybeAmbiguousPath {\n                                            path: TraitForTypeItemPath(\n                                                ItemPathId {\n                                                    data: ItemPathData::AssociatedItem(\n                                                        AssociatedItemPathData::TraitForTypeItem(\n                                                            TraitForTypeItemPathData {\n                                                                impl_block: TraitForTypeImplBlock {\n                                                                    data: TraitForTypeImplBlockPathData {\n                                                                        module_path: `malamute`,\n                                                                        trai_path: TraitPath(`core::ops::Unveil`),\n                                                                        ty_sketch: TypeSketch::Path(\n                                                                            TypePath(`malamute::OneVsAll`, `Enum`),\n                                                                        ),\n                                                                        disambiguator: 0,\n                                                                    },\n                                                                },\n                                                                ident: `Output`,\n                                                                item_kind: AssociatedType,\n                                                            },\n                                                        ),\n                                                    ),\n                                                },\n                                            ),\n                                            disambiguator: 0,\n                                        },\n                                    },\n                                ),\n                            ),\n                        },\n                    ),\n                ),\n            ),\n            AssociatedItem {\n                associated_item_kind: TraitForTypeItem(\n                    AssociatedType,\n                ),\n            },\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 29,
                                character: 9,
                            },
                            end: Position {
                                line: 29,
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
            241,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 240;\n\ntoken_line_group_idx = 24\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            ScopeResolution,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 31,
                                character: 75,
                            },
                            end: Position {
                                line: 31,
                                character: 77,
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
            257,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 256;\n\ntoken_line_group_idx = 26\n\ntoken = TokenData::Ident(\n    `core`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            3,\n            PrincipalEntityPath::Module(\n                `core`,\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::Module(\n                `core`,\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 33,
                                character: 42,
                            },
                            end: Position {
                                line: 33,
                                character: 46,
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
                            value: "\ntoken_idx = 272;\n\ntoken_line_group_idx = 27\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::HeavyArrow,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 34,
                                character: 38,
                            },
                            end: Position {
                                line: 34,
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
            289,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 288;\n\ntoken_line_group_idx = 28\n\ntoken = TokenData::Ident(\n    `Unconfident`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            18,\n            PrincipalEntityPath::TypeVariant(\n                TypeVariantPath(\n                    ItemPathId {\n                        data: ItemPathData::TypeVariant(\n                            TypeVariantPathData {\n                                parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),\n                                ident: `Unconfident`,\n                                index: U8(\n                                    2,\n                                ),\n                            },\n                        ),\n                    },\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::TypeVariant(\n                Room32,\n                TypeVariantPath(\n                    ItemPathId {\n                        data: ItemPathData::TypeVariant(\n                            TypeVariantPathData {\n                                parent_ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),\n                                ident: `Unconfident`,\n                                index: U8(\n                                    2,\n                                ),\n                            },\n                        ),\n                    },\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 35,
                                character: 26,
                            },
                            end: Position {
                                line: 35,
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
            305,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 304;\n\ntoken_line_group_idx = 29\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::LaOrLt,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 37,
                                character: 18,
                            },
                            end: Position {
                                line: 37,
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
            321,
        ),
        Some(
            HoverResult {
                hover: Hover {
                    contents: Markup(
                        MarkupContent {
                            kind: Markdown,
                            value: "\ntoken_idx = 320;\n\ntoken_line_group_idx = 29\n\ntoken = TokenData::Ident(\n    `f32`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            1,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::num::f32`, `Extern`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::num::f32`, `Extern`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                        },
                    ),
                    range: Some(
                        Range {
                            start: Position {
                                line: 38,
                                character: 13,
                            },
                            end: Position {
                                line: 38,
                                character: 16,
                            },
                        },
                    ),
                },
                actions: [],
            },
        ),
    ),
]