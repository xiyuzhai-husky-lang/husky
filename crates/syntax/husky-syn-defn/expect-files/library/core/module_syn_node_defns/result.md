[
    SynNodeDefn::MajorItem(
        MajorItemSynNodeDefn::Type(
            TypeSynNodeDefn::Enum(
                EnumTypeSynNodeDefn {
                    syn_node_path: TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`core::result::Result`, `Enum`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: EnumTypeSynNodeDecl {
                        syn_node_path: TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::result::Result`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                        template_parameter_decl_list: Ok(
                            Some(
                                SynTemplateParameterSyndicateList {
                                    langle: LaOrLtRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                    template_parameters: [
                                        TemplateParameterSyndicate {
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
                                        TemplateParameterSyndicate {
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
                                    commas: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                6,
                                            ),
                                        ),
                                    ],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            8,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: RegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Type(
                                            TypeSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypePath(`core::result::Result`, `Enum`),
                                                    disambiguator: 0,
                                                },
                                            },
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
                                symbol_region: SynSymbolRegion {
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
                            },
                        },
                    },
                },
            ),
        ),
    ),
    SynNodeDefn::ImplBlock(
        ImplBlockSynNodeDecl::TraitForType(
            TraitForTypeImplBlockSynNodeDecl {
                syn_node_path: TraitForTypeImplBlockSynNodePath {
                    path: TraitForTypeImplBlockPath {
                        module_path: `core::result`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`core::result::Result`, `Enum`),
                        ),
                        disambiguator: 0,
                    },
                },
                impl_regional_token: ImplRegionalToken {
                    regional_token_idx: RegionalTokenIdx(
                        1,
                    ),
                },
                template_parameter_decl_list: Ok(
                    Some(
                        SynTemplateParameterSyndicateList {
                            langle: LaOrLtRegionalToken(
                                RegionalTokenIdx(
                                    2,
                                ),
                            ),
                            template_parameters: [
                                TemplateParameterSyndicate {
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
                                TemplateParameterSyndicate {
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
                                TemplateParameterSyndicate {
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
                                TemplateParameterSyndicate {
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
                            commas: [
                                CommaRegionalToken(
                                    RegionalTokenIdx(
                                        4,
                                    ),
                                ),
                                CommaRegionalToken(
                                    RegionalTokenIdx(
                                        6,
                                    ),
                                ),
                                CommaRegionalToken(
                                    RegionalTokenIdx(
                                        8,
                                    ),
                                ),
                            ],
                            decl_list_result: Ok(
                                (),
                            ),
                            rangle: RaOrGtRegionalToken(
                                RegionalTokenIdx(
                                    10,
                                ),
                            ),
                        },
                    ),
                ),
                trai_expr: TraitSyndicate {
                    expr: 7,
                },
                for_token: ConnectionForRegionalToken {
                    regional_token_idx: RegionalTokenIdx(
                        19,
                    ),
                },
                self_ty_decl: PathLeadingExpr(
                    SelfTypeSyndicate {
                        expr: 12,
                    },
                ),
                eol_colon: Ok(
                    EolRegionalToken::Colon(
                        EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                23,
                            ),
                        },
                    ),
                ),
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: None,
                        path: RegionPath::Decl(
                            ItemSynNodePath::ImplBlock(
                                ImplBlockSynNodePath::TraitForTypeImplBlock(
                                    TraitForTypeImplBlockSynNodePath {
                                        path: TraitForTypeImplBlockPath {
                                            module_path: `core::result`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`core::result::Result`, `Enum`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
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
                        symbol_region: SynSymbolRegion {
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
                    },
                },
            },
        ),
    ),
    SynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDefn::TraitForTypeItem(
            TraitForTypeItemSynNodeDefn::AssociatedType(
                TraitForTypeAssociatedTypeSynNodeDefn {
                    syn_node_path: TraitForTypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `core::result`,
                                    trai_path: TraitPath(`core::ops::Unveil`),
                                    ty_sketch: TypeSketch::Path(
                                        TypePath(`core::result::Result`, `Enum`),
                                    ),
                                    disambiguator: 0,
                                },
                                ident: `Continue`,
                                item_kind: AssociatedType,
                            },
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitForTypeAssociatedTypeSynNodeDecl {
                        syn_node_path: TraitForTypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::result`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`core::result::Result`, `Enum`),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `Continue`,
                                    item_kind: AssociatedType,
                                },
                                disambiguator: 0,
                            },
                        },
                        generics: Ok(
                            None,
                        ),
                        eq_token: Ok(
                            EqRegionalToken(
                                RegionalTokenIdx(
                                    3,
                                ),
                            ),
                        ),
                        ty_term_expr_idx: 1,
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: SynExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                        TraitForTypeImplBlockSynNodePath {
                                                            path: TraitForTypeImplBlockPath {
                                                                module_path: `core::result`,
                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                ty_sketch: TypeSketch::Path(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                                disambiguator: 0,
                                                            },
                                                        },
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
                                            symbol_region: SynSymbolRegion {
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
                                        },
                                    },
                                ),
                                path: RegionPath::Decl(
                                    ItemSynNodePath::AssociatedItem(
                                        AssociatedItemSynNodePath::TraitForTypeItem(
                                            TraitForTypeItemSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitForTypeItemPath {
                                                        impl_block: TraitForTypeImplBlockPath {
                                                            module_path: `core::result`,
                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                            ty_sketch: TypeSketch::Path(
                                                                TypePath(`core::result::Result`, `Enum`),
                                                            ),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `Continue`,
                                                        item_kind: AssociatedType,
                                                    },
                                                    disambiguator: 0,
                                                },
                                            },
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
                                symbol_region: SynSymbolRegion {
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
                            },
                        },
                    },
                    body_with_syn_expr_region: None,
                },
            ),
        ),
    ),
    SynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDefn::TraitForTypeItem(
            TraitForTypeItemSynNodeDefn::MethodFn(
                TraitForTypeMethodFnSynNodeDefn {
                    syn_node_path: TraitForTypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `core::result`,
                                    trai_path: TraitPath(`core::ops::Unveil`),
                                    ty_sketch: TypeSketch::Path(
                                        TypePath(`core::result::Result`, `Enum`),
                                    ),
                                    disambiguator: 0,
                                },
                                ident: `branch`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TraitForTypeMethodFnSynNodeDecl {
                        syn_node_path: TraitForTypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TraitForTypeItemPath {
                                    impl_block: TraitForTypeImplBlockPath {
                                        module_path: `core::result`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`core::result::Result`, `Enum`),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `branch`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        template_parameter_decl_list: Ok(
                            None,
                        ),
                        parenate_parameter_decl_list: Ok(
                            ParenateParameterSyndicateList {
                                lpar: LparRegionalToken(
                                    RegionalTokenIdx(
                                        3,
                                    ),
                                ),
                                self_value_parameter: None,
                                comma_after_self_parameter: None,
                                parenate_parameters: [
                                    ParenateParameterSyndicate::Ordinary {
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
                                commas: [],
                                rpar: RparRegionalToken(
                                    RegionalTokenIdx(
                                        9,
                                    ),
                                ),
                            },
                        ),
                        light_arrow_token: Ok(
                            Some(
                                LightArrowRegionalToken(
                                    RegionalTokenIdx(
                                        10,
                                    ),
                                ),
                            ),
                        ),
                        return_ty: Ok(
                            Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 10,
                                },
                            ),
                        ),
                        eol_colon: Ok(
                            EolRegionalToken::Colon(
                                EolColonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        14,
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: SynExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                        TraitForTypeImplBlockSynNodePath {
                                                            path: TraitForTypeImplBlockPath {
                                                                module_path: `core::result`,
                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                ty_sketch: TypeSketch::Path(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                                disambiguator: 0,
                                                            },
                                                        },
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
                                            symbol_region: SynSymbolRegion {
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
                                        },
                                    },
                                ),
                                path: RegionPath::Decl(
                                    ItemSynNodePath::AssociatedItem(
                                        AssociatedItemSynNodePath::TraitForTypeItem(
                                            TraitForTypeItemSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitForTypeItemPath {
                                                        impl_block: TraitForTypeImplBlockPath {
                                                            module_path: `core::result`,
                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                            ty_sketch: TypeSketch::Path(
                                                                TypePath(`core::result::Result`, `Enum`),
                                                            ),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `branch`,
                                                        item_kind: MethodFn,
                                                    },
                                                    disambiguator: 0,
                                                },
                                            },
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
                                symbol_region: SynSymbolRegion {
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
                                                            path: RegionPath::Decl(
                                                                ItemSynNodePath::ImplBlock(
                                                                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                                        TraitForTypeImplBlockSynNodePath {
                                                                            path: TraitForTypeImplBlockPath {
                                                                                module_path: `core::result`,
                                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                                ty_sketch: TypeSketch::Path(
                                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                                ),
                                                                                disambiguator: 0,
                                                                            },
                                                                        },
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
                                                            symbol_region: SynSymbolRegion {
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
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::AssociatedItem(
                                                        AssociatedItemSynNodePath::TraitForTypeItem(
                                                            TraitForTypeItemSynNodePath {
                                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                    path: TraitForTypeItemPath {
                                                                        impl_block: TraitForTypeImplBlockPath {
                                                                            module_path: `core::result`,
                                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                                            ty_sketch: TypeSketch::Path(
                                                                                TypePath(`core::result::Result`, `Enum`),
                                                                            ),
                                                                            disambiguator: 0,
                                                                        },
                                                                        ident: `branch`,
                                                                        item_kind: MethodFn,
                                                                    },
                                                                    disambiguator: 0,
                                                                },
                                                            },
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
                                                symbol_region: SynSymbolRegion {
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
                                            },
                                        },
                                    ),
                                    path: RegionPath::Defn(
                                        ItemSynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TraitForTypeItemPath {
                                                            impl_block: TraitForTypeImplBlockPath {
                                                                module_path: `core::result`,
                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                ty_sketch: TypeSketch::Path(
                                                                    TypePath(`core::result::Result`, `Enum`),
                                                                ),
                                                                disambiguator: 0,
                                                            },
                                                            ident: `branch`,
                                                            item_kind: MethodFn,
                                                        },
                                                        disambiguator: 0,
                                                    },
                                                },
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
                                    symbol_region: SynSymbolRegion {
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
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]