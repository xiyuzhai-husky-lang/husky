Ok(
    [
        SynDefn::MajorItem(
            MajorItemSynDefn::Type(
                TypeSynDefn::Enum(
                    EnumTypeSynDefn {
                        path: TypePath(`core::result::Result`, `Enum`),
                        decl: EnumTypeSynDecl {
                            path: TypePath(`core::result::Result`, `Enum`),
                            template_parameters: [
                                TemplateSynParameterData {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    data: TemplateParameterSyndicateData::Type {
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
                                    symbol: 2,
                                    data: TemplateParameterSyndicateData::Type {
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
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TypePath(`core::result::Result`, `Enum`),
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
                                    expr_arena: Arena {
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
                                    symbol_region: SynSymbolRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [],
                                        },
                                        current_syn_symbol_arena: Arena {
                                            data: [
                                                CurrentSynSymbol {
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
                                                CurrentSynSymbol {
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
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                TemplateTypeParameter,
                                                ArenaIdxRange(
                                                    2..3,
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
                    },
                ),
            ),
        ),
        SynDefn::ImplBlock(
            ImplBlockSynDecl::TraitForType(
                TraitForTypeImplBlockSynDecl {
                    path: TraitForTypeImplBlock {
                        data: TraitForTypeImplBlockPathData {
                            module_path: `core::result`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                            disambiguator: 0,
                        },
                    },
                    template_parameters: [
                        TemplateSynParameterData {
                            annotated_variance_token: None,
                            symbol: 1,
                            data: TemplateParameterSyndicateData::Type {
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
                            symbol: 2,
                            data: TemplateParameterSyndicateData::Type {
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
                            symbol: 3,
                            data: TemplateParameterSyndicateData::Type {
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
                            symbol: 4,
                            data: TemplateParameterSyndicateData::Type {
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
                        expr: 7,
                    },
                    self_ty_decl: PathLeadingExpr(
                        SelfTypeSyndicate {
                            expr: 12,
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
                                                            path: TraitForTypeImplBlock {
                                                                data: TraitForTypeImplBlockPathData {
                                                                    module_path: `core::result`,
                                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`core::result::Result`, `Enum`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 3,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Trait(
                                                    TraitPath(`core::ops::Unveil`),
                                                ),
                                            ),
                                        ),
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
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 1,
                                        argument_expr_idx: 2,
                                    },
                                    SynExprData::CurrentSynSymbol {
                                        ident: `T2`,
                                        regional_token_idx: RegionalTokenIdx(
                                            17,
                                        ),
                                        current_syn_symbol_idx: 2,
                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
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
                                        function_expr_idx: 3,
                                        argument_expr_idx: 4,
                                    },
                                    SynExprData::CurrentSynSymbol {
                                        ident: `E2`,
                                        regional_token_idx: RegionalTokenIdx(
                                            18,
                                        ),
                                        current_syn_symbol_idx: 4,
                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
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
                                        function_expr_idx: 5,
                                        argument_expr_idx: 6,
                                    },
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 5,
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
                                        current_syn_symbol_idx: 1,
                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
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
                                        function_expr_idx: 8,
                                        argument_expr_idx: 9,
                                    },
                                    SynExprData::CurrentSynSymbol {
                                        ident: `E1`,
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                        current_syn_symbol_idx: 3,
                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
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
                                        function_expr_idx: 10,
                                        argument_expr_idx: 11,
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::CrateRoot(
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
                                        parent: 1,
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
                                        parent: 2,
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
                            symbol_region: SynSymbolRegionData {
                                inherited_syn_symbol_arena: Arena {
                                    data: [],
                                },
                                current_syn_symbol_arena: Arena {
                                    data: [
                                        CurrentSynSymbol {
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
                                        CurrentSynSymbol {
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
                                        CurrentSynSymbol {
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
                                        CurrentSynSymbol {
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
                                    (
                                        TemplateTypeParameter,
                                        ArenaIdxRange(
                                            4..5,
                                        ),
                                    ),
                                ],
                            },
                            syn_pattern_expr_roots: [],
                            syn_expr_roots: [
                                SynExprRoot {
                                    kind: SynExprRootKind::Trait,
                                    syn_expr_idx: 7,
                                },
                                SynExprRoot {
                                    kind: SynExprRootKind::SelfType,
                                    syn_expr_idx: 12,
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
        SynDefn::AssociatedItem(
            AssociatedItemSynDefn::TraitForTypeItem(
                TraitForTypeItemSynDefn::AssociatedType(
                    TraitForTypeAssociatedTypeSynDefn {
                        path: TraitForTypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 157,
                                },
                            ),
                        ),
                        decl: TraitForTypeAssociatedTypeSynDecl {
                            path: TraitForTypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 157,
                                    },
                                ),
                            ),
                            template_parameters: [],
                            ty_term_expr_idx: 1,
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
                                                                                path: TraitForTypeImplBlock {
                                                                                    data: TraitForTypeImplBlockPathData {
                                                                                        module_path: `core::result`,
                                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                                        ty_sketch: TypeSketch::Path(
                                                                                            TypePath(`core::result::Result`, `Enum`),
                                                                                        ),
                                                                                        disambiguator: 0,
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
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
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `T2`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                17,
                                                            ),
                                                            current_syn_symbol_idx: 2,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
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
                                                            function_expr_idx: 3,
                                                            argument_expr_idx: 4,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `E2`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                            current_syn_symbol_idx: 4,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
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
                                                            function_expr_idx: 5,
                                                            argument_expr_idx: 6,
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 5,
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
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
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
                                                            function_expr_idx: 8,
                                                            argument_expr_idx: 9,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `E1`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                22,
                                                            ),
                                                            current_syn_symbol_idx: 3,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
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
                                                            function_expr_idx: 10,
                                                            argument_expr_idx: 11,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::CrateRoot(
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
                                                            parent: 1,
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
                                                            parent: 2,
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
                                                symbol_region: SynSymbolRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSynSymbol {
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
                                                            CurrentSynSymbol {
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
                                                            CurrentSynSymbol {
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
                                                            CurrentSynSymbol {
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
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                4..5,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::Trait,
                                                        syn_expr_idx: 7,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 12,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TraitForTypeItem(
                                                                TraitForTypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TraitForTypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 157,
                                                                                },
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
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::InheritedSynSymbol {
                                                ident: `E2`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                                inherited_syn_symbol_idx: 4,
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
                                    symbol_region: SynSymbolRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
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
                                                        2,
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
                                                        3,
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
                                                        4,
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
                                            kind: SynExprRootKind::AssociatedTypeTerm,
                                            syn_expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [],
                                },
                            },
                        },
                        body_with_syn_expr_region: None,
                    },
                ),
            ),
        ),
        SynDefn::AssociatedItem(
            AssociatedItemSynDefn::TraitForTypeItem(
                TraitForTypeItemSynDefn::MethodFn(
                    TraitForTypeMethodFnSynDefn {
                        path: TraitForTypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 158,
                                },
                            ),
                        ),
                        decl: TraitForTypeMethodFnSynDecl {
                            path: TraitForTypeItemPath(
                                ItemPathId(
                                    Id {
                                        value: 158,
                                    },
                                ),
                            ),
                            template_parameters: [],
                            self_value_parameter: None,
                            parenate_parameters: [
                                ParenateSynParameterData::Ordinary {
                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                        syn_pattern_expr_idx: 1,
                                    },
                                    variables: ArenaIdxRange(
                                        1..2,
                                    ),
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            5,
                                        ),
                                    ),
                                    ty: 5,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 10,
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
                                                                                path: TraitForTypeImplBlock {
                                                                                    data: TraitForTypeImplBlockPathData {
                                                                                        module_path: `core::result`,
                                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                                        ty_sketch: TypeSketch::Path(
                                                                                            TypePath(`core::result::Result`, `Enum`),
                                                                                        ),
                                                                                        disambiguator: 0,
                                                                                    },
                                                                                },
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
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
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `T2`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                17,
                                                            ),
                                                            current_syn_symbol_idx: 2,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
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
                                                            function_expr_idx: 3,
                                                            argument_expr_idx: 4,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `E2`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                            current_syn_symbol_idx: 4,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
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
                                                            function_expr_idx: 5,
                                                            argument_expr_idx: 6,
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 5,
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
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
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
                                                            function_expr_idx: 8,
                                                            argument_expr_idx: 9,
                                                        },
                                                        SynExprData::CurrentSynSymbol {
                                                            ident: `E1`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                22,
                                                            ),
                                                            current_syn_symbol_idx: 3,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
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
                                                            function_expr_idx: 10,
                                                            argument_expr_idx: 11,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::CrateRoot(
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
                                                            parent: 1,
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
                                                            parent: 2,
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
                                                symbol_region: SynSymbolRegionData {
                                                    inherited_syn_symbol_arena: Arena {
                                                        data: [],
                                                    },
                                                    current_syn_symbol_arena: Arena {
                                                        data: [
                                                            CurrentSynSymbol {
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
                                                            CurrentSynSymbol {
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
                                                            CurrentSynSymbol {
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
                                                            CurrentSynSymbol {
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
                                                        (
                                                            TemplateTypeParameter,
                                                            ArenaIdxRange(
                                                                4..5,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                syn_pattern_expr_roots: [],
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::Trait,
                                                        syn_expr_idx: 7,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 12,
                                                    },
                                                ],
                                                has_self_lifetime: false,
                                                has_self_place: false,
                                                syn_pattern_to_current_syn_symbol_map: [],
                                            },
                                        },
                                    ),
                                    path: SynNodeRegionPath::Decl(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath(
                                                    ItemSynNodePathId {
                                                        data: ItemSynNodePathData::AssociatedItem(
                                                            AssociatedItemSynNodePathData::TraitForTypeItem(
                                                                TraitForTypeItemSynNodePathData {
                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                        path: TraitForTypeItemPath(
                                                                            ItemPathId(
                                                                                Id {
                                                                                    value: 158,
                                                                                },
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
                                    expr_arena: Arena {
                                        data: [
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
                                                ident: `T2`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                                inherited_syn_symbol_idx: 2,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `T2`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 1,
                                                argument_expr_idx: 2,
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `E2`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                                inherited_syn_symbol_idx: 4,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E2`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 3,
                                                argument_expr_idx: 4,
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
                                            SynExprData::InheritedSynSymbol {
                                                ident: `T1`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                                inherited_syn_symbol_idx: 1,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `T1`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 6,
                                                argument_expr_idx: 7,
                                            },
                                            SynExprData::InheritedSynSymbol {
                                                ident: `E1`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    13,
                                                ),
                                                inherited_syn_symbol_idx: 3,
                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E1`,
                                                    },
                                                ),
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 8,
                                                argument_expr_idx: 9,
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
                                                            4,
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
                                                            6,
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
                                                            11,
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
                                                SynPatternExpr::Ident {
                                                    symbol_modifier_tokens: None,
                                                    ident_token: IdentRegionalToken {
                                                        ident: `result`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            4,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                None,
                                            ],
                                        },
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                SynPatternSymbol::Atom(
                                                    1,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `result`,
                                                    1,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_modifiers: ArenaMap {
                                            data: [
                                                None,
                                            ],
                                        },
                                    },
                                    symbol_region: SynSymbolRegionData {
                                        inherited_syn_symbol_arena: Arena {
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
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
                                                        2,
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
                                                        3,
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
                                                        4,
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
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                        ident: `result`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                OrdinaryParenateParameter {
                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty_expr_idx: 5,
                                                },
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    syn_pattern_expr_roots: [
                                        SynPatternExprRoot {
                                            kind: SynPatternExprRootKind::Parenate,
                                            syn_pattern_expr_idx: 1,
                                        },
                                    ],
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 5,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 10,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                    syn_pattern_to_current_syn_symbol_map: [
                                        (
                                            1,
                                            1,
                                        ),
                                    ],
                                },
                            },
                        },
                        body_with_syn_expr_region: Some(
                            (
                                2,
                                SynExprRegion {
                                    data: SynExprRegionData {
                                        parent: Some(
                                            SynExprRegion {
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
                                                                                                path: TraitForTypeImplBlock {
                                                                                                    data: TraitForTypeImplBlockPathData {
                                                                                                        module_path: `core::result`,
                                                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                                                        ty_sketch: TypeSketch::Path(
                                                                                                            TypePath(`core::result::Result`, `Enum`),
                                                                                                        ),
                                                                                                        disambiguator: 0,
                                                                                                    },
                                                                                                },
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ),
                                                                ),
                                                                expr_arena: Arena {
                                                                    data: [
                                                                        SynExprData::PrincipalEntityPath {
                                                                            path_expr_idx: 3,
                                                                            opt_path: Some(
                                                                                PrincipalEntityPath::MajorItem(
                                                                                    MajorItemPath::Trait(
                                                                                        TraitPath(`core::ops::Unveil`),
                                                                                    ),
                                                                                ),
                                                                            ),
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
                                                                        SynExprData::ExplicitApplication {
                                                                            function_expr_idx: 1,
                                                                            argument_expr_idx: 2,
                                                                        },
                                                                        SynExprData::CurrentSynSymbol {
                                                                            ident: `T2`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                17,
                                                                            ),
                                                                            current_syn_symbol_idx: 2,
                                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
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
                                                                            function_expr_idx: 3,
                                                                            argument_expr_idx: 4,
                                                                        },
                                                                        SynExprData::CurrentSynSymbol {
                                                                            ident: `E2`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                18,
                                                                            ),
                                                                            current_syn_symbol_idx: 4,
                                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
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
                                                                            function_expr_idx: 5,
                                                                            argument_expr_idx: 6,
                                                                        },
                                                                        SynExprData::PrincipalEntityPath {
                                                                            path_expr_idx: 5,
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
                                                                            current_syn_symbol_idx: 1,
                                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
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
                                                                            function_expr_idx: 8,
                                                                            argument_expr_idx: 9,
                                                                        },
                                                                        SynExprData::CurrentSynSymbol {
                                                                            ident: `E1`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                22,
                                                                            ),
                                                                            current_syn_symbol_idx: 3,
                                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
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
                                                                            function_expr_idx: 10,
                                                                            argument_expr_idx: 11,
                                                                        },
                                                                    ],
                                                                },
                                                                principal_item_path_expr_arena: Arena {
                                                                    data: [
                                                                        SynPrincipalEntityPathExpr::Root {
                                                                            path_name_token: PathNameRegionalToken::CrateRoot(
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
                                                                            parent: 1,
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
                                                                            parent: 2,
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
                                                                symbol_region: SynSymbolRegionData {
                                                                    inherited_syn_symbol_arena: Arena {
                                                                        data: [],
                                                                    },
                                                                    current_syn_symbol_arena: Arena {
                                                                        data: [
                                                                            CurrentSynSymbol {
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
                                                                            CurrentSynSymbol {
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
                                                                            CurrentSynSymbol {
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
                                                                            CurrentSynSymbol {
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
                                                                        (
                                                                            TemplateTypeParameter,
                                                                            ArenaIdxRange(
                                                                                4..5,
                                                                            ),
                                                                        ),
                                                                    ],
                                                                },
                                                                syn_pattern_expr_roots: [],
                                                                syn_expr_roots: [
                                                                    SynExprRoot {
                                                                        kind: SynExprRootKind::Trait,
                                                                        syn_expr_idx: 7,
                                                                    },
                                                                    SynExprRoot {
                                                                        kind: SynExprRootKind::SelfType,
                                                                        syn_expr_idx: 12,
                                                                    },
                                                                ],
                                                                has_self_lifetime: false,
                                                                has_self_place: false,
                                                                syn_pattern_to_current_syn_symbol_map: [],
                                                            },
                                                        },
                                                    ),
                                                    path: SynNodeRegionPath::Decl(
                                                        ItemSynNodePath::AssociatedItem(
                                                            AssociatedItemSynNodePath::TraitForTypeItem(
                                                                TraitForTypeItemSynNodePath(
                                                                    ItemSynNodePathId {
                                                                        data: ItemSynNodePathData::AssociatedItem(
                                                                            AssociatedItemSynNodePathData::TraitForTypeItem(
                                                                                TraitForTypeItemSynNodePathData {
                                                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                                        path: TraitForTypeItemPath(
                                                                                            ItemPathId(
                                                                                                Id {
                                                                                                    value: 158,
                                                                                                },
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
                                                    expr_arena: Arena {
                                                        data: [
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
                                                                ident: `T2`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    7,
                                                                ),
                                                                inherited_syn_symbol_idx: 2,
                                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                                    InheritedTemplateParameterSynSymbol::Type {
                                                                        ident: `T2`,
                                                                    },
                                                                ),
                                                            },
                                                            SynExprData::ExplicitApplication {
                                                                function_expr_idx: 1,
                                                                argument_expr_idx: 2,
                                                            },
                                                            SynExprData::InheritedSynSymbol {
                                                                ident: `E2`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    8,
                                                                ),
                                                                inherited_syn_symbol_idx: 4,
                                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                                    InheritedTemplateParameterSynSymbol::Type {
                                                                        ident: `E2`,
                                                                    },
                                                                ),
                                                            },
                                                            SynExprData::ExplicitApplication {
                                                                function_expr_idx: 3,
                                                                argument_expr_idx: 4,
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
                                                            SynExprData::InheritedSynSymbol {
                                                                ident: `T1`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    12,
                                                                ),
                                                                inherited_syn_symbol_idx: 1,
                                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                                    InheritedTemplateParameterSynSymbol::Type {
                                                                        ident: `T1`,
                                                                    },
                                                                ),
                                                            },
                                                            SynExprData::ExplicitApplication {
                                                                function_expr_idx: 6,
                                                                argument_expr_idx: 7,
                                                            },
                                                            SynExprData::InheritedSynSymbol {
                                                                ident: `E1`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    13,
                                                                ),
                                                                inherited_syn_symbol_idx: 3,
                                                                inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                                    InheritedTemplateParameterSynSymbol::Type {
                                                                        ident: `E1`,
                                                                    },
                                                                ),
                                                            },
                                                            SynExprData::ExplicitApplication {
                                                                function_expr_idx: 8,
                                                                argument_expr_idx: 9,
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
                                                                            4,
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
                                                                            6,
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
                                                                            11,
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
                                                                SynPatternExpr::Ident {
                                                                    symbol_modifier_tokens: None,
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `result`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            4,
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        pattern_expr_contracts: ArenaMap {
                                                            data: [
                                                                None,
                                                            ],
                                                        },
                                                        pattern_symbol_arena: Arena {
                                                            data: [
                                                                SynPatternSymbol::Atom(
                                                                    1,
                                                                ),
                                                            ],
                                                        },
                                                        pattern_symbol_maps: [
                                                            [
                                                                (
                                                                    `result`,
                                                                    1,
                                                                ),
                                                            ],
                                                        ],
                                                        pattern_symbol_modifiers: ArenaMap {
                                                            data: [
                                                                None,
                                                            ],
                                                        },
                                                    },
                                                    symbol_region: SynSymbolRegionData {
                                                        inherited_syn_symbol_arena: Arena {
                                                            data: [
                                                                InheritedSynSymbol {
                                                                    parent_symbol_idx: Current(
                                                                        1,
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
                                                                        2,
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
                                                                        3,
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
                                                                        4,
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
                                                                CurrentSynSymbol {
                                                                    modifier: None,
                                                                    access_start: RegionalTokenIdx(
                                                                        5,
                                                                    ),
                                                                    access_end: None,
                                                                    data: CurrentSynSymbolData::ParenateRegularParameter {
                                                                        ident: `result`,
                                                                        pattern_symbol_idx: 1,
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        allow_self_type: True,
                                                        allow_self_value: True,
                                                        pattern_ty_constraints: [
                                                            (
                                                                OrdinaryParenateParameter {
                                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                                        syn_pattern_expr_idx: 1,
                                                                    },
                                                                    ty_expr_idx: 5,
                                                                },
                                                                ArenaIdxRange(
                                                                    1..2,
                                                                ),
                                                            ),
                                                        ],
                                                    },
                                                    syn_pattern_expr_roots: [
                                                        SynPatternExprRoot {
                                                            kind: SynPatternExprRootKind::Parenate,
                                                            syn_pattern_expr_idx: 1,
                                                        },
                                                    ],
                                                    syn_expr_roots: [
                                                        SynExprRoot {
                                                            kind: SynExprRootKind::ExplicitParameterType,
                                                            syn_expr_idx: 5,
                                                        },
                                                        SynExprRoot {
                                                            kind: SynExprRootKind::ReturnType,
                                                            syn_expr_idx: 10,
                                                        },
                                                    ],
                                                    has_self_lifetime: false,
                                                    has_self_place: false,
                                                    syn_pattern_to_current_syn_symbol_map: [
                                                        (
                                                            1,
                                                            1,
                                                        ),
                                                    ],
                                                },
                                            },
                                        ),
                                        path: SynNodeRegionPath::Defn(
                                            ItemSynNodePath::AssociatedItem(
                                                AssociatedItemSynNodePath::TraitForTypeItem(
                                                    TraitForTypeItemSynNodePath(
                                                        ItemSynNodePathId {
                                                            data: ItemSynNodePathData::AssociatedItem(
                                                                AssociatedItemSynNodePathData::TraitForTypeItem(
                                                                    TraitForTypeItemSynNodePathData {
                                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                            path: TraitForTypeItemPath(
                                                                                ItemPathId(
                                                                                    Id {
                                                                                        value: 158,
                                                                                    },
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
                                        expr_arena: Arena {
                                            data: [
                                                SynExprData::Todo {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        1,
                                                    ),
                                                },
                                                SynExprData::Block {
                                                    stmts: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                },
                                            ],
                                        },
                                        principal_item_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                SynStmtData::Eval {
                                                    expr_idx: 1,
                                                    eol_semicolon: Ok(
                                                        None,
                                                    ),
                                                },
                                            ],
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
                                        symbol_region: SynSymbolRegionData {
                                            inherited_syn_symbol_arena: Arena {
                                                data: [
                                                    InheritedSynSymbol {
                                                        parent_symbol_idx: Current(
                                                            1,
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
                                                            2,
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
                                                            3,
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
                                                            4,
                                                        ),
                                                        modifier: Const,
                                                        kind: InheritedSynSymbolKind::TemplateParameter(
                                                            InheritedTemplateParameterSynSymbol::Type {
                                                                ident: `E2`,
                                                            },
                                                        ),
                                                    },
                                                    InheritedSynSymbol {
                                                        parent_symbol_idx: Current(
                                                            1,
                                                        ),
                                                        modifier: None,
                                                        kind: InheritedSynSymbolKind::ParenateParameter {
                                                            ident: `result`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_syn_symbol_arena: Arena {
                                                data: [],
                                            },
                                            allow_self_type: True,
                                            allow_self_value: True,
                                            pattern_ty_constraints: [],
                                        },
                                        syn_pattern_expr_roots: [],
                                        syn_expr_roots: [
                                            SynExprRoot {
                                                kind: SynExprRootKind::EvalExpr,
                                                syn_expr_idx: 1,
                                            },
                                            SynExprRoot {
                                                kind: SynExprRootKind::BlockExpr,
                                                syn_expr_idx: 2,
                                            },
                                        ],
                                        has_self_lifetime: false,
                                        has_self_place: false,
                                        syn_pattern_to_current_syn_symbol_map: [],
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
    ],
)