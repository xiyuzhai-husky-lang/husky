Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::Enum(
                    EnumTypeSynDefn {
                        path: TypePath(`malamute::OneVsAll`, `Enum`),
                        decl: EnumTypeSynDecl {
                            path: TypePath(`malamute::OneVsAll`, `Enum`),
                            template_parameters: [
                                TemplateParameterDecl {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: TemplateParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `Label`,
                                            token_idx: TokenIdx(
                                                4,
                                            ),
                                        },
                                        traits: None,
                                    },
                                },
                                TemplateParameterDecl {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    variant: TemplateParameterDeclPatternVariant::Constant {
                                        const_token: ConstToken {
                                            token_idx: TokenIdx(
                                                6,
                                            ),
                                        },
                                        ident_token: IdentToken {
                                            ident: `label`,
                                            token_idx: TokenIdx(
                                                7,
                                            ),
                                        },
                                        colon_token: ColonToken(
                                            TokenIdx(
                                                8,
                                            ),
                                        ),
                                        ty_expr: 0,
                                    },
                                },
                            ],
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Type(
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
                                            SynExpr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    9,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                4,
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
                                    pattern_expr_region: PatternSynExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_infos: [],
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
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        5,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    4,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        10,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                            ident_token: IdentToken {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    7,
                                                                ),
                                                            },
                                                            ty_expr_idx: 0,
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ConstantImplicitParameterType,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::Enum(
                    EnumTypeSynDefn {
                        path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                        decl: EnumTypeSynDecl {
                            path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                            template_parameters: [
                                TemplateParameterDecl {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: TemplateParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `Label`,
                                            token_idx: TokenIdx(
                                                19,
                                            ),
                                        },
                                        traits: None,
                                    },
                                },
                                TemplateParameterDecl {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    variant: TemplateParameterDeclPatternVariant::Constant {
                                        const_token: ConstToken {
                                            token_idx: TokenIdx(
                                                21,
                                            ),
                                        },
                                        ident_token: IdentToken {
                                            ident: `label`,
                                            token_idx: TokenIdx(
                                                22,
                                            ),
                                        },
                                        colon_token: ColonToken(
                                            TokenIdx(
                                                23,
                                            ),
                                        ),
                                        ty_expr: 0,
                                    },
                                },
                            ],
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Type(
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
                                            SynExpr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    24,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                19,
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
                                    pattern_expr_region: PatternSynExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_infos: [],
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
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        20,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    19,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        25,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                            ident_token: IdentToken {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    22,
                                                                ),
                                                            },
                                                            ty_expr_idx: 0,
                                                        },
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: True,
                                        allow_self_value: True,
                                        pattern_ty_constraints: [
                                            (
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ConstantImplicitParameterType,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveSynDefn::Gn(
                    GnSynDefn {
                        path: FugitivePath(`malamute::narrow_down`, `Gn`),
                        decl: GnSynDecl {
                            path: FugitivePath(`malamute::narrow_down`, `Gn`),
                            template_parameters: [
                                TemplateParameterDecl {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: TemplateParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `Label`,
                                            token_idx: TokenIdx(
                                                63,
                                            ),
                                        },
                                        traits: None,
                                    },
                                },
                                TemplateParameterDecl {
                                    annotated_variance_token: None,
                                    symbol: 1,
                                    variant: TemplateParameterDeclPatternVariant::Constant {
                                        const_token: ConstToken {
                                            token_idx: TokenIdx(
                                                65,
                                            ),
                                        },
                                        ident_token: IdentToken {
                                            ident: `label`,
                                            token_idx: TokenIdx(
                                                66,
                                            ),
                                        },
                                        colon_token: ColonToken(
                                            TokenIdx(
                                                67,
                                            ),
                                        ),
                                        ty_expr: 0,
                                    },
                                },
                            ],
                            parenic_parameters: [
                                SpecificParameterDecl::Variadic {
                                    dot_dot_dot_token: DotDotDotToken(
                                        TokenIdx(
                                            71,
                                        ),
                                    ),
                                    variadic_variant: VariadicVariant::Vec {
                                        lbox_token: LeftBoxBracketToken(
                                            TokenIdx(
                                                72,
                                            ),
                                        ),
                                        rbox_token: RightBoxBracketToken(
                                            TokenIdx(
                                                73,
                                            ),
                                        ),
                                    },
                                    symbol_modifier_keyword_group: None,
                                    ident_token: IdentToken {
                                        ident: `f`,
                                        token_idx: TokenIdx(
                                            74,
                                        ),
                                    },
                                    variable: 2,
                                    colon: ColonToken(
                                        TokenIdx(
                                            75,
                                        ),
                                    ),
                                    ty: 1,
                                },
                                SpecificParameterDecl::Keyed {
                                    pattern: 0,
                                    symbol_modifier_keyword_group: None,
                                    ident_token: IdentToken {
                                        ident: `skip`,
                                        token_idx: TokenIdx(
                                            78,
                                        ),
                                    },
                                    variable: 3,
                                    colon: ColonToken(
                                        TokenIdx(
                                            79,
                                        ),
                                    ),
                                    ty: 2,
                                    eq_token: EqToken(
                                        TokenIdx(
                                            81,
                                        ),
                                    ),
                                    default: Right(
                                        3,
                                    ),
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeExprBeforeColon {
                                    expr: 8,
                                },
                            ),
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::ModuleItem(
                                            ModuleItemSynNodePath::Fugitive(
                                                FugitiveSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            SynExpr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    68,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                63,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 0,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::Literal(
                                                TokenIdx(
                                                    82,
                                                ),
                                                Literal::Integer(
                                                    UnspecifiedRegular(
                                                        5,
                                                    ),
                                                ),
                                            ),
                                            SynExpr::PrincipalEntityPath {
                                                item_path_expr: 2,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    87,
                                                ),
                                                current_symbol_idx: 0,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                        ident_token: IdentToken {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                63,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 4,
                                                argument_expr_idx: 5,
                                            },
                                            SynExpr::CurrentSymbol {
                                                ident: `label`,
                                                token_idx: TokenIdx(
                                                    88,
                                                ),
                                                current_symbol_idx: 1,
                                                current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                    implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                        ident_token: IdentToken {
                                                            ident: `label`,
                                                            token_idx: TokenIdx(
                                                                66,
                                                            ),
                                                        },
                                                    },
                                                },
                                            },
                                            SynExpr::ExplicitApplication {
                                                function_expr_idx: 6,
                                                argument_expr_idx: 7,
                                            },
                                        ],
                                    },
                                    principal_item_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            76,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `i32`,
                                                        token_idx: TokenIdx(
                                                            80,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::i32`, `Extern`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `OneVsAllResult`,
                                                        token_idx: TokenIdx(
                                                            86,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    stmt_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_region: PatternSynExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [
                                                PatternSynExpr::Ident {
                                                    symbol_modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `skip`,
                                                        token_idx: TokenIdx(
                                                            78,
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
                                        pattern_infos: [
                                            Parameter,
                                        ],
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                PatternSynSymbol::Atom(
                                                    0,
                                                ),
                                            ],
                                        },
                                        pattern_symbol_maps: [
                                            [
                                                (
                                                    `skip`,
                                                    0,
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
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        64,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    63,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        69,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                            ident_token: IdentToken {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    66,
                                                                ),
                                                            },
                                                            ty_expr_idx: 0,
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        72,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ExplicitVariadicParameter {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `f`,
                                                            token_idx: TokenIdx(
                                                                74,
                                                            ),
                                                        },
                                                    },
                                                },
                                                CurrentSynSymbol {
                                                    modifier: None,
                                                    access_start: TokenIdx(
                                                        79,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSynSymbolVariant::ExplicitRegularParameter {
                                                        ident: `skip`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                            (
                                                ImplicitTypeParameter,
                                                ArenaIdxRange(
                                                    1..2,
                                                ),
                                            ),
                                            (
                                                ExplicitVariadicParameter {
                                                    ty: 1,
                                                },
                                                ArenaIdxRange(
                                                    2..3,
                                                ),
                                            ),
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 0,
                                                    ty_expr_idx: 2,
                                                },
                                                ArenaIdxRange(
                                                    3..4,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        SynExprRoot {
                                            kind: ConstantImplicitParameterType,
                                            expr_idx: 0,
                                        },
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 1,
                                        },
                                        SynExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 2,
                                        },
                                        SynExprRoot {
                                            kind: ExplicitParameterDefaultValue {
                                                ty_expr_idx: 2,
                                            },
                                            expr_idx: 3,
                                        },
                                        SynExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 8,
                                        },
                                    ],
                                },
                            },
                        },
                        body: None,
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: SynExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntitySynNodePath::ModuleItem(
                                                    ModuleItemSynNodePath::Fugitive(
                                                        FugitiveSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                                                disambiguator: 0,
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    SynExpr::CurrentSymbol {
                                                        ident: `Label`,
                                                        token_idx: TokenIdx(
                                                            68,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                            implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `Label`,
                                                                    token_idx: TokenIdx(
                                                                        63,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 0,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::i32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::Literal(
                                                        TokenIdx(
                                                            82,
                                                        ),
                                                        Literal::Integer(
                                                            UnspecifiedRegular(
                                                                5,
                                                            ),
                                                        ),
                                                    ),
                                                    SynExpr::PrincipalEntityPath {
                                                        item_path_expr: 2,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    SynExpr::CurrentSymbol {
                                                        ident: `Label`,
                                                        token_idx: TokenIdx(
                                                            87,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                            implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `Label`,
                                                                    token_idx: TokenIdx(
                                                                        63,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 4,
                                                        argument_expr_idx: 5,
                                                    },
                                                    SynExpr::CurrentSymbol {
                                                        ident: `label`,
                                                        token_idx: TokenIdx(
                                                            88,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                            implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                                ident_token: IdentToken {
                                                                    ident: `label`,
                                                                    token_idx: TokenIdx(
                                                                        66,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    SynExpr::ExplicitApplication {
                                                        function_expr_idx: 6,
                                                        argument_expr_idx: 7,
                                                    },
                                                ],
                                            },
                                            principal_item_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    76,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `i32`,
                                                                token_idx: TokenIdx(
                                                                    80,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `OneVsAllResult`,
                                                                token_idx: TokenIdx(
                                                                    86,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [],
                                            },
                                            pattern_expr_region: PatternSynExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternSynExpr::Ident {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `skip`,
                                                                token_idx: TokenIdx(
                                                                    78,
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
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSynSymbol::Atom(
                                                            0,
                                                        ),
                                                    ],
                                                },
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `skip`,
                                                            0,
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
                                                        CurrentSynSymbol {
                                                            modifier: Const,
                                                            access_start: TokenIdx(
                                                                64,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `Label`,
                                                                        token_idx: TokenIdx(
                                                                            63,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        CurrentSynSymbol {
                                                            modifier: Const,
                                                            access_start: TokenIdx(
                                                                69,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                                    ident_token: IdentToken {
                                                                        ident: `label`,
                                                                        token_idx: TokenIdx(
                                                                            66,
                                                                        ),
                                                                    },
                                                                    ty_expr_idx: 0,
                                                                },
                                                            },
                                                        },
                                                        CurrentSynSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                72,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ExplicitVariadicParameter {
                                                                symbol_modifier_keyword_group: None,
                                                                ident_token: IdentToken {
                                                                    ident: `f`,
                                                                    token_idx: TokenIdx(
                                                                        74,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                        CurrentSynSymbol {
                                                            modifier: None,
                                                            access_start: TokenIdx(
                                                                79,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSynSymbolVariant::ExplicitRegularParameter {
                                                                ident: `skip`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    (
                                                        ImplicitTypeParameter,
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                    (
                                                        ImplicitTypeParameter,
                                                        ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitVariadicParameter {
                                                            ty: 1,
                                                        },
                                                        ArenaIdxRange(
                                                            2..3,
                                                        ),
                                                    ),
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 0,
                                                            ty_expr_idx: 2,
                                                        },
                                                        ArenaIdxRange(
                                                            3..4,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                SynExprRoot {
                                                    kind: ConstantImplicitParameterType,
                                                    expr_idx: 0,
                                                },
                                                SynExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 1,
                                                },
                                                SynExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 2,
                                                },
                                                SynExprRoot {
                                                    kind: ExplicitParameterDefaultValue {
                                                        ty_expr_idx: 2,
                                                    },
                                                    expr_idx: 3,
                                                },
                                                SynExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 8,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntitySynNodePath::ModuleItem(
                                        ModuleItemSynNodePath::Fugitive(
                                            FugitiveSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`malamute::narrow_down`, `Gn`),
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
                                pattern_expr_region: PatternSynExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [],
                                    },
                                    pattern_infos: [],
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
                                                    0,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `Label`,
                                                    },
                                                ),
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Constant {
                                                        ident: `label`,
                                                    },
                                                ),
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    2,
                                                ),
                                                modifier: None,
                                                kind: InheritedSynSymbolKind::ExplicitParameter {
                                                    ident: `f`,
                                                },
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    3,
                                                ),
                                                modifier: None,
                                                kind: InheritedSynSymbolKind::ExplicitParameter {
                                                    ident: `skip`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                roots: [],
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ImplBlock(
            ImplBlockSynDecl::TraitForType(
                TraitForTypeImplBlockSynDecl {
                    path: TraitForTypeImplBlockPath {
                        module_path: `malamute`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: Path(
                            TypePath(
                                Id {
                                    value: 104,
                                },
                            ),
                        ),
                        disambiguator: 0,
                    },
                    template_parameters: [
                        TemplateParameterDecl {
                            annotated_variance_token: None,
                            symbol: 0,
                            variant: TemplateParameterDeclPatternVariant::Type {
                                ident_token: IdentToken {
                                    ident: `Label`,
                                    token_idx: TokenIdx(
                                        34,
                                    ),
                                },
                                traits: None,
                            },
                        },
                        TemplateParameterDecl {
                            annotated_variance_token: None,
                            symbol: 1,
                            variant: TemplateParameterDeclPatternVariant::Constant {
                                const_token: ConstToken {
                                    token_idx: TokenIdx(
                                        36,
                                    ),
                                },
                                ident_token: IdentToken {
                                    ident: `label`,
                                    token_idx: TokenIdx(
                                        37,
                                    ),
                                },
                                colon_token: ColonToken(
                                    TokenIdx(
                                        38,
                                    ),
                                ),
                                ty_expr: 0,
                            },
                        },
                    ],
                    trai_expr: TraitExpr {
                        expr: 7,
                    },
                    self_ty_decl: PathLeadingExpr(
                        SelfTypeExpr {
                            expr: 12,
                        },
                    ),
                    syn_expr_region: SynExprRegion {
                        data: SynExprRegionData {
                            parent: None,
                            path: RegionPath::Decl(
                                EntitySynNodePath::ImplBlock(
                                    ImplBlockSynNodePath::TraitForTypeImplBlock(
                                        TraitForTypeImplBlockSynNodePath {
                                            path: TraitForTypeImplBlockPath {
                                                module_path: `malamute`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: Path(
                                                    TypePath(
                                                        Id {
                                                            value: 104,
                                                        },
                                                    ),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                ),
                            ),
                            expr_arena: Arena {
                                data: [
                                    SynExpr::CurrentSymbol {
                                        ident: `Label`,
                                        token_idx: TokenIdx(
                                            39,
                                        ),
                                        current_symbol_idx: 0,
                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                            implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `Label`,
                                                    token_idx: TokenIdx(
                                                        34,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    SynExpr::PrincipalEntityPath {
                                        item_path_expr: 2,
                                        opt_path: Some(
                                            PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::Unveil`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExpr::PrincipalEntityPath {
                                        item_path_expr: 3,
                                        opt_path: Some(
                                            PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExpr::ExplicitApplication {
                                        function_expr_idx: 1,
                                        argument_expr_idx: 2,
                                    },
                                    SynExpr::CurrentSymbol {
                                        ident: `Label`,
                                        token_idx: TokenIdx(
                                            47,
                                        ),
                                        current_symbol_idx: 0,
                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                            implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `Label`,
                                                    token_idx: TokenIdx(
                                                        34,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    SynExpr::ExplicitApplication {
                                        function_expr_idx: 3,
                                        argument_expr_idx: 4,
                                    },
                                    SynExpr::CurrentSymbol {
                                        ident: `label`,
                                        token_idx: TokenIdx(
                                            48,
                                        ),
                                        current_symbol_idx: 1,
                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                            implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                ident_token: IdentToken {
                                                    ident: `label`,
                                                    token_idx: TokenIdx(
                                                        37,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    SynExpr::ExplicitApplication {
                                        function_expr_idx: 5,
                                        argument_expr_idx: 6,
                                    },
                                    SynExpr::PrincipalEntityPath {
                                        item_path_expr: 4,
                                        opt_path: Some(
                                            PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                            ),
                                        ),
                                    },
                                    SynExpr::CurrentSymbol {
                                        ident: `Label`,
                                        token_idx: TokenIdx(
                                            51,
                                        ),
                                        current_symbol_idx: 0,
                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                            implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                ident_token: IdentToken {
                                                    ident: `Label`,
                                                    token_idx: TokenIdx(
                                                        34,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    SynExpr::ExplicitApplication {
                                        function_expr_idx: 8,
                                        argument_expr_idx: 9,
                                    },
                                    SynExpr::CurrentSymbol {
                                        ident: `label`,
                                        token_idx: TokenIdx(
                                            52,
                                        ),
                                        current_symbol_idx: 1,
                                        current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                            implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                ident_token: IdentToken {
                                                    ident: `label`,
                                                    token_idx: TokenIdx(
                                                        37,
                                                    ),
                                                },
                                            },
                                        },
                                    },
                                    SynExpr::ExplicitApplication {
                                        function_expr_idx: 10,
                                        argument_expr_idx: 11,
                                    },
                                ],
                            },
                            principal_item_path_expr_arena: Arena {
                                data: [
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameToken::Ident(
                                            IdentToken {
                                                ident: `core`,
                                                token_idx: TokenIdx(
                                                    41,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::Module(
                                            `core`,
                                        ),
                                    },
                                    PrincipalEntityPathExpr::Subitem {
                                        parent: 0,
                                        scope_resolution_token: ScopeResolutionToken(
                                            TokenIdx(
                                                42,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentToken {
                                                ident: `ops`,
                                                token_idx: TokenIdx(
                                                    43,
                                                ),
                                            },
                                        ),
                                        path: Ok(
                                            PrincipalEntityPath::Module(
                                                `core::ops`,
                                            ),
                                        ),
                                    },
                                    PrincipalEntityPathExpr::Subitem {
                                        parent: 1,
                                        scope_resolution_token: ScopeResolutionToken(
                                            TokenIdx(
                                                44,
                                            ),
                                        ),
                                        ident_token: Ok(
                                            IdentToken {
                                                ident: `Unveil`,
                                                token_idx: TokenIdx(
                                                    45,
                                                ),
                                            },
                                        ),
                                        path: Ok(
                                            PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Trait(
                                                    TraitPath(`core::ops::Unveil`),
                                                ),
                                            ),
                                        ),
                                    },
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameToken::Ident(
                                            IdentToken {
                                                ident: `OneVsAllResult`,
                                                token_idx: TokenIdx(
                                                    46,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                            ModuleItemPath::Type(
                                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                                            ),
                                        ),
                                    },
                                    PrincipalEntityPathExpr::Root {
                                        path_name_token: PathNameToken::Ident(
                                            IdentToken {
                                                ident: `OneVsAll`,
                                                token_idx: TokenIdx(
                                                    50,
                                                ),
                                            },
                                        ),
                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                            ModuleItemPath::Type(
                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                            ),
                                        ),
                                    },
                                ],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_expr_region: PatternSynExprRegion {
                                pattern_expr_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_contracts: ArenaMap {
                                    data: [],
                                },
                                pattern_infos: [],
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
                                        CurrentSynSymbol {
                                            modifier: Const,
                                            access_start: TokenIdx(
                                                35,
                                            ),
                                            access_end: None,
                                            variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                    ident_token: IdentToken {
                                                        ident: `Label`,
                                                        token_idx: TokenIdx(
                                                            34,
                                                        ),
                                                    },
                                                },
                                            },
                                        },
                                        CurrentSynSymbol {
                                            modifier: Const,
                                            access_start: TokenIdx(
                                                40,
                                            ),
                                            access_end: None,
                                            variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                    ident_token: IdentToken {
                                                        ident: `label`,
                                                        token_idx: TokenIdx(
                                                            37,
                                                        ),
                                                    },
                                                    ty_expr_idx: 0,
                                                },
                                            },
                                        },
                                    ],
                                },
                                allow_self_type: True,
                                allow_self_value: False,
                                pattern_ty_constraints: [
                                    (
                                        ImplicitTypeParameter,
                                        ArenaIdxRange(
                                            0..1,
                                        ),
                                    ),
                                    (
                                        ImplicitTypeParameter,
                                        ArenaIdxRange(
                                            1..2,
                                        ),
                                    ),
                                ],
                            },
                            roots: [
                                SynExprRoot {
                                    kind: ConstantImplicitParameterType,
                                    expr_idx: 0,
                                },
                                SynExprRoot {
                                    kind: Trait,
                                    expr_idx: 7,
                                },
                                SynExprRoot {
                                    kind: SelfType,
                                    expr_idx: 12,
                                },
                            ],
                        },
                    },
                },
            ),
        ),
        Defn::AssociatedItem(
            AssociatedItemSynDefn::TraitForTypeItem(
                TraitForTypeItemSynDefn::AssociatedType(
                    TraitForTypeAssociatedTypeSynDefn {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `malamute`,
                                trai_path: TraitPath(`core::ops::Unveil`),
                                ty_sketch: Path(
                                    TypePath(
                                        Id {
                                            value: 104,
                                        },
                                    ),
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
                                    ty_sketch: Path(
                                        TypePath(
                                            Id {
                                                value: 104,
                                            },
                                        ),
                                    ),
                                    disambiguator: 0,
                                },
                                ident: `Output`,
                                item_kind: AssociatedType,
                            },
                            template_parameters: [],
                            ty_term_expr_idx: 0,
                            syn_expr_region: SynExprRegion {
                                data: SynExprRegionData {
                                    parent: Some(
                                        SynExprRegion {
                                            data: SynExprRegionData {
                                                parent: None,
                                                path: RegionPath::Decl(
                                                    EntitySynNodePath::ImplBlock(
                                                        ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                            TraitForTypeImplBlockSynNodePath {
                                                                path: TraitForTypeImplBlockPath {
                                                                    module_path: `malamute`,
                                                                    trai_path: TraitPath(`core::ops::Unveil`),
                                                                    ty_sketch: Path(
                                                                        TypePath(
                                                                            Id {
                                                                                value: 104,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        SynExpr::CurrentSymbol {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                39,
                                                            ),
                                                            current_symbol_idx: 0,
                                                            current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `Label`,
                                                                        token_idx: TokenIdx(
                                                                            34,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExpr::PrincipalEntityPath {
                                                            item_path_expr: 2,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExpr::PrincipalEntityPath {
                                                            item_path_expr: 3,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 1,
                                                            argument_expr_idx: 2,
                                                        },
                                                        SynExpr::CurrentSymbol {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                47,
                                                            ),
                                                            current_symbol_idx: 0,
                                                            current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `Label`,
                                                                        token_idx: TokenIdx(
                                                                            34,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 3,
                                                            argument_expr_idx: 4,
                                                        },
                                                        SynExpr::CurrentSymbol {
                                                            ident: `label`,
                                                            token_idx: TokenIdx(
                                                                48,
                                                            ),
                                                            current_symbol_idx: 1,
                                                            current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                                    ident_token: IdentToken {
                                                                        ident: `label`,
                                                                        token_idx: TokenIdx(
                                                                            37,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 5,
                                                            argument_expr_idx: 6,
                                                        },
                                                        SynExpr::PrincipalEntityPath {
                                                            item_path_expr: 4,
                                                            opt_path: Some(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        SynExpr::CurrentSymbol {
                                                            ident: `Label`,
                                                            token_idx: TokenIdx(
                                                                51,
                                                            ),
                                                            current_symbol_idx: 0,
                                                            current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                    ident_token: IdentToken {
                                                                        ident: `Label`,
                                                                        token_idx: TokenIdx(
                                                                            34,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 8,
                                                            argument_expr_idx: 9,
                                                        },
                                                        SynExpr::CurrentSymbol {
                                                            ident: `label`,
                                                            token_idx: TokenIdx(
                                                                52,
                                                            ),
                                                            current_symbol_idx: 1,
                                                            current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                                    ident_token: IdentToken {
                                                                        ident: `label`,
                                                                        token_idx: TokenIdx(
                                                                            37,
                                                                        ),
                                                                    },
                                                                },
                                                            },
                                                        },
                                                        SynExpr::ExplicitApplication {
                                                            function_expr_idx: 10,
                                                            argument_expr_idx: 11,
                                                        },
                                                    ],
                                                },
                                                principal_item_path_expr_arena: Arena {
                                                    data: [
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `core`,
                                                                    token_idx: TokenIdx(
                                                                        41,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::Module(
                                                                `core`,
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Subitem {
                                                            parent: 0,
                                                            scope_resolution_token: ScopeResolutionToken(
                                                                TokenIdx(
                                                                    42,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentToken {
                                                                    ident: `ops`,
                                                                    token_idx: TokenIdx(
                                                                        43,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::Module(
                                                                    `core::ops`,
                                                                ),
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Subitem {
                                                            parent: 1,
                                                            scope_resolution_token: ScopeResolutionToken(
                                                                TokenIdx(
                                                                    44,
                                                                ),
                                                            ),
                                                            ident_token: Ok(
                                                                IdentToken {
                                                                    ident: `Unveil`,
                                                                    token_idx: TokenIdx(
                                                                        45,
                                                                    ),
                                                                },
                                                            ),
                                                            path: Ok(
                                                                PrincipalEntityPath::ModuleItem(
                                                                    ModuleItemPath::Trait(
                                                                        TraitPath(`core::ops::Unveil`),
                                                                    ),
                                                                ),
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `OneVsAllResult`,
                                                                    token_idx: TokenIdx(
                                                                        46,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                        PrincipalEntityPathExpr::Root {
                                                            path_name_token: PathNameToken::Ident(
                                                                IdentToken {
                                                                    ident: `OneVsAll`,
                                                                    token_idx: TokenIdx(
                                                                        50,
                                                                    ),
                                                                },
                                                            ),
                                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ),
                                                            ),
                                                        },
                                                    ],
                                                },
                                                stmt_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_region: PatternSynExprRegion {
                                                    pattern_expr_arena: Arena {
                                                        data: [],
                                                    },
                                                    pattern_expr_contracts: ArenaMap {
                                                        data: [],
                                                    },
                                                    pattern_infos: [],
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
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    35,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                        ident_token: IdentToken {
                                                                            ident: `Label`,
                                                                            token_idx: TokenIdx(
                                                                                34,
                                                                            ),
                                                                        },
                                                                    },
                                                                },
                                                            },
                                                            CurrentSynSymbol {
                                                                modifier: Const,
                                                                access_start: TokenIdx(
                                                                    40,
                                                                ),
                                                                access_end: None,
                                                                variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                                    implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                                        ident_token: IdentToken {
                                                                            ident: `label`,
                                                                            token_idx: TokenIdx(
                                                                                37,
                                                                            ),
                                                                        },
                                                                        ty_expr_idx: 0,
                                                                    },
                                                                },
                                                            },
                                                        ],
                                                    },
                                                    allow_self_type: True,
                                                    allow_self_value: False,
                                                    pattern_ty_constraints: [
                                                        (
                                                            ImplicitTypeParameter,
                                                            ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                        ),
                                                        (
                                                            ImplicitTypeParameter,
                                                            ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        ),
                                                    ],
                                                },
                                                roots: [
                                                    SynExprRoot {
                                                        kind: ConstantImplicitParameterType,
                                                        expr_idx: 0,
                                                    },
                                                    SynExprRoot {
                                                        kind: Trait,
                                                        expr_idx: 7,
                                                    },
                                                    SynExprRoot {
                                                        kind: SelfType,
                                                        expr_idx: 12,
                                                    },
                                                ],
                                            },
                                        },
                                    ),
                                    path: RegionPath::Decl(
                                        EntitySynNodePath::AssociatedItem(
                                            AssociatedItemSynNodePath::TraitForTypeItem(
                                                TraitForTypeItemSynNodePath {
                                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                                        path: TraitForTypeItemPath {
                                                            impl_block: TraitForTypeImplBlockPath {
                                                                module_path: `malamute`,
                                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                                ty_sketch: Path(
                                                                    TypePath(
                                                                        Id {
                                                                            value: 104,
                                                                        },
                                                                    ),
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
                                            SynExpr::Unit {
                                                lpar_token_idx: TokenIdx(
                                                    57,
                                                ),
                                                rpar_token_idx: TokenIdx(
                                                    58,
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
                                    pattern_expr_region: PatternSynExprRegion {
                                        pattern_expr_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [],
                                        },
                                        pattern_infos: [],
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
                                                        0,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSynSymbol::Type {
                                                            ident: `Label`,
                                                        },
                                                    ),
                                                },
                                                InheritedSynSymbol {
                                                    parent_symbol_idx: Current(
                                                        1,
                                                    ),
                                                    modifier: Const,
                                                    kind: InheritedSynSymbolKind::ImplicitParameter(
                                                        InheritedImplicitParameterSynSymbol::Constant {
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
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                        syn_expr_region: SynExprRegion {
                            data: SynExprRegionData {
                                parent: Some(
                                    SynExprRegion {
                                        data: SynExprRegionData {
                                            parent: Some(
                                                SynExprRegion {
                                                    data: SynExprRegionData {
                                                        parent: None,
                                                        path: RegionPath::Decl(
                                                            EntitySynNodePath::ImplBlock(
                                                                ImplBlockSynNodePath::TraitForTypeImplBlock(
                                                                    TraitForTypeImplBlockSynNodePath {
                                                                        path: TraitForTypeImplBlockPath {
                                                                            module_path: `malamute`,
                                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                                            ty_sketch: Path(
                                                                                TypePath(
                                                                                    Id {
                                                                                        value: 104,
                                                                                    },
                                                                                ),
                                                                            ),
                                                                            disambiguator: 0,
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                SynExpr::CurrentSymbol {
                                                                    ident: `Label`,
                                                                    token_idx: TokenIdx(
                                                                        39,
                                                                    ),
                                                                    current_symbol_idx: 0,
                                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `Label`,
                                                                                token_idx: TokenIdx(
                                                                                    34,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                SynExpr::PrincipalEntityPath {
                                                                    item_path_expr: 2,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::ops::Unveil`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                SynExpr::PrincipalEntityPath {
                                                                    item_path_expr: 3,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 1,
                                                                    argument_expr_idx: 2,
                                                                },
                                                                SynExpr::CurrentSymbol {
                                                                    ident: `Label`,
                                                                    token_idx: TokenIdx(
                                                                        47,
                                                                    ),
                                                                    current_symbol_idx: 0,
                                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `Label`,
                                                                                token_idx: TokenIdx(
                                                                                    34,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 3,
                                                                    argument_expr_idx: 4,
                                                                },
                                                                SynExpr::CurrentSymbol {
                                                                    ident: `label`,
                                                                    token_idx: TokenIdx(
                                                                        48,
                                                                    ),
                                                                    current_symbol_idx: 1,
                                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                                            ident_token: IdentToken {
                                                                                ident: `label`,
                                                                                token_idx: TokenIdx(
                                                                                    37,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 5,
                                                                    argument_expr_idx: 6,
                                                                },
                                                                SynExpr::PrincipalEntityPath {
                                                                    item_path_expr: 4,
                                                                    opt_path: Some(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                SynExpr::CurrentSymbol {
                                                                    ident: `Label`,
                                                                    token_idx: TokenIdx(
                                                                        51,
                                                                    ),
                                                                    current_symbol_idx: 0,
                                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Type {
                                                                            ident_token: IdentToken {
                                                                                ident: `Label`,
                                                                                token_idx: TokenIdx(
                                                                                    34,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 8,
                                                                    argument_expr_idx: 9,
                                                                },
                                                                SynExpr::CurrentSymbol {
                                                                    ident: `label`,
                                                                    token_idx: TokenIdx(
                                                                        52,
                                                                    ),
                                                                    current_symbol_idx: 1,
                                                                    current_symbol_kind: CurrentSynSymbolKind::ImplicitParameter {
                                                                        implicit_parameter_kind: CurrentImplicitParameterSynSymbolKind::Constant {
                                                                            ident_token: IdentToken {
                                                                                ident: `label`,
                                                                                token_idx: TokenIdx(
                                                                                    37,
                                                                                ),
                                                                            },
                                                                        },
                                                                    },
                                                                },
                                                                SynExpr::ExplicitApplication {
                                                                    function_expr_idx: 10,
                                                                    argument_expr_idx: 11,
                                                                },
                                                            ],
                                                        },
                                                        principal_item_path_expr_arena: Arena {
                                                            data: [
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `core`,
                                                                            token_idx: TokenIdx(
                                                                                41,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::Module(
                                                                        `core`,
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Subitem {
                                                                    parent: 0,
                                                                    scope_resolution_token: ScopeResolutionToken(
                                                                        TokenIdx(
                                                                            42,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `ops`,
                                                                            token_idx: TokenIdx(
                                                                                43,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    path: Ok(
                                                                        PrincipalEntityPath::Module(
                                                                            `core::ops`,
                                                                        ),
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Subitem {
                                                                    parent: 1,
                                                                    scope_resolution_token: ScopeResolutionToken(
                                                                        TokenIdx(
                                                                            44,
                                                                        ),
                                                                    ),
                                                                    ident_token: Ok(
                                                                        IdentToken {
                                                                            ident: `Unveil`,
                                                                            token_idx: TokenIdx(
                                                                                45,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    path: Ok(
                                                                        PrincipalEntityPath::ModuleItem(
                                                                            ModuleItemPath::Trait(
                                                                                TraitPath(`core::ops::Unveil`),
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `OneVsAllResult`,
                                                                            token_idx: TokenIdx(
                                                                                46,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                                        ),
                                                                    ),
                                                                },
                                                                PrincipalEntityPathExpr::Root {
                                                                    path_name_token: PathNameToken::Ident(
                                                                        IdentToken {
                                                                            ident: `OneVsAll`,
                                                                            token_idx: TokenIdx(
                                                                                50,
                                                                            ),
                                                                        },
                                                                    ),
                                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                                        ),
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                        stmt_arena: Arena {
                                                            data: [],
                                                        },
                                                        pattern_expr_region: PatternSynExprRegion {
                                                            pattern_expr_arena: Arena {
                                                                data: [],
                                                            },
                                                            pattern_expr_contracts: ArenaMap {
                                                                data: [],
                                                            },
                                                            pattern_infos: [],
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
                                                                    CurrentSynSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            35,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                                ident_token: IdentToken {
                                                                                    ident: `Label`,
                                                                                    token_idx: TokenIdx(
                                                                                        34,
                                                                                    ),
                                                                                },
                                                                            },
                                                                        },
                                                                    },
                                                                    CurrentSynSymbol {
                                                                        modifier: Const,
                                                                        access_start: TokenIdx(
                                                                            40,
                                                                        ),
                                                                        access_end: None,
                                                                        variant: CurrentSynSymbolVariant::ImplicitParameter {
                                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                                                ident_token: IdentToken {
                                                                                    ident: `label`,
                                                                                    token_idx: TokenIdx(
                                                                                        37,
                                                                                    ),
                                                                                },
                                                                                ty_expr_idx: 0,
                                                                            },
                                                                        },
                                                                    },
                                                                ],
                                                            },
                                                            allow_self_type: True,
                                                            allow_self_value: False,
                                                            pattern_ty_constraints: [
                                                                (
                                                                    ImplicitTypeParameter,
                                                                    ArenaIdxRange(
                                                                        0..1,
                                                                    ),
                                                                ),
                                                                (
                                                                    ImplicitTypeParameter,
                                                                    ArenaIdxRange(
                                                                        1..2,
                                                                    ),
                                                                ),
                                                            ],
                                                        },
                                                        roots: [
                                                            SynExprRoot {
                                                                kind: ConstantImplicitParameterType,
                                                                expr_idx: 0,
                                                            },
                                                            SynExprRoot {
                                                                kind: Trait,
                                                                expr_idx: 7,
                                                            },
                                                            SynExprRoot {
                                                                kind: SelfType,
                                                                expr_idx: 12,
                                                            },
                                                        ],
                                                    },
                                                },
                                            ),
                                            path: RegionPath::Decl(
                                                EntitySynNodePath::AssociatedItem(
                                                    AssociatedItemSynNodePath::TraitForTypeItem(
                                                        TraitForTypeItemSynNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: TraitForTypeItemPath {
                                                                    impl_block: TraitForTypeImplBlockPath {
                                                                        module_path: `malamute`,
                                                                        trai_path: TraitPath(`core::ops::Unveil`),
                                                                        ty_sketch: Path(
                                                                            TypePath(
                                                                                Id {
                                                                                    value: 104,
                                                                                },
                                                                            ),
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
                                                    SynExpr::Unit {
                                                        lpar_token_idx: TokenIdx(
                                                            57,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            58,
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
                                            pattern_expr_region: PatternSynExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [],
                                                },
                                                pattern_infos: [],
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
                                                                0,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSynSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSynSymbol::Type {
                                                                    ident: `Label`,
                                                                },
                                                            ),
                                                        },
                                                        InheritedSynSymbol {
                                                            parent_symbol_idx: Current(
                                                                1,
                                                            ),
                                                            modifier: Const,
                                                            kind: InheritedSynSymbolKind::ImplicitParameter(
                                                                InheritedImplicitParameterSynSymbol::Constant {
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
                                                    expr_idx: 0,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntitySynNodePath::AssociatedItem(
                                        AssociatedItemSynNodePath::TraitForTypeItem(
                                            TraitForTypeItemSynNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: TraitForTypeItemPath {
                                                        impl_block: TraitForTypeImplBlockPath {
                                                            module_path: `malamute`,
                                                            trai_path: TraitPath(`core::ops::Unveil`),
                                                            ty_sketch: Path(
                                                                TypePath(
                                                                    Id {
                                                                        value: 104,
                                                                    },
                                                                ),
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
                                    data: [],
                                },
                                principal_item_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [],
                                },
                                pattern_expr_region: PatternSynExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [],
                                    },
                                    pattern_infos: [],
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
                                                    0,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Type {
                                                        ident: `Label`,
                                                    },
                                                ),
                                            },
                                            InheritedSynSymbol {
                                                parent_symbol_idx: Current(
                                                    1,
                                                ),
                                                modifier: Const,
                                                kind: InheritedSynSymbolKind::ImplicitParameter(
                                                    InheritedImplicitParameterSynSymbol::Constant {
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
                                roots: [],
                            },
                        },
                    },
                ),
            ),
        ),
    ],
)