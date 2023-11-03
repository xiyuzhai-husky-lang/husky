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
                4,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 3;\n\ntoken_line_group_idx = 0\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Star,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 0,
                                    character: 11,
                                },
                                end: Position {
                                    line: 0,
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
                7,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 6;\n\ntoken_line_group_idx = 1\n\ntoken = TokenData::Ident(\n    `Result`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::UseExpr(\n            4,\n        ),\n        data: TokenInfoData::UseExpr {\n            use_expr_idx: 4,\n            rule_idx: OnceUseRuleIdx(\n                1,\n            ),\n            state: OnceUseRuleState::Resolved {\n                original_symbol: Some(\n                    EntitySymbol::MajorItem {\n                        module_item_path: MajorItemPath::Type(\n                            TypePath(`core::result::Result`, `Enum`),\n                        ),\n                    },\n                ),\n            },\n        },\n    },\n);\n\nuse\n",
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
                10,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 9;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Keyword(\n    Keyword::Pub,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 0,
                                },
                                end: Position {
                                    line: 4,
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
                13,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 12;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::LaOrLt,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 15,
                                },
                                end: Position {
                                    line: 4,
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
                16,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 15;\n\ntoken_line_group_idx = 2\n\ntoken = TokenData::Ident(\n    `E`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::TemplateParameter(\n            2,\n        ),\n        data: TokenInfoData::CurrentSymbol {\n            current_symbol_idx: 2,\n            current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {\n                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {\n                    ident_token: IdentRegionalToken {\n                        ident: `E`,\n                        regional_token_idx: RegionalTokenIdx(\n                            7,\n                        ),\n                    },\n                },\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nSynCurrentSymbol {\n    modifier: Const,\n    access_start: RegionalTokenIdx(\n        8,\n    ),\n    access_end: None,\n    variant: SynCurrentSymbolVariant::TemplateParameter {\n        syn_attrs: TemplateParameterSynAttrs {\n            syn_attrs: [],\n        },\n        annotated_variance_token: None,\n        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {\n            ident_token: IdentRegionalToken {\n                ident: `E`,\n                regional_token_idx: RegionalTokenIdx(\n                    7,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 4,
                                    character: 19,
                                },
                                end: Position {
                                    line: 4,
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
                19,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 18;\n\ntoken_line_group_idx = 3\n\ntoken = TokenData::Ident(\n    `Ok`,\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 5,
                                    character: 2,
                                },
                                end: Position {
                                    line: 5,
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
                22,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 21;\n\ntoken_line_group_idx = 3\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Ket(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 5,
                                    character: 6,
                                },
                                end: Position {
                                    line: 5,
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
                25,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 24;\n\ntoken_line_group_idx = 4\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Bra(\n            Par,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 6,
                                    character: 5,
                                },
                                end: Position {
                                    line: 6,
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
                28,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 27;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Keyword(\n    Keyword::Impl,\n);\n\ntoken_info = None;\n\n\n",
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
                31,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 30;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Comma,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 8,
                                    character: 7,
                                },
                                end: Position {
                                    line: 8,
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
                34,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 33;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Ident(\n    `E1`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::TemplateParameter(\n            3,\n        ),\n        data: TokenInfoData::CurrentSymbol {\n            current_symbol_idx: 3,\n            current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {\n                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {\n                    ident_token: IdentRegionalToken {\n                        ident: `E1`,\n                        regional_token_idx: RegionalTokenIdx(\n                            7,\n                        ),\n                    },\n                },\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nSynCurrentSymbol {\n    modifier: Const,\n    access_start: RegionalTokenIdx(\n        8,\n    ),\n    access_end: None,\n    variant: SynCurrentSymbolVariant::TemplateParameter {\n        syn_attrs: TemplateParameterSynAttrs {\n            syn_attrs: [],\n        },\n        annotated_variance_token: None,\n        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {\n            ident_token: IdentRegionalToken {\n                ident: `E1`,\n                regional_token_idx: RegionalTokenIdx(\n                    7,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 8,
                                    character: 13,
                                },
                                end: Position {
                                    line: 8,
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
                37,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 36;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::RaOrGt,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 8,
                                    character: 19,
                                },
                                end: Position {
                                    line: 8,
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
                40,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 39;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Ident(\n    `ops`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            2,\n            PrincipalEntityPath::Module(\n                `core::ops`,\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::Module(\n                `core::ops`,\n            ),\n        ),\n    },\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 8,
                                    character: 28,
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
                43,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 42;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Ident(\n    `Result`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SynPrincipalEntityPathExpr(\n            4,\n            PrincipalEntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::result::Result`, `Enum`),\n                ),\n            ),\n        ),\n        data: TokenInfoData::Entity(\n            EntityPath::MajorItem(\n                MajorItemPath::Type(\n                    TypePath(`core::result::Result`, `Enum`),\n                ),\n            ),\n        ),\n    },\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 8,
                                    character: 40,
                                },
                                end: Position {
                                    line: 8,
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
                46,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "Other\ntoken_idx = 45;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Keyword(\n    Keyword::Connection(\n        For,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 8,
                                    character: 53,
                                },
                                end: Position {
                                    line: 8,
                                    character: 56,
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
                                value: "\ntoken_idx = 48;\n\ntoken_line_group_idx = 5\n\ntoken = TokenData::Ident(\n    `E1`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                11,\n            ),\n        ),\n        data: TokenInfoData::CurrentSymbol {\n            current_symbol_idx: 3,\n            current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {\n                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {\n                    ident_token: IdentRegionalToken {\n                        ident: `E1`,\n                        regional_token_idx: RegionalTokenIdx(\n                            7,\n                        ),\n                    },\n                },\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nSynCurrentSymbol {\n    modifier: Const,\n    access_start: RegionalTokenIdx(\n        8,\n    ),\n    access_end: None,\n    variant: SynCurrentSymbolVariant::TemplateParameter {\n        syn_attrs: TemplateParameterSynAttrs {\n            syn_attrs: [],\n        },\n        annotated_variance_token: None,\n        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {\n            ident_token: IdentRegionalToken {\n                ident: `E1`,\n                regional_token_idx: RegionalTokenIdx(\n                    7,\n                ),\n            },\n        },\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 8,
                                    character: 67,
                                },
                                end: Position {
                                    line: 8,
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
                52,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 51;\n\ntoken_line_group_idx = 6\n\ntoken = TokenData::Ident(\n    `Continue`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::AstIdentifiable,\n        data: TokenInfoData::EntityNode(\n            ItemSynNodePath::AssociatedItem(\n                AssociatedItemSynNodePath::TraitForTypeItem(\n                    TraitForTypeItemSynNodePath {\n                        maybe_ambiguous_path: MaybeAmbiguousPath {\n                            path: TraitForTypeItemPath {\n                                impl_block: TraitForTypeImplBlockPath {\n                                    module_path: `core::result`,\n                                    trai_path: TraitPath(`core::ops::Unveil`),\n                                    ty_sketch: TypeSketch::Path(\n                                        TypePath(`core::result::Result`, `Enum`),\n                                    ),\n                                    disambiguator: 0,\n                                },\n                                ident: `Continue`,\n                                item_kind: AssociatedType,\n                            },\n                            disambiguator: 0,\n                        },\n                    },\n                ),\n            ),\n            AssociatedItem {\n                associated_item_kind: TraitForTypeItem(\n                    AssociatedType,\n                ),\n            },\n        ),\n    },\n);\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 9,
                                    character: 9,
                                },
                                end: Position {
                                    line: 9,
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
                55,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "This is a paradigm\ntoken_idx = 54;\n\ntoken_line_group_idx = 7\n\ntoken = TokenData::Keyword(\n    Keyword::Fugitive(\n        Fn,\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 11,
                                    character: 4,
                                },
                                end: Position {
                                    line: 11,
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
                58,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 57;\n\ntoken_line_group_idx = 7\n\ntoken = TokenData::Ident(\n    `result`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::PatternExpr(\n            1,\n        ),\n        data: TokenInfoData::CurrentSymbol {\n            current_symbol_idx: 1,\n            current_symbol_kind: SynCurrentSymbolKind::ExplicitRegularParameter {\n                pattern_symbol_idx: 1,\n            },\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nSynCurrentSymbol {\n    modifier: None,\n    access_start: RegionalTokenIdx(\n        5,\n    ),\n    access_end: None,\n    variant: SynCurrentSymbolVariant::ParenateRegularParameter {\n        ident: `result`,\n        pattern_symbol_idx: 1,\n    },\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 11,
                                    character: 14,
                                },
                                end: Position {
                                    line: 11,
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
                61,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 60;\n\ntoken_line_group_idx = 7\n\ntoken = TokenData::Ident(\n    `T2`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                2,\n            ),\n        ),\n        data: TokenInfoData::InheritedSymbol {\n            inherited_symbol_idx: 2,\n            inherited_symbol_kind: SynInheritedSymbolKind::TemplateParameter(\n                InheritedTemplateParameterSynSymbol::Type {\n                    ident: `T2`,\n                },\n            ),\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nSynInheritedSymbol {\n    parent_symbol_idx: Current(\n        2,\n    ),\n    modifier: Const,\n    kind: SynInheritedSymbolKind::TemplateParameter(\n        InheritedTemplateParameterSynSymbol::Type {\n            ident: `T2`,\n        },\n    ),\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 11,
                                    character: 29,
                                },
                                end: Position {
                                    line: 11,
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
                64,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 63;\n\ntoken_line_group_idx = 7\n\ntoken = TokenData::Punctuation(\n    Punctuation(\n        PunctuationMapped::Binary(\n            CurryType,\n        ),\n    ),\n);\n\ntoken_info = None;\n\n\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 11,
                                    character: 36,
                                },
                                end: Position {
                                    line: 11,
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
                67,
            ),
            Some(
                HoverResult {
                    hover: Hover {
                        contents: Markup(
                            MarkupContent {
                                kind: Markdown,
                                value: "\ntoken_idx = 66;\n\ntoken_line_group_idx = 7\n\ntoken = TokenData::Ident(\n    `E1`,\n);\n\ntoken_info = Some(\n    TokenInfo {\n        src: TokenInfoSource::SemaExpr(\n            SemaExprIdx(\n                9,\n            ),\n        ),\n        data: TokenInfoData::InheritedSymbol {\n            inherited_symbol_idx: 3,\n            inherited_symbol_kind: SynInheritedSymbolKind::TemplateParameter(\n                InheritedTemplateParameterSynSymbol::Type {\n                    ident: `E1`,\n                },\n            ),\n            syn_expr_region: ExprRegionLeash(_),\n        },\n    },\n);\n\nSynInheritedSymbol {\n    parent_symbol_idx: Current(\n        3,\n    ),\n    modifier: Const,\n    kind: SynInheritedSymbolKind::TemplateParameter(\n        InheritedTemplateParameterSynSymbol::Type {\n            ident: `E1`,\n        },\n    ),\n}\n",
                            },
                        ),
                        range: Some(
                            Range {
                                start: Position {
                                    line: 11,
                                    character: 49,
                                },
                                end: Position {
                                    line: 11,
                                    character: 51,
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