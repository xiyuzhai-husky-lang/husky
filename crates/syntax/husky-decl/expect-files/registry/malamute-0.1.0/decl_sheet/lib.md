Ok(
    DeclSheet {
        [salsa id]: 48,
        decls: [
            (
                EntityPath::ModuleItem(
                    ModuleItemPath::Type(
                        TypePath(`malamute::OneVsAllResult`, `Enum`),
                    ),
                ),
                Decl::ModuleItem(
                    ModuleItemDecl::Type(
                        TypeDecl::Enum(
                            EnumTypeDecl {
                                path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                                implicit_parameters: [
                                    ImplicitParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 0,
                                        variant: ImplicitParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    4,
                                                ),
                                            },
                                            traits: None,
                                        },
                                    },
                                    ImplicitParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 1,
                                        variant: ImplicitParameterDeclPatternVariant::Constant {
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
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntityNodePath::ModuleItem(
                                                ModuleItemNodePath::Type(
                                                    TypeNodePath {
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
                                                Expr::CurrentSymbol {
                                                    ident: `Label`,
                                                    token_idx: TokenIdx(
                                                        9,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
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
                                        principal_entity_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [],
                                        },
                                        pattern_expr_region: PatternExprRegion {
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
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            5,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
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
                                                    CurrentSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            10,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
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
                                            ExprRoot {
                                                kind: ConstantImplicitParameterType,
                                                expr_idx: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                EntityPath::ModuleItem(
                    ModuleItemPath::Fugitive(
                        FugitivePath(`malamute::narrow_down`, `Gn`),
                    ),
                ),
                Decl::ModuleItem(
                    ModuleItemDecl::Fugitive(
                        FugitiveDecl::Gn(
                            GnDecl {
                                path: FugitivePath(`malamute::narrow_down`, `Gn`),
                                implicit_parameters: [
                                    ImplicitParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 0,
                                        variant: ImplicitParameterDeclPatternVariant::Type {
                                            ident_token: IdentToken {
                                                ident: `Label`,
                                                token_idx: TokenIdx(
                                                    21,
                                                ),
                                            },
                                            traits: None,
                                        },
                                    },
                                    ImplicitParameterDecl {
                                        annotated_variance_token: None,
                                        symbol: 1,
                                        variant: ImplicitParameterDeclPatternVariant::Constant {
                                            const_token: ConstToken {
                                                token_idx: TokenIdx(
                                                    23,
                                                ),
                                            },
                                            ident_token: IdentToken {
                                                ident: `label`,
                                                token_idx: TokenIdx(
                                                    24,
                                                ),
                                            },
                                            colon_token: ColonToken(
                                                TokenIdx(
                                                    25,
                                                ),
                                            ),
                                            ty_expr: 0,
                                        },
                                    },
                                ],
                                explicit_parameters: [
                                    ExplicitParameterDecl::Variadic {
                                        dot_dot_dot_token: DotDotDotToken(
                                            TokenIdx(
                                                29,
                                            ),
                                        ),
                                        variadic_variant: VariadicVariant::Vec {
                                            lbox_token: LeftBoxBracketToken(
                                                TokenIdx(
                                                    30,
                                                ),
                                            ),
                                            rbox_token: RightBoxBracketToken(
                                                TokenIdx(
                                                    31,
                                                ),
                                            ),
                                        },
                                        symbol_modifier_keyword_group: None,
                                        ident_token: IdentToken {
                                            ident: `f`,
                                            token_idx: TokenIdx(
                                                32,
                                            ),
                                        },
                                        variable: 2,
                                        colon: ColonToken(
                                            TokenIdx(
                                                33,
                                            ),
                                        ),
                                        ty: 1,
                                    },
                                    ExplicitParameterDecl::Keyed {
                                        pattern: 0,
                                        symbol_modifier_keyword_group: None,
                                        ident_token: IdentToken {
                                            ident: `skip`,
                                            token_idx: TokenIdx(
                                                36,
                                            ),
                                        },
                                        variable: 3,
                                        colon: ColonToken(
                                            TokenIdx(
                                                37,
                                            ),
                                        ),
                                        ty: 2,
                                        eq_token: EqToken(
                                            TokenIdx(
                                                39,
                                            ),
                                        ),
                                        default: Right(
                                            3,
                                        ),
                                    },
                                ],
                                return_ty: Some(
                                    ReturnTypeExpr {
                                        expr: 8,
                                    },
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntityNodePath::ModuleItem(
                                                ModuleItemNodePath::Fugitive(
                                                    FugitiveNodePath {
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
                                                Expr::CurrentSymbol {
                                                    ident: `Label`,
                                                    token_idx: TokenIdx(
                                                        26,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    21,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 0,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 1,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        40,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            5,
                                                        ),
                                                    ),
                                                ),
                                                Expr::PrincipalEntityPath {
                                                    entity_path_expr: 2,
                                                    opt_path: Some(
                                                        PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`malamute::OneVsAllResult`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `Label`,
                                                    token_idx: TokenIdx(
                                                        45,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Type {
                                                            ident_token: IdentToken {
                                                                ident: `Label`,
                                                                token_idx: TokenIdx(
                                                                    21,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                Expr::ExplicitApplication {
                                                    function_expr_idx: 4,
                                                    argument_expr_idx: 5,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `label`,
                                                    token_idx: TokenIdx(
                                                        46,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::ImplicitParameter {
                                                        implicit_parameter_kind: CurrentImplicitParameterSymbolKind::Constant {
                                                            ident_token: IdentToken {
                                                                ident: `label`,
                                                                token_idx: TokenIdx(
                                                                    24,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                },
                                                Expr::ExplicitApplication {
                                                    function_expr_idx: 6,
                                                    argument_expr_idx: 7,
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `f32`,
                                                            token_idx: TokenIdx(
                                                                34,
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
                                                                38,
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
                                                                44,
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
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Ident {
                                                        symbol_modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `skip`,
                                                            token_idx: TokenIdx(
                                                                36,
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
                                                    PatternSymbol::Atom(
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
                                        symbol_region: SymbolRegion {
                                            inherited_symbol_arena: Arena {
                                                data: [],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            22,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                                ident_token: IdentToken {
                                                                    ident: `Label`,
                                                                    token_idx: TokenIdx(
                                                                        21,
                                                                    ),
                                                                },
                                                            },
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Const,
                                                        access_start: TokenIdx(
                                                            27,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ImplicitParameter {
                                                            implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                                ident_token: IdentToken {
                                                                    ident: `label`,
                                                                    token_idx: TokenIdx(
                                                                        24,
                                                                    ),
                                                                },
                                                                ty_expr_idx: 0,
                                                            },
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: None,
                                                        access_start: TokenIdx(
                                                            30,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitVariadicParameter {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `f`,
                                                                token_idx: TokenIdx(
                                                                    32,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: None,
                                                        access_start: TokenIdx(
                                                            37,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitRegularParameter {
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
                                            ExprRoot {
                                                kind: ConstantImplicitParameterType,
                                                expr_idx: 0,
                                            },
                                            ExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 1,
                                            },
                                            ExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 2,
                                            },
                                            ExprRoot {
                                                kind: ExplicitParameterDefaultValue {
                                                    ty_expr_idx: 2,
                                                },
                                                expr_idx: 3,
                                            },
                                            ExprRoot {
                                                kind: ReturnType,
                                                expr_idx: 8,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)