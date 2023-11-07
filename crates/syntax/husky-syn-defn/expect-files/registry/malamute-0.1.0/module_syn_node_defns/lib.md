Ok(
    [
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Type(
                TypeSynNodeDefn::Enum(
                    EnumTypeSynNodeDefn {
                        syn_node_path: TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`malamute::Class`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: EnumTypeSynNodeDecl {
                            syn_node_path: TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`malamute::Class`, `Enum`),
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
                                                        ident: `Label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            5,
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
                                                6,
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
                                                        path: TypePath(`malamute::Class`, `Enum`),
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
                                        inherited_symbol_arena: Arena {
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
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `Label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    5,
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
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::Attr(
            AttrSynNodeDefn {
                syn_node_decl: Derive(
                    DeriveAttrSynNodeDecl(
                        Id {
                            value: 19,
                        },
                    ),
                ),
            },
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Type(
                TypeSynNodeDefn::Enum(
                    EnumTypeSynNodeDefn {
                        syn_node_path: TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`malamute::OneVsAll`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: EnumTypeSynNodeDecl {
                            syn_node_path: TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                        ident: `Label`,
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
                                                data: TemplateParameterSyndicateData::Constant {
                                                    const_token: ConstRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                    ident_token: IdentRegionalToken {
                                                        ident: `label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    colon_token: ColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            11,
                                                        ),
                                                    ),
                                                    ty_expr: 1,
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
                                                13,
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
                                                        path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::CurrentSymbol {
                                                ident: `Label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
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
                                        inherited_symbol_arena: Arena {
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
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `Label`,
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
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [
                                                                Phantom(
                                                                    PoundRegionalToken(
                                                                        RegionalTokenIdx(
                                                                            7,
                                                                        ),
                                                                    ),
                                                                    PhantomRegionalToken {
                                                                        token_idx: RegionalTokenIdx(
                                                                            8,
                                                                        ),
                                                                    },
                                                                ),
                                                            ],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    10,
                                                                ),
                                                            },
                                                            ty_expr_idx: 1,
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
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ConstantImplicitParameterType,
                                            syn_expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::Attr(
            AttrSynNodeDefn {
                syn_node_decl: Derive(
                    DeriveAttrSynNodeDecl(
                        Id {
                            value: 20,
                        },
                    ),
                ),
            },
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Type(
                TypeSynNodeDefn::Enum(
                    EnumTypeSynNodeDefn {
                        syn_node_path: TypeSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: EnumTypeSynNodeDecl {
                            syn_node_path: TypeSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                                                        ident: `Label`,
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
                                                data: TemplateParameterSyndicateData::Constant {
                                                    const_token: ConstRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                    ident_token: IdentRegionalToken {
                                                        ident: `label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    colon_token: ColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            11,
                                                        ),
                                                    ),
                                                    ty_expr: 1,
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
                                                13,
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
                                                        path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::CurrentSymbol {
                                                ident: `Label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
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
                                        inherited_symbol_arena: Arena {
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
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `Label`,
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
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [
                                                                Phantom(
                                                                    PoundRegionalToken(
                                                                        RegionalTokenIdx(
                                                                            7,
                                                                        ),
                                                                    ),
                                                                    PhantomRegionalToken {
                                                                        token_idx: RegionalTokenIdx(
                                                                            8,
                                                                        ),
                                                                    },
                                                                ),
                                                            ],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    10,
                                                                ),
                                                            },
                                                            ty_expr_idx: 1,
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
                                    syn_expr_roots: [
                                        SynExprRoot {
                                            kind: SynExprRootKind::ConstantImplicitParameterType,
                                            syn_expr_idx: 1,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        SynNodeDefn::Attr(
            AttrSynNodeDefn {
                syn_node_decl: Derive(
                    DeriveAttrSynNodeDecl(
                        Id {
                            value: 21,
                        },
                    ),
                ),
            },
        ),
        SynNodeDefn::MajorItem(
            MajorItemSynNodeDefn::Fugitive(
                FugitiveSynNodeDefn::Gn(
                    GnSynNodeDefn {
                        syn_node_path: FugitiveSynNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                disambiguator: 0,
                            },
                        },
                        syn_node_decl: GnSynNodeDecl {
                            syn_node_path: FugitiveSynNodePath {
                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                    path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                    disambiguator: 0,
                                },
                            },
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        ItemSynNodePath::MajorItem(
                                            MajorItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExprData::CurrentSymbol {
                                                ident: `Label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    12,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::Literal(
                                                RegionalTokenIdx(
                                                    26,
                                                ),
                                                LiteralData::Integer(
                                                    UnspecifiedRegular(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                            SynExprData::PrincipalEntityPath {
                                                path_expr_idx: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::MajorItem(
                                                        MajorItemPath::Type(
                                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExprData::CurrentSymbol {
                                                ident: `Label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    31,
                                                ),
                                                current_syn_symbol_idx: 1,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                5,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExprData::ExplicitApplication {
                                                function_expr_idx: 5,
                                                argument_expr_idx: 6,
                                            },
                                            SynExprData::CurrentSymbol {
                                                ident: `label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    32,
                                                ),
                                                current_syn_symbol_idx: 2,
                                                current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                    template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                        ident_token: IdentRegionalToken {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
                                                            ),
                                                        },
                                                    },
                                                },
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
                                                        ident: `f32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            20,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `i32`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            24,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            SynPrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameRegionalToken::Ident(
                                                    IdentRegionalToken {
                                                        ident: `OneVsAllResult`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            30,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                                                        ident: `skip`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            22,
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
                                                    `skip`,
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
                                        inherited_symbol_arena: Arena {
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
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `Label`,
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
                                                        13,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [
                                                                Phantom(
                                                                    PoundRegionalToken(
                                                                        RegionalTokenIdx(
                                                                            7,
                                                                        ),
                                                                    ),
                                                                    PhantomRegionalToken {
                                                                        token_idx: RegionalTokenIdx(
                                                                            8,
                                                                        ),
                                                                    },
                                                                ),
                                                            ],
                                                        },
                                                        annotated_variance_token: None,
                                                        template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                            ident_token: IdentRegionalToken {
                                                                ident: `label`,
                                                                regional_token_idx: RegionalTokenIdx(
                                                                    10,
                                                                ),
                                                            },
                                                            ty_expr_idx: 1,
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        16,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateVariadicParameter {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `f`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        23,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ParenateRegularParameter {
                                                        ident: `skip`,
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
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
                                                VariadicParenateParameter {
                                                    ty: 2,
                                                },
                                                ArenaIdxRange(
                                                    3..4,
                                                ),
                                            ),
                                            (
                                                OrdinaryParenateParameter {
                                                    syn_pattern_root: ParenateSynPatternExprRoot {
                                                        syn_pattern_expr_idx: 1,
                                                    },
                                                    ty_expr_idx: 3,
                                                },
                                                ArenaIdxRange(
                                                    4..5,
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
                                            kind: SynExprRootKind::ConstantImplicitParameterType,
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterType,
                                            syn_expr_idx: 3,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ExplicitParameterDefaultValue {
                                                ty_syn_expr_idx: 3,
                                            },
                                            syn_expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: SynExprRootKind::ReturnType,
                                            syn_expr_idx: 9,
                                        },
                                    ],
                                    has_self_lifetime: false,
                                    has_self_place: false,
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
                                                        ident: `Label`,
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
                                                data: TemplateParameterSyndicateData::Constant {
                                                    const_token: ConstRegionalToken {
                                                        regional_token_idx: RegionalTokenIdx(
                                                            9,
                                                        ),
                                                    },
                                                    ident_token: IdentRegionalToken {
                                                        ident: `label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            10,
                                                        ),
                                                    },
                                                    colon_token: ColonRegionalToken(
                                                        RegionalTokenIdx(
                                                            11,
                                                        ),
                                                    ),
                                                    ty_expr: 1,
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
                                                13,
                                            ),
                                        ),
                                    },
                                ),
                            ),
                            parenate_parameter_decl_list: Ok(
                                ParenateParameterSyndicateList {
                                    lpar: LparRegionalToken(
                                        RegionalTokenIdx(
                                            14,
                                        ),
                                    ),
                                    self_value_parameter: None,
                                    comma_after_self_parameter: None,
                                    parenate_parameters: [
                                        ParenateParameterSyndicate::Variadic {
                                            dot_dot_dot_token: DotDotDotRegionalToken(
                                                RegionalTokenIdx(
                                                    15,
                                                ),
                                            ),
                                            variadic_variant: SynVariadicParameterVariant::Vec {
                                                lbox_token: LboxRegionalToken(
                                                    RegionalTokenIdx(
                                                        16,
                                                    ),
                                                ),
                                                rbox_token: RboxRegionalToken(
                                                    RegionalTokenIdx(
                                                        17,
                                                    ),
                                                ),
                                            },
                                            symbol_modifier_keyword_group: None,
                                            ident_token: IdentRegionalToken {
                                                ident: `f`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    18,
                                                ),
                                            },
                                            variable: 3,
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    19,
                                                ),
                                            ),
                                            ty: 2,
                                        },
                                        ParenateParameterSyndicate::Keyed {
                                            syn_pattern_root: ParenateSynPatternExprRoot {
                                                syn_pattern_expr_idx: 1,
                                            },
                                            symbol_modifier_keyword_group: None,
                                            ident_token: IdentRegionalToken {
                                                ident: `skip`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    22,
                                                ),
                                            },
                                            variable: 4,
                                            colon: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    23,
                                                ),
                                            ),
                                            ty: 3,
                                            eq_token: EqRegionalToken(
                                                RegionalTokenIdx(
                                                    25,
                                                ),
                                            ),
                                            default: Right(
                                                4,
                                            ),
                                        },
                                    ],
                                    commas: [
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                21,
                                            ),
                                        ),
                                        CommaRegionalToken(
                                            RegionalTokenIdx(
                                                27,
                                            ),
                                        ),
                                    ],
                                    rpar: RparRegionalToken(
                                        RegionalTokenIdx(
                                            28,
                                        ),
                                    ),
                                },
                            ),
                            light_arrow_token: Ok(
                                Some(
                                    LightArrowRegionalToken(
                                        RegionalTokenIdx(
                                            29,
                                        ),
                                    ),
                                ),
                            ),
                            return_ty: Ok(
                                Some(
                                    ReturnTypeBeforeColonSyndicate {
                                        syn_expr_idx: 9,
                                    },
                                ),
                            ),
                            eol_colon: Ok(
                                EolRegionalToken::Semicolon(
                                    EolSemicolonRegionalToken {
                                        regional_token_idx: RegionalTokenIdx(
                                            33,
                                        ),
                                    },
                                ),
                            ),
                        },
                        body_with_syn_expr_region: None,
                    },
                ),
            ),
        ),
        SynNodeDefn::ImplBlock(
            ImplBlockSynNodeDecl::TraitForType(
                TraitForTypeImplBlockSynNodeDecl {
                    syn_node_path: TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `malamute`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`malamute::Class`, `Enum`),
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
                                                ident: `Label`,
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
                                        data: TemplateParameterSyndicateData::Constant {
                                            const_token: ConstRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                            ident_token: IdentRegionalToken {
                                                ident: `label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                            },
                                            colon_token: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                            ty_expr: 1,
                                        },
                                    },
                                ],
                                commas: [
                                    CommaRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                ],
                                decl_list_result: Ok(
                                    (),
                                ),
                                rangle: RaOrGtRegionalToken(
                                    RegionalTokenIdx(
                                        11,
                                    ),
                                ),
                            },
                        ),
                    ),
                    trai_expr: TraitSyndicate {
                        expr: 8,
                    },
                    for_token: ConnectionForRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            20,
                        ),
                    },
                    self_ty_decl: PathLeadingExpr(
                        SelfTypeSyndicate {
                            expr: 11,
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
                                                module_path: `malamute`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`malamute::Class`, `Enum`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    SynExprData::CurrentSymbol {
                                        ident: `Label`,
                                        regional_token_idx: RegionalTokenIdx(
                                            10,
                                        ),
                                        current_syn_symbol_idx: 1,
                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        3,
                                                    ),
                                                },
                                            },
                                        },
                                    },
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
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 2,
                                        argument_expr_idx: 3,
                                    },
                                    SynExprData::CurrentSymbol {
                                        ident: `Label`,
                                        regional_token_idx: RegionalTokenIdx(
                                            18,
                                        ),
                                        current_syn_symbol_idx: 1,
                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        3,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 4,
                                        argument_expr_idx: 5,
                                    },
                                    SynExprData::CurrentSymbol {
                                        ident: `label`,
                                        regional_token_idx: RegionalTokenIdx(
                                            19,
                                        ),
                                        current_syn_symbol_idx: 2,
                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                ident_token: IdentRegionalToken {
                                                    ident: `label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 6,
                                        argument_expr_idx: 7,
                                    },
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 5,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`malamute::Class`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::CurrentSymbol {
                                        ident: `Label`,
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                        current_syn_symbol_idx: 1,
                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        3,
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
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `core`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    12,
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
                                                13,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentRegionalToken {
                                                ident: `ops`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    14,
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
                                                15,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentRegionalToken {
                                                ident: `Unveil`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    16,
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
                                                ident: `OneVsAll`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    17,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                            ),
                                        ),
                                    },
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `Class`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    21,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`malamute::Class`, `Enum`),
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
                                inherited_symbol_arena: Arena {
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
                                            variant: CurrentSynSymbolVariant::TemplateParameter {
                                                syn_attrs: TemplateParameterSynAttrs {
                                                    syn_attrs: [],
                                                },
                                                annotated_variance_token: None,
                                                template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `Label`,
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
                                                11,
                                            ),
                                            access_end: None,
                                            variant: CurrentSynSymbolVariant::TemplateParameter {
                                                syn_attrs: TemplateParameterSynAttrs {
                                                    syn_attrs: [
                                                        Phantom(
                                                            PoundRegionalToken(
                                                                RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            ),
                                                            PhantomRegionalToken {
                                                                token_idx: RegionalTokenIdx(
                                                                    6,
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                                annotated_variance_token: None,
                                                template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                    ty_expr_idx: 1,
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
                            syn_expr_roots: [
                                SynExprRoot {
                                    kind: SynExprRootKind::ConstantImplicitParameterType,
                                    syn_expr_idx: 1,
                                },
                                SynExprRoot {
                                    kind: SynExprRootKind::Trait,
                                    syn_expr_idx: 8,
                                },
                                SynExprRoot {
                                    kind: SynExprRootKind::SelfType,
                                    syn_expr_idx: 11,
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
                                        module_path: `malamute`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`malamute::Class`, `Enum`),
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
                                            module_path: `malamute`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`malamute::Class`, `Enum`),
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
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePath {
                                                                path: TraitForTypeImplBlockPath {
                                                                    module_path: `malamute`,
                                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`malamute::Class`, `Enum`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::CurrentSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
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
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 2,
                                                            argument_expr_idx: 3,
                                                        },
                                                        SynExprData::CurrentSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 4,
                                                            argument_expr_idx: 5,
                                                        },
                                                        SynExprData::CurrentSymbol {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                19,
                                                            ),
                                                            current_syn_symbol_idx: 2,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            8,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 6,
                                                            argument_expr_idx: 7,
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 5,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::Class`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                22,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
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
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `core`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        12,
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
                                                                    13,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `ops`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        14,
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
                                                                    15,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `Unveil`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        16,
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
                                                                    ident: `OneVsAll`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        17,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `Class`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        21,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::Class`, `Enum`),
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
                                                    inherited_symbol_arena: Arena {
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
                                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `Label`,
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
                                                                    11,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [
                                                                            Phantom(
                                                                                PoundRegionalToken(
                                                                                    RegionalTokenIdx(
                                                                                        5,
                                                                                    ),
                                                                                ),
                                                                                PhantomRegionalToken {
                                                                                    token_idx: RegionalTokenIdx(
                                                                                        6,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                8,
                                                                            ),
                                                                        },
                                                                        ty_expr_idx: 1,
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
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ConstantImplicitParameterType,
                                                        syn_expr_idx: 1,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::Trait,
                                                        syn_expr_idx: 8,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 11,
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
                                                                module_path: `malamute`,
                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                ty_sketch: TypeSketch::Path(
                                                                    TypePath(`malamute::Class`, `Enum`),
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
                                            SynExprData::Unit {
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    5,
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
                                        inherited_symbol_arena: Arena {
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `Label`,
                                                        },
                                                    ),
                                                },
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        2,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Constant {
                                                            ident: `label`,
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
        SynNodeDefn::ImplBlock(
            ImplBlockSynNodeDecl::TraitForType(
                TraitForTypeImplBlockSynNodeDecl {
                    syn_node_path: TraitForTypeImplBlockSynNodePath {
                        path: TraitForTypeImplBlockPath {
                            module_path: `malamute`,
                            trai_path: TraitPath(`core::ops::Unveil`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                ident: `Label`,
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
                                        data: TemplateParameterSyndicateData::Constant {
                                            const_token: ConstRegionalToken {
                                                regional_token_idx: RegionalTokenIdx(
                                                    7,
                                                ),
                                            },
                                            ident_token: IdentRegionalToken {
                                                ident: `label`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    8,
                                                ),
                                            },
                                            colon_token: ColonRegionalToken(
                                                RegionalTokenIdx(
                                                    9,
                                                ),
                                            ),
                                            ty_expr: 1,
                                        },
                                    },
                                ],
                                commas: [
                                    CommaRegionalToken(
                                        RegionalTokenIdx(
                                            4,
                                        ),
                                    ),
                                ],
                                decl_list_result: Ok(
                                    (),
                                ),
                                rangle: RaOrGtRegionalToken(
                                    RegionalTokenIdx(
                                        11,
                                    ),
                                ),
                            },
                        ),
                    ),
                    trai_expr: TraitSyndicate {
                        expr: 8,
                    },
                    for_token: ConnectionForRegionalToken {
                        regional_token_idx: RegionalTokenIdx(
                            20,
                        ),
                    },
                    self_ty_decl: PathLeadingExpr(
                        SelfTypeSyndicate {
                            expr: 13,
                        },
                    ),
                    eol_colon: Ok(
                        EolRegionalToken::Colon(
                            EolColonRegionalToken {
                                regional_token_idx: RegionalTokenIdx(
                                    24,
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
                                                module_path: `malamute`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    SynExprData::CurrentSymbol {
                                        ident: `Label`,
                                        regional_token_idx: RegionalTokenIdx(
                                            10,
                                        ),
                                        current_syn_symbol_idx: 1,
                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        3,
                                                    ),
                                                },
                                            },
                                        },
                                    },
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
                                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 2,
                                        argument_expr_idx: 3,
                                    },
                                    SynExprData::CurrentSymbol {
                                        ident: `Label`,
                                        regional_token_idx: RegionalTokenIdx(
                                            18,
                                        ),
                                        current_syn_symbol_idx: 1,
                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        3,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 4,
                                        argument_expr_idx: 5,
                                    },
                                    SynExprData::CurrentSymbol {
                                        ident: `label`,
                                        regional_token_idx: RegionalTokenIdx(
                                            19,
                                        ),
                                        current_syn_symbol_idx: 2,
                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                ident_token: IdentRegionalToken {
                                                    ident: `label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 6,
                                        argument_expr_idx: 7,
                                    },
                                    SynExprData::PrincipalEntityPath {
                                        path_expr_idx: 5,
                                        opt_path: Some(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExprData::CurrentSymbol {
                                        ident: `Label`,
                                        regional_token_idx: RegionalTokenIdx(
                                            22,
                                        ),
                                        current_syn_symbol_idx: 1,
                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                ident_token: IdentRegionalToken {
                                                    ident: `Label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        3,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 9,
                                        argument_expr_idx: 10,
                                    },
                                    SynExprData::CurrentSymbol {
                                        ident: `label`,
                                        regional_token_idx: RegionalTokenIdx(
                                            23,
                                        ),
                                        current_syn_symbol_idx: 2,
                                        current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                ident_token: IdentRegionalToken {
                                                    ident: `label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        8,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    SynExprData::ExplicitApplication {
                                        function_expr_idx: 11,
                                        argument_expr_idx: 12,
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `core`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    12,
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
                                                13,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentRegionalToken {
                                                ident: `ops`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    14,
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
                                                15,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentRegionalToken {
                                                ident: `Unveil`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    16,
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
                                                ident: `OneVsAllResult`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    17,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                                            ),
                                        ),
                                    },
                                    SynPrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameRegionalToken::Ident(
                                            IdentRegionalToken {
                                                ident: `OneVsAll`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    21,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::MajorItem(
                                            MajorItemPath::Type(
                                                TypePath(`malamute::OneVsAll`, `Enum`),
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
                                inherited_symbol_arena: Arena {
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
                                            variant: CurrentSynSymbolVariant::TemplateParameter {
                                                syn_attrs: TemplateParameterSynAttrs {
                                                    syn_attrs: [],
                                                },
                                                annotated_variance_token: None,
                                                template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `Label`,
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
                                                11,
                                            ),
                                            access_end: None,
                                            variant: CurrentSynSymbolVariant::TemplateParameter {
                                                syn_attrs: TemplateParameterSynAttrs {
                                                    syn_attrs: [
                                                        Phantom(
                                                            PoundRegionalToken(
                                                                RegionalTokenIdx(
                                                                    5,
                                                                ),
                                                            ),
                                                            PhantomRegionalToken {
                                                                token_idx: RegionalTokenIdx(
                                                                    6,
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                },
                                                annotated_variance_token: None,
                                                template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            8,
                                                        ),
                                                    },
                                                    ty_expr_idx: 1,
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
                            syn_expr_roots: [
                                SynExprRoot {
                                    kind: SynExprRootKind::ConstantImplicitParameterType,
                                    syn_expr_idx: 1,
                                },
                                SynExprRoot {
                                    kind: SynExprRootKind::Trait,
                                    syn_expr_idx: 8,
                                },
                                SynExprRoot {
                                    kind: SynExprRootKind::SelfType,
                                    syn_expr_idx: 13,
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
                                        module_path: `malamute`,
                                        trai_path: TraitPath(`core::ops::Unveil`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`malamute::OneVsAll`, `Enum`),
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
                                            module_path: `malamute`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                path: RegionPath::Decl(
                                                    ItemSynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePath {
                                                                path: TraitForTypeImplBlockPath {
                                                                    module_path: `malamute`,
                                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExprData::CurrentSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                10,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
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
                                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 2,
                                                            argument_expr_idx: 3,
                                                        },
                                                        SynExprData::CurrentSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                18,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 4,
                                                            argument_expr_idx: 5,
                                                        },
                                                        SynExprData::CurrentSymbol {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                19,
                                                            ),
                                                            current_syn_symbol_idx: 2,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            8,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 6,
                                                            argument_expr_idx: 7,
                                                        },
                                                        SynExprData::PrincipalEntityPath {
                                                            path_expr_idx: 5,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::MajorItem(
                                                                    MajorItemPath::Type(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExprData::CurrentSymbol {
                                                            ident: `Label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                22,
                                                            ),
                                                            current_syn_symbol_idx: 1,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `Label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            3,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 9,
                                                            argument_expr_idx: 10,
                                                        },
                                                        SynExprData::CurrentSymbol {
                                                            ident: `label`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                23,
                                                            ),
                                                            current_syn_symbol_idx: 2,
                                                            current_syn_symbol_kind: CurrentSynSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            8,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExprData::ExplicitApplication {
                                                            function_expr_idx: 11,
                                                            argument_expr_idx: 12,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `core`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        12,
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
                                                                    13,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `ops`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        14,
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
                                                                    15,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `Unveil`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        16,
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
                                                                    ident: `OneVsAllResult`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        17,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                        SynPrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameRegionalToken::Ident(
                                                                IdentRegionalToken {
                                                                    ident: `OneVsAll`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        21,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::MajorItem(
                                                                MajorItemPath::Type(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                                                    inherited_symbol_arena: Arena {
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
                                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Type {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `Label`,
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
                                                                    11,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSynSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [
                                                                            Phantom(
                                                                                PoundRegionalToken(
                                                                                    RegionalTokenIdx(
                                                                                        5,
                                                                                    ),
                                                                                ),
                                                                                PhantomRegionalToken {
                                                                                    token_idx: RegionalTokenIdx(
                                                                                        6,
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                8,
                                                                            ),
                                                                        },
                                                                        ty_expr_idx: 1,
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
                                                syn_expr_roots: [
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::ConstantImplicitParameterType,
                                                        syn_expr_idx: 1,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::Trait,
                                                        syn_expr_idx: 8,
                                                    },
                                                    SynExprRoot {
                                                        kind: SynExprRootKind::SelfType,
                                                        syn_expr_idx: 13,
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
                                                                module_path: `malamute`,
                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                ty_sketch: TypeSketch::Path(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                                            SynExprData::Unit {
                                                lpar_regional_token_idx: RegionalTokenIdx(
                                                    4,
                                                ),
                                                rpar_regional_token_idx: RegionalTokenIdx(
                                                    5,
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
                                        inherited_symbol_arena: Arena {
                                            data: [
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `Label`,
                                                        },
                                                    ),
                                                },
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        2,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Constant {
                                                            ident: `label`,
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
    ],
)