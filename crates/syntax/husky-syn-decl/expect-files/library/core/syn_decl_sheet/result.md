```rust
SynDeclSheet {
    decls: [
        (
            ItemPath::MajorItem(
                MajorItemPath::Type(
                    TypePath(`core::result::Result`, `Enum`),
                ),
            ),
            SynDecl::MajorItem(
                MajorItemSynDecl::Type(
                    TypeSynDecl::Enum(
                        EnumSynDecl {
                            path: TypePath(`core::result::Result`, `Enum`),
                            template_parameters: [
                                TemplateSynParameterData {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: TemplateParameterSyndicateVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `T`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                        traits: None,
                                    },
                                },
                                TemplateSynParameterData {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    variant: TemplateParameterSyndicateVariant::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `E`,
                                            regional_token_idx: RegionalTokenIdx(
                                                7,
                                            ),
                                        },
                                        traits: None,
                                    },
                                },
                            ],
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Type(
                                                TypeSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::MajorItem(
                                                            MajorItemSynNodePathData::Type(
                                                                TypeSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TypePath(`core::result::Result`, `Enum`),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbolEntry {
                                                    modifier: Const,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `T`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbolEntry {
                                                    modifier: Const,
                                                    access_start: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `E`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    7,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
        (
            ItemPath::ImplBlock(
                ImplBlockPath::TraitForTypeImplBlock(
                    TraitForTypeImplBlockPath(`core::result::Result as core::ops::Unveil(0)`),
                ),
            ),
            SynDecl::ImplBlock(
                ImplBlockSynDecl::TraitForType(
                    TraitForTypeImplBlockSynDecl {
                        path: TraitForTypeImplBlockPath(`core::result::Result as core::ops::Unveil(0)`),
                        template_parameters: [
                            TemplateSynParameterData {
                                annotated_variance_token: None,
                                symbol: 0,
                                variant: TemplateParameterSyndicateVariant::Type {
                                    ident_token: IdentRegionalToken {
                                        ident: `T1`,
                                        regional_token_idx: RegionalTokenIdx(
                                            3,
                                        ),
                                    },
                                    traits: None,
                                },
                            },
                            TemplateSynParameterData {
                                annotated_variance_token: None,
                                symbol: 1,
                                variant: TemplateParameterSyndicateVariant::Type {
                                    ident_token: IdentRegionalToken {
                                        ident: `T2`,
                                        regional_token_idx: RegionalTokenIdx(
                                            5,
                                        ),
                                    },
                                    traits: None,
                                },
                            },
                            TemplateSynParameterData {
                                annotated_variance_token: None,
                                symbol: 2,
                                variant: TemplateParameterSyndicateVariant::Type {
                                    ident_token: IdentRegionalToken {
                                        ident: `E1`,
                                        regional_token_idx: RegionalTokenIdx(
                                            7,
                                        ),
                                    },
                                    traits: None,
                                },
                            },
                            TemplateSynParameterData {
                                annotated_variance_token: None,
                                symbol: 3,
                                variant: TemplateParameterSyndicateVariant::Type {
                                    ident_token: IdentRegionalToken {
                                        ident: `E2`,
                                        regional_token_idx: RegionalTokenIdx(
                                            9,
                                        ),
                                    },
                                    traits: None,
                                },
                            },
                        ],
                        trai_expr: TraitSyndicate {
                            syn_expr_idx: 6,
                        },
                        self_ty_decl: PathLeadingExpr(
                            SelfTypeSyndicate {
                                expr: 11,
                            },
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::Decl(
                                    ItemSynNodePath::ImplBlock(
                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                            TraitForTypeImplBlockSynNodePath(
                                                ItemSynNodePathId {
                                                    data: ItemSynNodePathData::ImplBlock(
                                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePathData {
                                                                path: TraitForTypeImplBlockPath(`core::result::Result as core::ops::Unveil(0)`),
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                syn_expr_arena: Arena {
                                    data: [
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Trait(
                                                        TraitPath(`core::ops::Unveil`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::result::Result`, `Enum`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 0,
                                            argument_expr_idx: 1,
                                        },
                                        SynExprData::CurrentSynSymbol {
                                            ident: `T2`,
                                            regional_token_idx: RegionalTokenIdx(
                                                17,
                                            ),
                                            current_syn_symbol_idx: 1,
                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `T2`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 2,
                                            argument_expr_idx: 3,
                                        },
                                        SynExprData::CurrentSynSymbol {
                                            ident: `E2`,
                                            regional_token_idx: RegionalTokenIdx(
                                                18,
                                            ),
                                            current_syn_symbol_idx: 3,
                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `E2`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 4,
                                            argument_expr_idx: 5,
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 4,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::result::Result`, `Enum`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::CurrentSynSymbol {
                                            ident: `T1`,
                                            regional_token_idx: RegionalTokenIdx(
                                                21,
                                            ),
                                            current_syn_symbol_idx: 0,
                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `T1`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            3,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 7,
                                            argument_expr_idx: 8,
                                        },
                                        SynExprData::CurrentSynSymbol {
                                            ident: `E1`,
                                            regional_token_idx: RegionalTokenIdx(
                                                22,
                                            ),
                                            current_syn_symbol_idx: 2,
                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `E1`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        SynExprData::ExplicitApplication {
                                            function_expr_idx: 9,
                                            argument_expr_idx: 10,
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::CrateRootMod(
                                                CrateRegionalToken {
                                                    token_idx: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::Module(
                                                `core`,
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Subitem {
                                            parent: 0,
                                            colon_colon_token: ColonColonRegionalToken(
                                                RegionalTokenIdx(
                                                    12,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentRegionalToken {
                                                    ident: `ops`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                PrincipalEntityPath::Module(
                                                    `core::ops`,
                                                ),
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Subitem {
                                            parent: 1,
                                            colon_colon_token: ColonColonRegionalToken(
                                                RegionalTokenIdx(
                                                    14,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentRegionalToken {
                                                    ident: `Unveil`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Trait(
                                                        TraitPath(`core::ops::Unveil`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `Result`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::result::Result`, `Enum`),
                                                ),
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `Result`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        20,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::result::Result`, `Enum`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: SynPatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [],
                                    },
                                    pattern_symbol_arena: Arena {
                                        data: [],
                                    },
                                    pattern_symbol_maps: [],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [],
                                    },
                                },
                                symbol_region: VariableRegionData {
                                    inherited_syn_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_syn_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbolEntry {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    4,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `T1`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                3,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            CurrentSynSymbolEntry {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    6,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `T2`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            CurrentSynSymbolEntry {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    8,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `E1`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                7,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            CurrentSynSymbolEntry {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    10,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: None,
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `E2`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                9,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: True,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                0..1,
                                            ),
                                        ),
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                1..2,
                                            ),
                                        ),
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                2..3,
                                            ),
                                        ),
                                        (
                                            TemplateTypeParameter,
                                            ArenaIdxRange(
                                                3..4,
                                            ),
                                        ),
                                    ],
                                },
                                syn_pattern_expr_roots: [],
                                syn_expr_roots: [
                                    SynExprRoot {
                                        kind: SynExprRootKind::Trait,
                                        syn_expr_idx: 6,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::SelfType,
                                        syn_expr_idx: 11,
                                    },
                                ],
                                has_self_lifetime: false,
                                has_self_place: false,
                                syn_pattern_to_current_syn_symbol_map: [],
                            },
                        },
                    },
                ),
            ),
        ),
        (
            ItemPath::AssocItem(
                AssocItemPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        `<core::result::Result as core::ops::Unveil(0)>::Continue`,
                        TraitItemKind::AssocType,
                    ),
                ),
            ),
            SynDecl::AssocItem(
                AssocItemSynDecl::TraitForTypeItem(
                    TraitForTypeItemSynDecl::AssocType(
                        TraitForTypeAssocTypeSynDecl {
                            path: TraitForTypeItemPath(
                                `<core::result::Result as core::ops::Unveil(0)>::Continue`,
                                TraitItemKind::AssocType,
                            ),
                            template_parameters: [],
                            ty_term_expr_idx: 0,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                                            TraitForTypeImplBlockSynNodePathData {
                                                                                path: TraitForTypeImplBlockPath(`core::result::Result as core::ops::Unveil(0)`),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                syn_expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::result::Result`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 0,
                                                            argument_expr_idx: 1,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `T2`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                17,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `T2`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 2,
                                                            argument_expr_idx: 3,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `E2`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                            current_syn_symbol_idx: 3,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `E2`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            9,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 4,
                                                            argument_expr_idx: 5,
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 4,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::result::Result`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `T1`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                21,
                                                            ),
                                                            current_syn_symbol_idx: 0,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `T1`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 7,
                                                            argument_expr_idx: 8,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `E1`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                22,
                                                            ),
                                                            current_syn_symbol_idx: 2,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `E1`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            7,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 9,
                                                            argument_expr_idx: 10,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::CrateRootMod(
                                                                CrateRegionalToken {
                                                                    token_idx: RegionalTokenIdx(
                                                                        11,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::Module(
                                                                `core`,
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Subitem {
                                                            parent: 0,
                                                            colon_colon_token: ColonColonRegionalToken(
                                                                RegionalTokenIdx(
                                                                    12,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `ops`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        13,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::Module(
                                                                    `core::ops`,
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Subitem {
                                                            parent: 1,
                                                            colon_colon_token: ColonColonRegionalToken(
                                                                RegionalTokenIdx(
                                                                    14,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `Unveil`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        15,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Result`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        16,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Result`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        20,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: SynPatternExprRegion {
                                                    pattern_expr_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSynSymbolEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `T1`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                3,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSynSymbolEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    6,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `T2`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                5,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSynSymbolEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    8,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `E1`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                7,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSynSymbolEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    10,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `E2`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                9,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                3..4,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::Trait,
                                                        syn_expr_idx: 6,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 11,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TraitForTypeItem(
                                                                TraitForTypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                                            `<core::result::Result as core::ops::Unveil(0)>::Continue`,
                                                                            TraitItemKind::AssocType,
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [
                                            SynExprData::InheritedSynSymbol {
                                                ident: `E2`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                                inherited_syn_symbol_idx: 3,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E2`,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [],
                                        },
                                        pattern_symbol_maps: [],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [],
                                        },
                                    },
                                    symbol_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        0,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `T1`,
                                                        },
                                                    ),
                                                },
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `T2`,
                                                        },
                                                    ),
                                                },
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        2,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `E1`,
                                                        },
                                                    ),
                                                },
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        3,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `E2`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    syn_pattern_expr_roots: [],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::AssocTypeTerm,
                                            syn_expr_idx: 0,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
        (
            ItemPath::AssocItem(
                AssocItemPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        `<core::result::Result as core::ops::Unveil(0)>::unveil`,
                        TraitItemKind::AssocRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                ),
            ),
            SynDecl::AssocItem(
                AssocItemSynDecl::TraitForTypeItem(
                    TraitForTypeItemSynDecl::AssocFn(
                        TraitForTypeAssocFnSynDecl {
                            path: TraitForTypeItemPath(
                                `<core::result::Result as core::ops::Unveil(0)>::unveil`,
                                TraitItemKind::AssocRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            template_parameters: [],
                            parenate_parameters: [
                                ParenateParameterSyndicate::Simple {
                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                        syn_pattern_expr_idx: 0,
                                    },
                                    variables: ArenaIdxRange(
                                        0..1,
                                    ),
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            6,
                                        ),
                                    ),
                                    ty: 4,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 9,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: SynNodeRegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePath(
                                                                ItemSynNodePathId {
                                                                    data: ItemSynNodePathData::ImplBlock(
                                                                        ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                                                            TraitForTypeImplBlockSynNodePathData {
                                                                                path: TraitForTypeImplBlockPath(`core::result::Result as core::ops::Unveil(0)`),
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                syn_expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::result::Result`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 0,
                                                            argument_expr_idx: 1,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `T2`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                17,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `T2`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            5,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 2,
                                                            argument_expr_idx: 3,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `E2`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                            current_syn_symbol_idx: 3,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `E2`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            9,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 4,
                                                            argument_expr_idx: 5,
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 4,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`core::result::Result`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `T1`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                21,
                                                            ),
                                                            current_syn_symbol_idx: 0,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `T1`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 7,
                                                            argument_expr_idx: 8,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `E1`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                22,
                                                            ),
                                                            current_syn_symbol_idx: 2,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentTemplateParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `E1`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            7,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 9,
                                                            argument_expr_idx: 10,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::CrateRootMod(
                                                                CrateRegionalToken {
                                                                    token_idx: RegionalTokenIdx(
                                                                        11,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::Module(
                                                                `core`,
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Subitem {
                                                            parent: 0,
                                                            colon_colon_token: ColonColonRegionalToken(
                                                                RegionalTokenIdx(
                                                                    12,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `ops`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        13,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::Module(
                                                                    `core::ops`,
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Subitem {
                                                            parent: 1,
                                                            colon_colon_token: ColonColonRegionalToken(
                                                                RegionalTokenIdx(
                                                                    14,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `Unveil`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        15,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Result`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        16,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Result`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        20,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: SynPatternExprRegion {
                                                    pattern_expr_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_symbol_maps: [],
                                                    pattern_symbol_modifiers: ArenaMap {
                                                        data: [],
                                                    },
                                                },
                                                symbol_region: VariableRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSynSymbolEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `T1`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                3,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSynSymbolEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    6,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `T2`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                5,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSynSymbolEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    8,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `E1`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                7,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSynSymbolEntry {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    10,
                                                                ),
                                                                access_end: None,
                                                                data: CurrentSynSymbolData::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `E2`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                9,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                        ),
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                3..4,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::Trait,
                                                        syn_expr_idx: 6,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 11,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssocItem(
                                            AssocItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssocItem(
                                                            AssocItemSynNodePathData::TraitForTypeItem(
                                                                TraitForTypeItemSynNodePathData {
                                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                                        maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                                            `<core::result::Result as core::ops::Unveil(0)>::unveil`,
                                                                            TraitItemKind::AssocRitchie(
                                                                                RitchieItemKind::Fn,
                                                                            ),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                    syn_expr_arena: Arena {
                                        data: [
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::result::Result`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `T2`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `T2`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 0,
                                                argument_expr_idx: 1,
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `E2`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    9,
                                                ),
                                                inherited_syn_symbol_idx: 3,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E2`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::result::Result`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `T1`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    13,
                                                ),
                                                inherited_syn_symbol_idx: 0,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `T1`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 5,
                                                argument_expr_idx: 6,
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `E1`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    14,
                                                ),
                                                inherited_syn_symbol_idx: 2,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E1`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 7,
                                                argument_expr_idx: 8,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `result`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::Module(
                                                    `core::result`,
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Result`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::result::Result`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `Result`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::result::Result`, `Enum`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: SynPatternExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                SynPatternData::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `result`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                Pure,
                                            ],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                SynPatternSymbol::Atom(
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `result`,
                                                    0,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                Pure,
                                            ],
                                        },
                                    },
                                    symbol_region: VariableRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        0,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `T1`,
                                                        },
                                                    ),
                                                },
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `T2`,
                                                        },
                                                    ),
                                                },
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        2,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `E1`,
                                                        },
                                                    ),
                                                },
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        3,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `E2`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbolEntry {
                                                    modifier: Pure,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::SimpleParenateParameter {
                                                        ident: `result`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                SimpleParenateParameter {
                                                    syn_pattern_root: ParenateParameterSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 0,
                                                    },
                                                    ty: 4,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [
                                        SynPatternRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 0,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 9,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [
                                        (
                                            0,
                                            0,
                                        ),
                                    ],
                                },
                            },
                        },
                    ),
                ),
            ),
        ),
    ],
}
```