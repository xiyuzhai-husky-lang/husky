[
    ItemSynNodeDefn::MajorItem(
        MajorItemSynNodeDefn::Type(
            TypeSynNodeDefn::Extern(
                ExternTypeSynNodeDefn {
                    syn_node_path: TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`core::slice::Slice`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: ExternTypeSynNodeDecl {
                        syn_node_path: TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::slice::Slice`, `Extern`),
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
                                        TemplateSynParameterData {
                                            annotated_variance_token: Some(
                                                VarianceRegionalToken::Covariant(
                                                    CovariantRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            symbol: 1,
                                            data: TemplateParameterSyndicateData::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `E`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Type(
                                            TypeSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypePath(`core::slice::Slice`, `Extern`),
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
                                symbol_region: SynSymbolRegionData {
                                    inherited_syn_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_syn_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    7,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: Some(
                                                        VarianceRegionalToken::Covariant(
                                                            CovariantRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `E`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
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
    ItemSynNodeDefn::MajorItem(
        MajorItemSynNodeDefn::Type(
            TypeSynNodeDefn::Extern(
                ExternTypeSynNodeDefn {
                    syn_node_path: TypeSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: ExternTypeSynNodeDecl {
                        syn_node_path: TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                        TemplateSynParameterData {
                                            annotated_variance_token: Some(
                                                VarianceRegionalToken::Covariant(
                                                    CovariantRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
                                                        ),
                                                    },
                                                ),
                                            ),
                                            symbol: 1,
                                            data: TemplateParameterSyndicateData::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `E`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                },
                                                traits: None,
                                            },
                                        },
                                    ],
                                    commas: [],
                                    decl_list_result: Ok(
                                        (),
                                    ),
                                    rangle: RaOrGtRegionalToken(
                                        RegionalTokenIdx(
                                            7,
                                        ),
                                    ),
                                },
                            ),
                        ),
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: None,
                                path: SynNodeRegionPath::Decl(
                                    ItemSynNodePath::MajorItem(
                                        MajorItemSynNodePath::Type(
                                            TypeSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                symbol_region: SynSymbolRegionData {
                                    inherited_syn_symbol_arena: Arena {
                                        data: [],
                                    },
                                    current_syn_symbol_arena: Arena {
                                        data: [
                                            CurrentSynSymbol {
                                                modifier: Const,
                                                access_start: RegionalTokenIdx(
                                                    7,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::TemplateParameter {
                                                    syn_attrs: TemplateParameterSynAttrs {
                                                        syn_attrs: [],
                                                    },
                                                    annotated_variance_token: Some(
                                                        VarianceRegionalToken::Covariant(
                                                            CovariantRegionalToken {
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `E`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                6,
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
    ItemSynNodeDefn::Attr(
        AttrSynNodeDefn {
            syn_node_decl: Derive(
                DeriveAttrSynNodeDecl(
                    Id {
                        value: 16,
                    },
                ),
            ),
        },
    ),
    ItemSynNodeDefn::ImplBlock(
        ImplBlockSynNodeDecl::Type(
            TypeImplBlockSynNodeDecl {
                syn_node_path: TypeImplBlockSynNodePath {
                    path: TypeImplBlockPath {
                        module_path: `core::slice`,
                        ty_path: TypePath(`core::slice::Slice`, `Extern`),
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
                                TemplateSynParameterData {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    data: TemplateParameterSyndicateData::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `E`,
                                            regional_token_idx: RegionalTokenIdx(
                                                3,
                                            ),
                                        },
                                        traits: None,
                                    },
                                },
                            ],
                            commas: [],
                            decl_list_result: Ok(
                                (),
                            ),
                            rangle: RaOrGtRegionalToken(
                                RegionalTokenIdx(
                                    4,
                                ),
                            ),
                        },
                    ),
                ),
                self_ty_expr: SelfTypeSyndicate {
                    expr: 3,
                },
                eol_colon: Ok(
                    EolRegionalToken::Colon(
                        EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                7,
                            ),
                        },
                    ),
                ),
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: None,
                        path: SynNodeRegionPath::Decl(
                            ItemSynNodePath::ImplBlock(
                                ImplBlockSynNodePath::TypeImplBlock(
                                    TypeImplBlockSynNodePath {
                                        path: TypeImplBlockPath {
                                            module_path: `core::slice`,
                                            ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::slice::Slice`, `Extern`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `E`,
                                    regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                            ident_token: IdentRegionalToken {
                                                ident: `E`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    3,
                                                ),
                                            },
                                        },
                                    },
                                },
                                SynExprData::ExplicitApplication {
                                    function_expr_idx: 1,
                                    argument_expr_idx: 2,
                                },
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `Slice`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::slice::Slice`, `Extern`),
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
                                                    ident: `E`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        3,
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
                            ],
                        },
                        syn_pattern_expr_roots: [],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::SelfType,
                                syn_expr_idx: 3,
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
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDataDefn::TypeItem(
            TypeItemSynNodeDefn::MethodFn(
                TypeMethodFnSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `core::slice`,
                                    ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                    disambiguator: 0,
                                },
                                ident: `len`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TypeMethodFnSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::slice`,
                                        ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `len`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        template_parameters: Ok(
                            None,
                        ),
                        parenate_parameters: Ok(
                            ParenateParameterSyndicateList {
                                lpar: LparRegionalToken(
                                    RegionalTokenIdx(
                                        4,
                                    ),
                                ),
                                self_value_parameter: None,
                                comma_after_self_parameter: None,
                                parenate_parameters: [],
                                commas: [],
                                rpar: RparRegionalToken(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                ),
                            },
                        ),
                        light_arrow_token: Ok(
                            Some(
                                LightArrowRegionalToken(
                                    RegionalTokenIdx(
                                        6,
                                    ),
                                ),
                            ),
                        ),
                        return_ty: Ok(
                            Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 1,
                                },
                            ),
                        ),
                        eol_colon: Ok(
                            EolRegionalToken::Semicolon(
                                EolSemicolonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        8,
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath {
                                                            path: TypeImplBlockPath {
                                                                module_path: `core::slice`,
                                                                ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    SynExprData::PrincipalEntityPath {
                                                        path_expr_idx: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::slice::Slice`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExprData::CurrentSynSymbol {
                                                        ident: `E`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                        current_syn_symbol_idx: 1,
                                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `E`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        3,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    SynExprData::ExplicitApplication {
                                                        function_expr_idx: 1,
                                                        argument_expr_idx: 2,
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameRegionalToken::Ident(
                                                            IdentRegionalToken {
                                                                ident: `Slice`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::slice::Slice`, `Extern`),
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
                                                                        ident: `E`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
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
                                                ],
                                            },
                                            syn_pattern_expr_roots: [],
                                            syn_expr_roots: [
                                                SynExprRoot {
                                                    kind: SynExprRootKind::SelfType,
                                                    syn_expr_idx: 3,
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
                                        AssociatedItemSynNodeDataPath::TypeItem(
                                            TypeItemSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `core::slice`,
                                                            ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `len`,
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
                                            path_expr_idx: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `usize`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
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
                                        data: [
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
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
                                        kind: SynExprRootKind::ReturnType,
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
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDataDefn::TypeItem(
            TypeItemSynNodeDefn::MethodFn(
                TypeMethodFnSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `core::slice`,
                                    ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                    disambiguator: 0,
                                },
                                ident: `swap`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TypeMethodFnSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::slice`,
                                        ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `swap`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        template_parameters: Ok(
                            None,
                        ),
                        parenate_parameters: Ok(
                            ParenateParameterSyndicateList {
                                lpar: LparRegionalToken(
                                    RegionalTokenIdx(
                                        4,
                                    ),
                                ),
                                self_value_parameter: Some(
                                    SelfValueParameterSyndicate {
                                        ephem_symbol_modifier_token_group: Some(
                                            AmbersandMut(
                                                AmbersandRegionalToken(
                                                    RegionalTokenIdx(
                                                        5,
                                                    ),
                                                ),
                                                None,
                                                MutRegionalToken {
                                                    regional_token_idx: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                },
                                            ),
                                        ),
                                        self_value_token: SelfValueRegionalToken {
                                            regional_token_idx: RegionalTokenIdx(
                                                7,
                                            ),
                                        },
                                    },
                                ),
                                comma_after_self_parameter: Some(
                                    CommaRegionalToken(
                                        RegionalTokenIdx(
                                            8,
                                        ),
                                    ),
                                ),
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
                                                10,
                                            ),
                                        ),
                                        ty: 1,
                                    },
                                    ParenateSynParameterData::Ordinary {
                                        syn_pattern_root: ParenateSynPatternExprRoot {
                                            syn_pattern_expr_idx: 2,
                                        },
                                        variables: ArenaIdxRange(
                                            2..3,
                                        ),
                                        colon: ColonRegionalToken(
                                            RegionalTokenIdx(
                                                14,
                                            ),
                                        ),
                                        ty: 2,
                                    },
                                ],
                                commas: [
                                    CommaRegionalToken(
                                        RegionalTokenIdx(
                                            12,
                                        ),
                                    ),
                                ],
                                rpar: RparRegionalToken(
                                    RegionalTokenIdx(
                                        16,
                                    ),
                                ),
                            },
                        ),
                        light_arrow_token: Ok(
                            None,
                        ),
                        return_ty: Ok(
                            None,
                        ),
                        eol_colon: Ok(
                            EolRegionalToken::Semicolon(
                                EolSemicolonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        17,
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath {
                                                            path: TypeImplBlockPath {
                                                                module_path: `core::slice`,
                                                                ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    SynExprData::PrincipalEntityPath {
                                                        path_expr_idx: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::slice::Slice`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExprData::CurrentSynSymbol {
                                                        ident: `E`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                        current_syn_symbol_idx: 1,
                                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `E`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        3,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    SynExprData::ExplicitApplication {
                                                        function_expr_idx: 1,
                                                        argument_expr_idx: 2,
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameRegionalToken::Ident(
                                                            IdentRegionalToken {
                                                                ident: `Slice`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::slice::Slice`, `Extern`),
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
                                                                        ident: `E`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
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
                                                ],
                                            },
                                            syn_pattern_expr_roots: [],
                                            syn_expr_roots: [
                                                SynExprRoot {
                                                    kind: SynExprRootKind::SelfType,
                                                    syn_expr_idx: 3,
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
                                        AssociatedItemSynNodeDataPath::TypeItem(
                                            TypeItemSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `core::slice`,
                                                            ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `swap`,
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
                                            path_expr_idx: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        SynExprData::PrincipalEntityPath {
                                            path_expr_idx: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `usize`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
                                                ),
                                            ),
                                        },
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `usize`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        15,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::usize`, `Extern`),
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
                                                    ident: `a`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        9,
                                                    ),
                                                },
                                            },
                                            SynPatternExpr::Ident {
                                                symbol_modifier_tokens: None,
                                                ident_token: IdentRegionalToken {
                                                    ident: `b`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        13,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            None,
                                            None,
                                        ],
                                    },
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            SynPatternSymbol::Atom(
                                                1,
                                            ),
                                            SynPatternSymbol::Atom(
                                                2,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `a`,
                                                1,
                                            ),
                                        ],
                                        [
                                            (
                                                `b`,
                                                2,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            None,
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
                                                        ident: `E`,
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
                                                    10,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::ParenateRegularParameter {
                                                    ident: `a`,
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSynSymbol {
                                                modifier: None,
                                                access_start: RegionalTokenIdx(
                                                    14,
                                                ),
                                                access_end: None,
                                                data: CurrentSynSymbolData::ParenateRegularParameter {
                                                    ident: `b`,
                                                    pattern_symbol_idx: 2,
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
                                                ty_expr_idx: 1,
                                            },
                                            ArenaIdxRange(
                                                1..2,
                                            ),
                                        ),
                                        (
                                            OrdinaryParenateParameter {
                                                syn_pattern_root: ParenateSynPatternExprRoot {
                                                    syn_pattern_expr_idx: 2,
                                                },
                                                ty_expr_idx: 2,
                                            },
                                            ArenaIdxRange(
                                                2..3,
                                            ),
                                        ),
                                    ],
                                },
                                syn_pattern_expr_roots: [
                                    SynPatternExprRoot {
                                        kind: SynPatternExprRootKind::Parenate,
                                        syn_pattern_expr_idx: 1,
                                    },
                                    SynPatternExprRoot {
                                        kind: SynPatternExprRootKind::Parenate,
                                        syn_pattern_expr_idx: 2,
                                    },
                                ],
                                syn_expr_roots: [
                                    SynExprRoot {
                                        kind: SynExprRootKind::ExplicitParameterType,
                                        syn_expr_idx: 1,
                                    },
                                    SynExprRoot {
                                        kind: SynExprRootKind::ExplicitParameterType,
                                        syn_expr_idx: 2,
                                    },
                                ],
                                has_self_lifetime: true,
                                has_self_place: false,
                                syn_pattern_to_current_syn_symbol_map: [
                                    (
                                        1,
                                        1,
                                    ),
                                    (
                                        2,
                                        2,
                                    ),
                                ],
                            },
                        },
                    },
                    body_with_syn_expr_region: None,
                },
            ),
        ),
    ),
    ItemSynNodeDefn::ImplBlock(
        ImplBlockSynNodeDecl::TraitForType(
            TraitForTypeImplBlockSynNodeDecl {
                syn_node_path: TraitForTypeImplBlockSynNodePath {
                    path: TraitForTypeImplBlockPath {
                        module_path: `core::slice`,
                        trai_path: TraitPath(`core::ops::IntIndex`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                TemplateSynParameterData {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    data: TemplateParameterSyndicateData::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `E`,
                                            regional_token_idx: RegionalTokenIdx(
                                                3,
                                            ),
                                        },
                                        traits: None,
                                    },
                                },
                            ],
                            commas: [],
                            decl_list_result: Ok(
                                (),
                            ),
                            rangle: RaOrGtRegionalToken(
                                RegionalTokenIdx(
                                    4,
                                ),
                            ),
                        },
                    ),
                ),
                trai_expr: TraitSyndicate {
                    expr: 1,
                },
                for_token: ConnectionForRegionalToken {
                    regional_token_idx: RegionalTokenIdx(
                        10,
                    ),
                },
                self_ty_decl: PathLeadingExpr(
                    SelfTypeSyndicate {
                        expr: 4,
                    },
                ),
                eol_colon: Ok(
                    EolRegionalToken::Colon(
                        EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                13,
                            ),
                        },
                    ),
                ),
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: None,
                        path: SynNodeRegionPath::Decl(
                            ItemSynNodePath::ImplBlock(
                                ImplBlockSynNodePath::TraitForTypeImplBlock(
                                    TraitForTypeImplBlockSynNodePath {
                                        path: TraitForTypeImplBlockPath {
                                            module_path: `core::slice`,
                                            trai_path: TraitPath(`core::ops::IntIndex`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                                TraitPath(`core::ops::IntIndex`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 4,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::slice::CyclicSlice`, `Extern`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `E`,
                                    regional_token_idx: RegionalTokenIdx(
                                        12,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                            ident_token: IdentRegionalToken {
                                                ident: `E`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    3,
                                                ),
                                            },
                                        },
                                    },
                                },
                                SynExprData::ExplicitApplication {
                                    function_expr_idx: 2,
                                    argument_expr_idx: 3,
                                },
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::CrateRoot(
                                        CrateRegionalToken {
                                            token_idx: RegionalTokenIdx(
                                                5,
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
                                            6,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `ops`,
                                            regional_token_idx: RegionalTokenIdx(
                                                7,
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
                                            8,
                                        ),
                                    ),
                                    ident_token: Ok(
                                        IdentRegionalToken {
                                            ident: `IntIndex`,
                                            regional_token_idx: RegionalTokenIdx(
                                                9,
                                            ),
                                        },
                                    ),
                                    path: Ok(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Trait(
                                                TraitPath(`core::ops::IntIndex`),
                                            ),
                                        ),
                                    ),
                                },
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `CyclicSlice`,
                                            regional_token_idx: RegionalTokenIdx(
                                                11,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                                    ident: `E`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        3,
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
                            ],
                        },
                        syn_pattern_expr_roots: [],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::Trait,
                                syn_expr_idx: 1,
                            },
                            SynExprRoot {
                                kind: SynExprRootKind::SelfType,
                                syn_expr_idx: 4,
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
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDataDefn::TraitForTypeItem(
            TraitForTypeItemSynNodeDefn::AssociatedType(
                TraitForTypeAssociatedTypeSynNodeDefn {
                    syn_node_path: TraitForTypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TraitForTypeItemPath {
                                impl_block: TraitForTypeImplBlockPath {
                                    module_path: `core::slice`,
                                    trai_path: TraitPath(`core::ops::IntIndex`),
                                    ty_sketch: TypeSketch::Path(
                                        TypePath(`core::slice::CyclicSlice`, `Extern`),
                                    ),
                                    disambiguator: 0,
                                },
                                ident: `Output`,
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
                                        module_path: `core::slice`,
                                        trai_path: TraitPath(`core::ops::IntIndex`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`core::slice::CyclicSlice`, `Extern`),
                                        ),
                                        disambiguator: 0,
                                    },
                                    ident: `Output`,
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                        TraitForTypeImplBlockSynNodePath {
                                                            path: TraitForTypeImplBlockPath {
                                                                module_path: `core::slice`,
                                                                trai_path: TraitPath(`core::ops::IntIndex`),
                                                                ty_sketch: TypeSketch::Path(
                                                                    TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                                                    TraitPath(`core::ops::IntIndex`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExprData::PrincipalEntityPath {
                                                        path_expr_idx: 4,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExprData::CurrentSynSymbol {
                                                        ident: `E`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            12,
                                                        ),
                                                        current_syn_symbol_idx: 1,
                                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `E`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        3,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    SynExprData::ExplicitApplication {
                                                        function_expr_idx: 2,
                                                        argument_expr_idx: 3,
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameRegionalToken::CrateRoot(
                                                            CrateRegionalToken {
                                                                token_idx: RegionalTokenIdx(
                                                                    5,
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
                                                                6,
                                                            ),
                                                        ),
                                                        ident_token: Ok(
                                                            IdentRegionalToken {
                                                                ident: `ops`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    7,
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
                                                                8,
                                                            ),
                                                        ),
                                                        ident_token: Ok(
                                                            IdentRegionalToken {
                                                                ident: `IntIndex`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    9,
                                                                ),
                                                            },
                                                        ),
                                                        path: Ok(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Trait(
                                                                    TraitPath(`core::ops::IntIndex`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynPrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameRegionalToken::Ident(
                                                            IdentRegionalToken {
                                                                ident: `CyclicSlice`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    11,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                                                        ident: `E`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
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
                                                ],
                                            },
                                            syn_pattern_expr_roots: [],
                                            syn_expr_roots: [
                                                SynExprRoot {
                                                    kind: SynExprRootKind::Trait,
                                                    syn_expr_idx: 1,
                                                },
                                                SynExprRoot {
                                                    kind: SynExprRootKind::SelfType,
                                                    syn_expr_idx: 4,
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
                                        AssociatedItemSynNodeDataPath::TraitForTypeItem(
                                            TraitForTypeItemSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitForTypeItemPath {
                                                        impl_block: TraitForTypeImplBlockPath {
                                                            module_path: `core::slice`,
                                                            trai_path: TraitPath(`core::ops::IntIndex`),
                                                            ty_sketch: TypeSketch::Path(
                                                                TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                            ),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `Output`,
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
                                            ident: `E`,
                                            regional_token_idx: RegionalTokenIdx(
                                                4,
                                            ),
                                            inherited_syn_symbol_idx: 1,
                                            inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                InheritedTemplateParameterSynSymbol::Type {
                                                    ident: `E`,
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
                                                        ident: `E`,
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
    ItemSynNodeDefn::ImplBlock(
        ImplBlockSynNodeDecl::Type(
            TypeImplBlockSynNodeDecl {
                syn_node_path: TypeImplBlockSynNodePath {
                    path: TypeImplBlockPath {
                        module_path: `core::slice`,
                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                TemplateSynParameterData {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    data: TemplateParameterSyndicateData::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `E`,
                                            regional_token_idx: RegionalTokenIdx(
                                                3,
                                            ),
                                        },
                                        traits: None,
                                    },
                                },
                            ],
                            commas: [],
                            decl_list_result: Ok(
                                (),
                            ),
                            rangle: RaOrGtRegionalToken(
                                RegionalTokenIdx(
                                    4,
                                ),
                            ),
                        },
                    ),
                ),
                self_ty_expr: SelfTypeSyndicate {
                    expr: 3,
                },
                eol_colon: Ok(
                    EolRegionalToken::Colon(
                        EolColonRegionalToken {
                            regional_token_idx: RegionalTokenIdx(
                                7,
                            ),
                        },
                    ),
                ),
                syn_expr_region: SynExprRegion {
                    data: SynExprRegionData {
                        parent: None,
                        path: SynNodeRegionPath::Decl(
                            ItemSynNodePath::ImplBlock(
                                ImplBlockSynNodePath::TypeImplBlock(
                                    TypeImplBlockSynNodePath {
                                        path: TypeImplBlockPath {
                                            module_path: `core::slice`,
                                            ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [
                                SynExprData::PrincipalEntityPath {
                                    path_expr_idx: 1,
                                    opt_path: Some(
                                        PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`core::slice::CyclicSlice`, `Extern`),
                                            ),
                                        ),
                                    ),
                                },
                                SynExprData::CurrentSynSymbol {
                                    ident: `E`,
                                    regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                    current_syn_symbol_idx: 1,
                                    current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                        template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                            ident_token: IdentRegionalToken {
                                                ident: `E`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    3,
                                                ),
                                            },
                                        },
                                    },
                                },
                                SynExprData::ExplicitApplication {
                                    function_expr_idx: 1,
                                    argument_expr_idx: 2,
                                },
                            ],
                        },
                        principal_item_path_expr_arena: Arena {
                            data: [
                                SynPrincipalEntityPathExpr::Root {
                                    path_name_token: PathNameRegionalToken::Ident(
                                        IdentRegionalToken {
                                            ident: `CyclicSlice`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                    ),
                                    principal_entity_path: PrincipalEntityPath::MajorItem(
                                        MajorItemPath::Type(
                                            TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                                    ident: `E`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        3,
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
                            ],
                        },
                        syn_pattern_expr_roots: [],
                        syn_expr_roots: [
                            SynExprRoot {
                                kind: SynExprRootKind::SelfType,
                                syn_expr_idx: 3,
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
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDataDefn::TypeItem(
            TypeItemSynNodeDefn::MethodFn(
                TypeMethodFnSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `core::slice`,
                                    ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                    disambiguator: 0,
                                },
                                ident: `ilen`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TypeMethodFnSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::slice`,
                                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `ilen`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        template_parameters: Ok(
                            None,
                        ),
                        parenate_parameters: Ok(
                            ParenateParameterSyndicateList {
                                lpar: LparRegionalToken(
                                    RegionalTokenIdx(
                                        4,
                                    ),
                                ),
                                self_value_parameter: None,
                                comma_after_self_parameter: None,
                                parenate_parameters: [],
                                commas: [],
                                rpar: RparRegionalToken(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                ),
                            },
                        ),
                        light_arrow_token: Ok(
                            Some(
                                LightArrowRegionalToken(
                                    RegionalTokenIdx(
                                        6,
                                    ),
                                ),
                            ),
                        ),
                        return_ty: Ok(
                            Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 1,
                                },
                            ),
                        ),
                        eol_colon: Ok(
                            EolRegionalToken::Semicolon(
                                EolSemicolonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        8,
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath {
                                                            path: TypeImplBlockPath {
                                                                module_path: `core::slice`,
                                                                ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    SynExprData::PrincipalEntityPath {
                                                        path_expr_idx: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExprData::CurrentSynSymbol {
                                                        ident: `E`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                        current_syn_symbol_idx: 1,
                                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `E`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        3,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    SynExprData::ExplicitApplication {
                                                        function_expr_idx: 1,
                                                        argument_expr_idx: 2,
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameRegionalToken::Ident(
                                                            IdentRegionalToken {
                                                                ident: `CyclicSlice`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                                                        ident: `E`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
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
                                                ],
                                            },
                                            syn_pattern_expr_roots: [],
                                            syn_expr_roots: [
                                                SynExprRoot {
                                                    kind: SynExprRootKind::SelfType,
                                                    syn_expr_idx: 3,
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
                                        AssociatedItemSynNodeDataPath::TypeItem(
                                            TypeItemSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `core::slice`,
                                                            ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `ilen`,
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
                                            path_expr_idx: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `i32`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
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
                                        data: [
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
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
                                        kind: SynExprRootKind::ReturnType,
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
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDataDefn::TypeItem(
            TypeItemSynNodeDefn::MethodFn(
                TypeMethodFnSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `core::slice`,
                                    ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                    disambiguator: 0,
                                },
                                ident: `start`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TypeMethodFnSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::slice`,
                                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `start`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        template_parameters: Ok(
                            None,
                        ),
                        parenate_parameters: Ok(
                            ParenateParameterSyndicateList {
                                lpar: LparRegionalToken(
                                    RegionalTokenIdx(
                                        4,
                                    ),
                                ),
                                self_value_parameter: None,
                                comma_after_self_parameter: None,
                                parenate_parameters: [],
                                commas: [],
                                rpar: RparRegionalToken(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                ),
                            },
                        ),
                        light_arrow_token: Ok(
                            Some(
                                LightArrowRegionalToken(
                                    RegionalTokenIdx(
                                        6,
                                    ),
                                ),
                            ),
                        ),
                        return_ty: Ok(
                            Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 1,
                                },
                            ),
                        ),
                        eol_colon: Ok(
                            EolRegionalToken::Semicolon(
                                EolSemicolonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        8,
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath {
                                                            path: TypeImplBlockPath {
                                                                module_path: `core::slice`,
                                                                ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    SynExprData::PrincipalEntityPath {
                                                        path_expr_idx: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExprData::CurrentSynSymbol {
                                                        ident: `E`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                        current_syn_symbol_idx: 1,
                                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `E`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        3,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    SynExprData::ExplicitApplication {
                                                        function_expr_idx: 1,
                                                        argument_expr_idx: 2,
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameRegionalToken::Ident(
                                                            IdentRegionalToken {
                                                                ident: `CyclicSlice`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                                                        ident: `E`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
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
                                                ],
                                            },
                                            syn_pattern_expr_roots: [],
                                            syn_expr_roots: [
                                                SynExprRoot {
                                                    kind: SynExprRootKind::SelfType,
                                                    syn_expr_idx: 3,
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
                                        AssociatedItemSynNodeDataPath::TypeItem(
                                            TypeItemSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `core::slice`,
                                                            ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `start`,
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
                                            path_expr_idx: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `i32`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
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
                                        data: [
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
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
                                        kind: SynExprRootKind::ReturnType,
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
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDataDefn::TypeItem(
            TypeItemSynNodeDefn::MethodFn(
                TypeMethodFnSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `core::slice`,
                                    ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                    disambiguator: 0,
                                },
                                ident: `end`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TypeMethodFnSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::slice`,
                                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `end`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        template_parameters: Ok(
                            None,
                        ),
                        parenate_parameters: Ok(
                            ParenateParameterSyndicateList {
                                lpar: LparRegionalToken(
                                    RegionalTokenIdx(
                                        4,
                                    ),
                                ),
                                self_value_parameter: None,
                                comma_after_self_parameter: None,
                                parenate_parameters: [],
                                commas: [],
                                rpar: RparRegionalToken(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                ),
                            },
                        ),
                        light_arrow_token: Ok(
                            Some(
                                LightArrowRegionalToken(
                                    RegionalTokenIdx(
                                        6,
                                    ),
                                ),
                            ),
                        ),
                        return_ty: Ok(
                            Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 1,
                                },
                            ),
                        ),
                        eol_colon: Ok(
                            EolRegionalToken::Semicolon(
                                EolSemicolonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        8,
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath {
                                                            path: TypeImplBlockPath {
                                                                module_path: `core::slice`,
                                                                ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    SynExprData::PrincipalEntityPath {
                                                        path_expr_idx: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExprData::CurrentSynSymbol {
                                                        ident: `E`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                        current_syn_symbol_idx: 1,
                                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `E`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        3,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    SynExprData::ExplicitApplication {
                                                        function_expr_idx: 1,
                                                        argument_expr_idx: 2,
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameRegionalToken::Ident(
                                                            IdentRegionalToken {
                                                                ident: `CyclicSlice`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                                                        ident: `E`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
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
                                                ],
                                            },
                                            syn_pattern_expr_roots: [],
                                            syn_expr_roots: [
                                                SynExprRoot {
                                                    kind: SynExprRootKind::SelfType,
                                                    syn_expr_idx: 3,
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
                                        AssociatedItemSynNodeDataPath::TypeItem(
                                            TypeItemSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `core::slice`,
                                                            ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `end`,
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
                                            path_expr_idx: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [
                                        SynPrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameRegionalToken::Ident(
                                                IdentRegionalToken {
                                                    ident: `i32`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        7,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`core::num::i32`, `Extern`),
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
                                        data: [
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::TemplateParameter(
                                                    InheritedTemplateParameterSynSymbol::Type {
                                                        ident: `E`,
                                                    },
                                                ),
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
                                        kind: SynExprRootKind::ReturnType,
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
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDataDefn::TypeItem(
            TypeItemSynNodeDefn::MethodFn(
                TypeMethodFnSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `core::slice`,
                                    ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                    disambiguator: 0,
                                },
                                ident: `first`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TypeMethodFnSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::slice`,
                                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `first`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        template_parameters: Ok(
                            None,
                        ),
                        parenate_parameters: Ok(
                            ParenateParameterSyndicateList {
                                lpar: LparRegionalToken(
                                    RegionalTokenIdx(
                                        4,
                                    ),
                                ),
                                self_value_parameter: None,
                                comma_after_self_parameter: None,
                                parenate_parameters: [],
                                commas: [],
                                rpar: RparRegionalToken(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                ),
                            },
                        ),
                        light_arrow_token: Ok(
                            Some(
                                LightArrowRegionalToken(
                                    RegionalTokenIdx(
                                        6,
                                    ),
                                ),
                            ),
                        ),
                        return_ty: Ok(
                            Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 3,
                                },
                            ),
                        ),
                        eol_colon: Ok(
                            EolRegionalToken::Semicolon(
                                EolSemicolonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        10,
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath {
                                                            path: TypeImplBlockPath {
                                                                module_path: `core::slice`,
                                                                ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    SynExprData::PrincipalEntityPath {
                                                        path_expr_idx: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExprData::CurrentSynSymbol {
                                                        ident: `E`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                        current_syn_symbol_idx: 1,
                                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `E`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        3,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    SynExprData::ExplicitApplication {
                                                        function_expr_idx: 1,
                                                        argument_expr_idx: 2,
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameRegionalToken::Ident(
                                                            IdentRegionalToken {
                                                                ident: `CyclicSlice`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                                                        ident: `E`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
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
                                                ],
                                            },
                                            syn_pattern_expr_roots: [],
                                            syn_expr_roots: [
                                                SynExprRoot {
                                                    kind: SynExprRootKind::SelfType,
                                                    syn_expr_idx: 3,
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
                                        AssociatedItemSynNodeDataPath::TypeItem(
                                            TypeItemSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `core::slice`,
                                                            ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `first`,
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
                                        SynExprData::InheritedSynSymbol {
                                            ident: `E`,
                                            regional_token_idx: RegionalTokenIdx(
                                                9,
                                            ),
                                            inherited_syn_symbol_idx: 1,
                                            inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                InheritedTemplateParameterSynSymbol::Type {
                                                    ident: `E`,
                                                },
                                            ),
                                        },
                                        SynExprData::Prefix {
                                            opr: Tilde,
                                            opr_regional_token_idx: RegionalTokenIdx(
                                                8,
                                            ),
                                            opd: 1,
                                        },
                                        SynExprData::Prefix {
                                            opr: Option,
                                            opr_regional_token_idx: RegionalTokenIdx(
                                                7,
                                            ),
                                            opd: 2,
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
                                                        ident: `E`,
                                                    },
                                                ),
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
                                        kind: SynExprRootKind::ReturnType,
                                        syn_expr_idx: 3,
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
    ItemSynNodeDefn::AssociatedItem(
        AssociatedItemSynNodeDataDefn::TypeItem(
            TypeItemSynNodeDefn::MethodFn(
                TypeMethodFnSynNodeDefn {
                    syn_node_path: TypeItemSynNodePath {
                        maybe_ambiguous_path: MaybeAmbiguousPath {
                            path: TypeItemPath {
                                impl_block: TypeImplBlockPath {
                                    module_path: `core::slice`,
                                    ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                    disambiguator: 0,
                                },
                                ident: `last`,
                                item_kind: MethodFn,
                            },
                            disambiguator: 0,
                        },
                    },
                    syn_node_decl: TypeMethodFnSynNodeDecl {
                        syn_node_path: TypeItemSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypeItemPath {
                                    impl_block: TypeImplBlockPath {
                                        module_path: `core::slice`,
                                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                        disambiguator: 0,
                                    },
                                    ident: `last`,
                                    item_kind: MethodFn,
                                },
                                disambiguator: 0,
                            },
                        },
                        template_parameters: Ok(
                            None,
                        ),
                        parenate_parameters: Ok(
                            ParenateParameterSyndicateList {
                                lpar: LparRegionalToken(
                                    RegionalTokenIdx(
                                        4,
                                    ),
                                ),
                                self_value_parameter: None,
                                comma_after_self_parameter: None,
                                parenate_parameters: [],
                                commas: [],
                                rpar: RparRegionalToken(
                                    RegionalTokenIdx(
                                        5,
                                    ),
                                ),
                            },
                        ),
                        light_arrow_token: Ok(
                            Some(
                                LightArrowRegionalToken(
                                    RegionalTokenIdx(
                                        6,
                                    ),
                                ),
                            ),
                        ),
                        return_ty: Ok(
                            Some(
                                ReturnTypeBeforeColonSyndicate {
                                    syn_expr_idx: 3,
                                },
                            ),
                        ),
                        eol_colon: Ok(
                            EolRegionalToken::Semicolon(
                                EolSemicolonRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        10,
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
                                            path: SynNodeRegionPath::Decl(
                                                ItemSynNodePath::ImplBlock(
                                                    ImplBlockSynNodePath::TypeImplBlock(
                                                        TypeImplBlockSynNodePath {
                                                            path: TypeImplBlockPath {
                                                                module_path: `core::slice`,
                                                                ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    SynExprData::PrincipalEntityPath {
                                                        path_expr_idx: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExprData::CurrentSynSymbol {
                                                        ident: `E`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
                                                        ),
                                                        current_syn_symbol_idx: 1,
                                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                ident_token: IdentRegionalToken {
                                                                    ident: `E`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        3,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    SynExprData::ExplicitApplication {
                                                        function_expr_idx: 1,
                                                        argument_expr_idx: 2,
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    SynPrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameRegionalToken::Ident(
                                                            IdentRegionalToken {
                                                                ident: `CyclicSlice`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                                            MajorItemPath::Type(
                                                                TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                                                        ident: `E`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
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
                                                ],
                                            },
                                            syn_pattern_expr_roots: [],
                                            syn_expr_roots: [
                                                SynExprRoot {
                                                    kind: SynExprRootKind::SelfType,
                                                    syn_expr_idx: 3,
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
                                        AssociatedItemSynNodeDataPath::TypeItem(
                                            TypeItemSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TypeItemPath {
                                                        impl_block: TypeImplBlockPath {
                                                            module_path: `core::slice`,
                                                            ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                            disambiguator: 0,
                                                        },
                                                        ident: `last`,
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
                                        SynExprData::InheritedSynSymbol {
                                            ident: `E`,
                                            regional_token_idx: RegionalTokenIdx(
                                                9,
                                            ),
                                            inherited_syn_symbol_idx: 1,
                                            inherited_syn_symbol_kind: InheritedSynSymbolKind::TemplateParameter(
                                                InheritedTemplateParameterSynSymbol::Type {
                                                    ident: `E`,
                                                },
                                            ),
                                        },
                                        SynExprData::Prefix {
                                            opr: Tilde,
                                            opr_regional_token_idx: RegionalTokenIdx(
                                                8,
                                            ),
                                            opd: 1,
                                        },
                                        SynExprData::Prefix {
                                            opr: Option,
                                            opr_regional_token_idx: RegionalTokenIdx(
                                                7,
                                            ),
                                            opd: 2,
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
                                                        ident: `E`,
                                                    },
                                                ),
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
                                        kind: SynExprRootKind::ReturnType,
                                        syn_expr_idx: 3,
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
]