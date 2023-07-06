Ok(
    DeclSheet {
        [salsa id]: 48,
        decls: [
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
                                implicit_parameters: [],
                                explicit_parameters: [
                                    ExplicitParameterDecl::Variadic {
                                        dot_dot_dot_token: DotDotDotToken(
                                            TokenIdx(
                                                4,
                                            ),
                                        ),
                                        variadic_variant: VariadicVariant::Vec {
                                            lbox_token: LeftBoxBracketToken(
                                                TokenIdx(
                                                    5,
                                                ),
                                            ),
                                            rbox_token: RightBoxBracketToken(
                                                TokenIdx(
                                                    6,
                                                ),
                                            ),
                                        },
                                        symbol_modifier_keyword_group: None,
                                        ident_token: IdentToken {
                                            ident: `f`,
                                            token_idx: TokenIdx(
                                                7,
                                            ),
                                        },
                                        variable: 0,
                                        colon: ColonToken(
                                            TokenIdx(
                                                8,
                                            ),
                                        ),
                                        ty: 0,
                                    },
                                    ExplicitParameterDecl::Keyed {
                                        pattern: 0,
                                        symbol_modifier_keyword_group: None,
                                        ident_token: IdentToken {
                                            ident: `skip`,
                                            token_idx: TokenIdx(
                                                11,
                                            ),
                                        },
                                        variable: 1,
                                        colon: ColonToken(
                                            TokenIdx(
                                                12,
                                            ),
                                        ),
                                        ty: 1,
                                        eq_token: EqToken(
                                            TokenIdx(
                                                14,
                                            ),
                                        ),
                                        default: Right(
                                            2,
                                        ),
                                    },
                                ],
                                return_ty: None,
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
                                                        15,
                                                    ),
                                                    Literal::Integer(
                                                        UnspecifiedRegular(
                                                            5,
                                                        ),
                                                    ),
                                                ),
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `f32`,
                                                            token_idx: TokenIdx(
                                                                9,
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
                                                                13,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::i32`, `Extern`),
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
                                                                11,
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
                                                        modifier: None,
                                                        access_start: TokenIdx(
                                                            5,
                                                        ),
                                                        access_end: None,
                                                        variant: CurrentSymbolVariant::ExplicitVariadicParameter {
                                                            symbol_modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `f`,
                                                                token_idx: TokenIdx(
                                                                    7,
                                                                ),
                                                            },
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: None,
                                                        access_start: TokenIdx(
                                                            12,
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
                                                    ExplicitVariadicParameter {
                                                        ty: 0,
                                                    },
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                                (
                                                    ExplicitRegularParameter {
                                                        pattern_expr_idx: 0,
                                                        ty_expr_idx: 1,
                                                    },
                                                    ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                ),
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 0,
                                            },
                                            ExprRoot {
                                                kind: ExplicitParameterType,
                                                expr_idx: 1,
                                            },
                                            ExprRoot {
                                                kind: ExplicitParameterDefaultValue {
                                                    ty_expr_idx: 1,
                                                },
                                                expr_idx: 2,
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