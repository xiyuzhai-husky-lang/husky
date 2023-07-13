Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Type(
                TypeDefn::Extern(
                    ExternTypeDefn {
                        path: TypePath(`core::array::Array`, `Extern`),
                        decl: ExternTypeDecl {
                            path: TypePath(`core::array::Array`, `Extern`),
                            generic_parameters: [
                                GenericParameterDecl {
                                    annotated_variance_token: None,
                                    symbol: 0,
                                    variant: GenericParameterDeclPatternVariant::Constant {
                                        const_token: ConstToken {
                                            token_idx: TokenIdx(
                                                4,
                                            ),
                                        },
                                        ident_token: IdentToken {
                                            ident: `L`,
                                            token_idx: TokenIdx(
                                                5,
                                            ),
                                        },
                                        colon_token: ColonToken(
                                            TokenIdx(
                                                6,
                                            ),
                                        ),
                                        ty_expr: 0,
                                    },
                                },
                                GenericParameterDecl {
                                    annotated_variance_token: Some(
                                        VarianceToken::Covariant(
                                            CovariantToken {
                                                token_idx: TokenIdx(
                                                    9,
                                                ),
                                            },
                                        ),
                                    ),
                                    symbol: 1,
                                    variant: GenericParameterDeclPatternVariant::Type {
                                        ident_token: IdentToken {
                                            ident: `E`,
                                            token_idx: TokenIdx(
                                                10,
                                            ),
                                        },
                                        traits: None,
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
                                                        path: TypePath(`core::array::Array`, `Extern`),
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
                                                            TypePath(`core::num::usize`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `usize`,
                                                        token_idx: TokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::usize`, `Extern`),
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
                                                        8,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Constant {
                                                            ident_token: IdentToken {
                                                                ident: `L`,
                                                                token_idx: TokenIdx(
                                                                    5,
                                                                ),
                                                            },
                                                            ty_expr_idx: 0,
                                                        },
                                                    },
                                                },
                                                CurrentSymbol {
                                                    modifier: Const,
                                                    access_start: TokenIdx(
                                                        11,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ImplicitParameter {
                                                        implicit_parameter_variant: CurrentImplicitParameterSymbol::Type {
                                                            ident_token: IdentToken {
                                                                ident: `E`,
                                                                token_idx: TokenIdx(
                                                                    10,
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
                    },
                ),
            ),
        ),
    ],
)