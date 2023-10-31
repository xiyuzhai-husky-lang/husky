Ok(
    [
        SynDefn::MajorItem(
            MajorItemSynDefn::Type(
                TypeSynDefn::Enum(
                    EnumTypeSynDefn {
                        path: TypePath(`malamute::Class`, `Enum`),
                        decl: EnumTypeSynDecl {
                            path: TypePath(`malamute::Class`, `Enum`),
                            template_parameters: [
                                TemplateParameterObelisk {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    data: TemplateParameterObeliskData::Type {
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
                                        current_symbol_arena: Arena {
                                            data: [
                                                SynCurrentSymbol {
                                                    modifier: Const,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    variant: SynCurrentSymbolVariant::TemplateParameter {
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
                                    roots: [],
                                    has_self_lifetime: false,
                                    has_self_place: false,
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        SynDefn::MajorItem(
            MajorItemSynDefn::Type(
                TypeSynDefn::Enum(
                    EnumTypeSynDefn {
                        path: TypePath(`malamute::OneVsAll`, `Enum`),
                        decl: EnumTypeSynDecl {
                            path: TypePath(`malamute::OneVsAll`, `Enum`),
                            template_parameters: [
                                TemplateParameterObelisk {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    data: TemplateParameterObeliskData::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Label`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                        traits: None,
                                    },
                                },
                                TemplateParameterObelisk {
                                    annotated_variance_token: None,
                                    symbol: 2,
                                    data: TemplateParameterObeliskData::Constant {
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
                                                    10,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                        current_symbol_arena: Arena {
                                            data: [
                                                SynCurrentSymbol {
                                                    modifier: Const,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    variant: SynCurrentSymbolVariant::TemplateParameter {
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
                                                SynCurrentSymbol {
                                                    modifier: Const,
                                                    access_start: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    access_end: None,
                                                    variant: SynCurrentSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
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
                                    roots: [
                                        SynExprRoot {
                                            kind: ConstantImplicitParameterType,
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Type(
                TypeSynDefn::Enum(
                    EnumTypeSynDefn {
                        path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                        decl: EnumTypeSynDecl {
                            path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                            template_parameters: [
                                TemplateParameterObelisk {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    data: TemplateParameterObeliskData::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Label`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                        traits: None,
                                    },
                                },
                                TemplateParameterObelisk {
                                    annotated_variance_token: None,
                                    symbol: 2,
                                    data: TemplateParameterObeliskData::Constant {
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
                                                    10,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                        current_symbol_arena: Arena {
                                            data: [
                                                SynCurrentSymbol {
                                                    modifier: Const,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    variant: SynCurrentSymbolVariant::TemplateParameter {
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
                                                SynCurrentSymbol {
                                                    modifier: Const,
                                                    access_start: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    access_end: None,
                                                    variant: SynCurrentSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
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
                                    roots: [
                                        SynExprRoot {
                                            kind: ConstantImplicitParameterType,
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
        SynDefn::MajorItem(
            MajorItemSynDefn::Fugitive(
                FugitiveSynDefn::FunctionGn(
                    GnSynDefn {
                        path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                        decl: GnSynDecl {
                            path: FugitivePath(`malamute::narrow_down`, `FunctionGn`),
                            template_parameters: [
                                TemplateParameterObelisk {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    data: TemplateParameterObeliskData::Type {
                                        ident_token: IdentRegionalToken {
                                            ident: `Label`,
                                            regional_token_idx: RegionalTokenIdx(
                                                5,
                                            ),
                                        },
                                        traits: None,
                                    },
                                },
                                TemplateParameterObelisk {
                                    annotated_variance_token: None,
                                    symbol: 2,
                                    data: TemplateParameterObeliskData::Constant {
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
                            parenate_parameters: [
                                SpecificParameterObelisk::Variadic {
                                    dot_dot_dot_token: DotDotDotRegionalToken(
                                        RegionalTokenIdx(
                                            13,
                                        ),
                                    ),
                                    variadic_variant: VariadicVariant::Vec {
                                        lbox_token: LboxRegionalToken(
                                            RegionalTokenIdx(
                                                14,
                                            ),
                                        ),
                                        rbox_token: RboxRegionalToken(
                                            RegionalTokenIdx(
                                                15,
                                            ),
                                        ),
                                    },
                                    symbol_modifier_keyword_group: None,
                                    ident_token: IdentRegionalToken {
                                        ident: `f`,
                                        regional_token_idx: RegionalTokenIdx(
                                            16,
                                        ),
                                    },
                                    variable: 3,
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            17,
                                        ),
                                    ),
                                    ty: 2,
                                },
                                SpecificParameterObelisk::Keyed {
                                    syn_pattern_root: SynPatternRoot(
                                        1,
                                    ),
                                    symbol_modifier_keyword_group: None,
                                    ident_token: IdentRegionalToken {
                                        ident: `skip`,
                                        regional_token_idx: RegionalTokenIdx(
                                            20,
                                        ),
                                    },
                                    variable: 4,
                                    colon: ColonRegionalToken(
                                        RegionalTokenIdx(
                                            21,
                                        ),
                                    ),
                                    ty: 3,
                                    eq_token: EqRegionalToken(
                                        RegionalTokenIdx(
                                            23,
                                        ),
                                    ),
                                    default: Right(
                                        4,
                                    ),
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeBeforeColonObelisk {
                                    expr: 9,
                                },
                            ),
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
                                                    10,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                                    24,
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
                                                    29,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                                    30,
                                                ),
                                                current_symbol_idx: 2,
                                                current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                                            18,
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
                                                            22,
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
                                                            28,
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
                                                            20,
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
                                        current_symbol_arena: Arena {
                                            data: [
                                                SynCurrentSymbol {
                                                    modifier: Const,
                                                    access_start: RegionalTokenIdx(
                                                        6,
                                                    ),
                                                    access_end: None,
                                                    variant: SynCurrentSymbolVariant::TemplateParameter {
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
                                                SynCurrentSymbol {
                                                    modifier: Const,
                                                    access_start: RegionalTokenIdx(
                                                        11,
                                                    ),
                                                    access_end: None,
                                                    variant: SynCurrentSymbolVariant::TemplateParameter {
                                                        syn_attrs: TemplateParameterSynAttrs {
                                                            syn_attrs: [],
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
                                                SynCurrentSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        14,
                                                    ),
                                                    access_end: None,
                                                    variant: SynCurrentSymbolVariant::ParenateVariadicParameter {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentRegionalToken {
                                                            ident: `f`,
                                                            regional_token_idx: RegionalTokenIdx(
                                                                16,
                                                            ),
                                                        },
                                                    },
                                                },
                                                SynCurrentSymbol {
                                                    modifier: None,
                                                    access_start: RegionalTokenIdx(
                                                        21,
                                                    ),
                                                    access_end: None,
                                                    variant: SynCurrentSymbolVariant::ParenateRegularParameter {
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
                                                ExplicitVariadicParameter {
                                                    ty: 2,
                                                },
                                                ArenaIdxRange(
                                                    3..4,
                                                ),
                                            ),
                                            (
                                                ExplicitRegularParameter {
                                                    syn_pattern_root: SynPatternRoot(
                                                        1,
                                                    ),
                                                    ty_expr_idx: 3,
                                                },
                                                ArenaIdxRange(
                                                    4..5,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ConstantImplicitParameterType,
                                            syn_expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            syn_expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            syn_expr_idx: 3,
                                        },
                                        SynExprRoot {
                                            kind: ExplicitParameterDefaultValue {
                                                ty_syn_expr_idx: 3,
                                            },
                                            syn_expr_idx: 4,
                                        },
                                        SynExprRoot {
                                            kind: ReturnType,
                                            syn_expr_idx: 9,
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
        SynDefn::ImplBlock(
            ImplBlockSynDecl::TraitForType(
                TraitForTypeImplBlockSynDecl {
                    path: TraitForTypeImplBlockPath {
                        module_path: `malamute`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`malamute::Class`, `Enum`),
                        ),
                        disambiguator: 0,
                    },
                    template_parameters: [
                        TemplateParameterObelisk {
                            annotated_variance_token: None,
                            symbol: 1,
                            data: TemplateParameterObeliskData::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `Label`,
                                    regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                },
                                traits: None,
                            },
                        },
                        TemplateParameterObelisk {
                            annotated_variance_token: None,
                            symbol: 2,
                            data: TemplateParameterObeliskData::Constant {
                                const_token: ConstRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                },
                                ident_token: IdentRegionalToken {
                                    ident: `label`,
                                    regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                },
                                colon_token: ColonRegionalToken(
                                    RegionalTokenIdx(
                                        7,
                                    ),
                                ),
                                ty_expr: 1,
                            },
                        },
                    ],
                    trai_expr: TraitObelisk {
                        expr: 8,
                    },
                    self_ty_decl: PathLeadingExpr(
                        SelfTypeObelisk {
                            expr: 11,
                        },
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
                                            8,
                                        ),
                                        current_symbol_idx: 1,
                                        current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                            16,
                                        ),
                                        current_symbol_idx: 1,
                                        current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                            17,
                                        ),
                                        current_symbol_idx: 2,
                                        current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                ident_token: IdentRegionalToken {
                                                    ident: `label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        6,
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
                                            20,
                                        ),
                                        current_symbol_idx: 1,
                                        current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                                    10,
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
                                                11,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentRegionalToken {
                                                ident: `ops`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    12,
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
                                                13,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentRegionalToken {
                                                ident: `Unveil`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    14,
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
                                                    15,
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
                                                    19,
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
                                current_symbol_arena: Arena {
                                    data: [
                                        SynCurrentSymbol {
                                            modifier: Const,
                                            access_start: RegionalTokenIdx(
                                                4,
                                            ),
                                            access_end: None,
                                            variant: SynCurrentSymbolVariant::TemplateParameter {
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
                                        SynCurrentSymbol {
                                            modifier: Const,
                                            access_start: RegionalTokenIdx(
                                                9,
                                            ),
                                            access_end: None,
                                            variant: SynCurrentSymbolVariant::TemplateParameter {
                                                syn_attrs: TemplateParameterSynAttrs {
                                                    syn_attrs: [],
                                                },
                                                annotated_variance_token: None,
                                                template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
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
                            roots: [
                                SynExprRoot {
                                    kind: ConstantImplicitParameterType,
                                    syn_expr_idx: 1,
                                },
                                SynExprRoot {
                                    kind: Trait,
                                    syn_expr_idx: 8,
                                },
                                SynExprRoot {
                                    kind: SelfType,
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
        SynDefn::AssociatedItem(
            AssociatedItemSynDefn::TraitForTypeItem(
                TraitForTypeItemSynDefn::AssociatedType(
                    TraitForTypeAssociatedTypeSynDefn {
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
                        decl: TraitForTypeAssociatedTypeSynDecl {
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
                            template_parameters: [],
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
                                                                8,
                                                            ),
                                                            current_symbol_idx: 1,
                                                            current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                                                16,
                                                            ),
                                                            current_symbol_idx: 1,
                                                            current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                                                17,
                                                            ),
                                                            current_symbol_idx: 2,
                                                            current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            6,
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
                                                                20,
                                                            ),
                                                            current_symbol_idx: 1,
                                                            current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                                                        10,
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
                                                                    11,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `ops`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        12,
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
                                                                    13,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `Unveil`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        14,
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
                                                                        15,
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
                                                                        19,
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
                                                    current_symbol_arena: Arena {
                                                        data: [
                                                            SynCurrentSymbol {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                                access_end: None,
                                                                variant: SynCurrentSymbolVariant::TemplateParameter {
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
                                                            SynCurrentSymbol {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    9,
                                                                ),
                                                                access_end: None,
                                                                variant: SynCurrentSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                6,
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
                                                roots: [
                                                    SynExprRoot {
                                                        kind: ConstantImplicitParameterType,
                                                        syn_expr_idx: 1,
                                                    },
                                                    SynExprRoot {
                                                        kind: Trait,
                                                        syn_expr_idx: 8,
                                                    },
                                                    SynExprRoot {
                                                        kind: SelfType,
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
                                                SynInheritedSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: SynInheritedSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `Label`,
                                                        },
                                                    ),
                                                },
                                                SynInheritedSymbol {
                                                    parent_symbol_idx: Current(
                                                        2,
                                                    ),
                                                    modifier: Const,
                                                    kind: SynInheritedSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Constant {
                                                            ident: `label`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: AssociatedTypeTerm,
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
        SynDefn::ImplBlock(
            ImplBlockSynDecl::TraitForType(
                TraitForTypeImplBlockSynDecl {
                    path: TraitForTypeImplBlockPath {
                        module_path: `malamute`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`malamute::OneVsAll`, `Enum`),
                        ),
                        disambiguator: 0,
                    },
                    template_parameters: [
                        TemplateParameterObelisk {
                            annotated_variance_token: None,
                            symbol: 1,
                            data: TemplateParameterObeliskData::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `Label`,
                                    regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                },
                                traits: None,
                            },
                        },
                        TemplateParameterObelisk {
                            annotated_variance_token: None,
                            symbol: 2,
                            data: TemplateParameterObeliskData::Constant {
                                const_token: ConstRegionalToken {
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                },
                                ident_token: IdentRegionalToken {
                                    ident: `label`,
                                    regional_token_idx: RegionalTokenIdx(
                                        6,
                                    ),
                                },
                                colon_token: ColonRegionalToken(
                                    RegionalTokenIdx(
                                        7,
                                    ),
                                ),
                                ty_expr: 1,
                            },
                        },
                    ],
                    trai_expr: TraitObelisk {
                        expr: 8,
                    },
                    self_ty_decl: PathLeadingExpr(
                        SelfTypeObelisk {
                            expr: 13,
                        },
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
                                            8,
                                        ),
                                        current_symbol_idx: 1,
                                        current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                            16,
                                        ),
                                        current_symbol_idx: 1,
                                        current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                            17,
                                        ),
                                        current_symbol_idx: 2,
                                        current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                ident_token: IdentRegionalToken {
                                                    ident: `label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        6,
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
                                            20,
                                        ),
                                        current_symbol_idx: 1,
                                        current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                            21,
                                        ),
                                        current_symbol_idx: 2,
                                        current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
                                            template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                ident_token: IdentRegionalToken {
                                                    ident: `label`,
                                                    regional_token_idx: RegionalTokenIdx(
                                                        6,
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
                                                    10,
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
                                                11,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentRegionalToken {
                                                ident: `ops`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    12,
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
                                                13,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentRegionalToken {
                                                ident: `Unveil`,
                                                regional_token_idx: RegionalTokenIdx(
                                                    14,
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
                                                    15,
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
                                                    19,
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
                                current_symbol_arena: Arena {
                                    data: [
                                        SynCurrentSymbol {
                                            modifier: Const,
                                            access_start: RegionalTokenIdx(
                                                4,
                                            ),
                                            access_end: None,
                                            variant: SynCurrentSymbolVariant::TemplateParameter {
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
                                        SynCurrentSymbol {
                                            modifier: Const,
                                            access_start: RegionalTokenIdx(
                                                9,
                                            ),
                                            access_end: None,
                                            variant: SynCurrentSymbolVariant::TemplateParameter {
                                                syn_attrs: TemplateParameterSynAttrs {
                                                    syn_attrs: [],
                                                },
                                                annotated_variance_token: None,
                                                template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                    ident_token: IdentRegionalToken {
                                                        ident: `label`,
                                                        regional_token_idx: RegionalTokenIdx(
                                                            6,
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
                            roots: [
                                SynExprRoot {
                                    kind: ConstantImplicitParameterType,
                                    syn_expr_idx: 1,
                                },
                                SynExprRoot {
                                    kind: Trait,
                                    syn_expr_idx: 8,
                                },
                                SynExprRoot {
                                    kind: SelfType,
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
        SynDefn::AssociatedItem(
            AssociatedItemSynDefn::TraitForTypeItem(
                TraitForTypeItemSynDefn::AssociatedType(
                    TraitForTypeAssociatedTypeSynDefn {
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
                        decl: TraitForTypeAssociatedTypeSynDecl {
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
                            template_parameters: [],
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
                                                                8,
                                                            ),
                                                            current_symbol_idx: 1,
                                                            current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                                                16,
                                                            ),
                                                            current_symbol_idx: 1,
                                                            current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                                                17,
                                                            ),
                                                            current_symbol_idx: 2,
                                                            current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            6,
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
                                                                20,
                                                            ),
                                                            current_symbol_idx: 1,
                                                            current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
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
                                                                21,
                                                            ),
                                                            current_symbol_idx: 2,
                                                            current_symbol_kind: SynCurrentSymbolKind::TemplateParameter {
                                                                template_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                                    ident_token: IdentRegionalToken {
                                                                        ident: `label`,
                                                                        regional_token_idx: RegionalTokenIdx(
                                                                            6,
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
                                                                        10,
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
                                                                    11,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `ops`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        12,
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
                                                                    13,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentRegionalToken {
                                                                    ident: `Unveil`,
                                                                    regional_token_idx: RegionalTokenIdx(
                                                                        14,
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
                                                                        15,
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
                                                                        19,
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
                                                    current_symbol_arena: Arena {
                                                        data: [
                                                            SynCurrentSymbol {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    4,
                                                                ),
                                                                access_end: None,
                                                                variant: SynCurrentSymbolVariant::TemplateParameter {
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
                                                            SynCurrentSymbol {
                                                                modifier: Const,
                                                                access_start: RegionalTokenIdx(
                                                                    9,
                                                                ),
                                                                access_end: None,
                                                                variant: SynCurrentSymbolVariant::TemplateParameter {
                                                                    syn_attrs: TemplateParameterSynAttrs {
                                                                        syn_attrs: [],
                                                                    },
                                                                    annotated_variance_token: None,
                                                                    template_parameter_variant: CurrentTemplateParameterSynSymbolVariant::Constant {
                                                                        ident_token: IdentRegionalToken {
                                                                            ident: `label`,
                                                                            regional_token_idx: RegionalTokenIdx(
                                                                                6,
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
                                                roots: [
                                                    SynExprRoot {
                                                        kind: ConstantImplicitParameterType,
                                                        syn_expr_idx: 1,
                                                    },
                                                    SynExprRoot {
                                                        kind: Trait,
                                                        syn_expr_idx: 8,
                                                    },
                                                    SynExprRoot {
                                                        kind: SelfType,
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
                                                SynInheritedSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: SynInheritedSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Type {
                                                            ident: `Label`,
                                                        },
                                                    ),
                                                },
                                                SynInheritedSymbol {
                                                    parent_symbol_idx: Current(
                                                        2,
                                                    ),
                                                    modifier: Const,
                                                    kind: SynInheritedSymbolKind::TemplateParameter(
                                                        InheritedTemplateParameterSynSymbol::Constant {
                                                            ident: `label`,
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: AssociatedTypeTerm,
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