Ok(
    NodeDeclSheet {
        [salsa id]: 27,
        decls: [
            (
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Fugitive(
                        FugitiveNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ModuleItem(
                    ModuleItemNodeDecl::Fugitive(
                        FugitiveNodeDecl::Val(
                            ValNodeDecl {
                                node_path: FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 18,
                                colon_token: Ok(
                                    Some(
                                        ColonToken(
                                            TokenIdx(
                                                6,
                                            ),
                                        ),
                                    ),
                                ),
                                val_ty: Ok(
                                    Some(
                                        FormTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                ),
                                eq_token: Ok(
                                    EqToken(
                                        TokenIdx(
                                            8,
                                        ),
                                    ),
                                ),
                                expr: None,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntityNodePath::ModuleItem(
                                                ModuleItemNodePath::Fugitive(
                                                    FugitiveNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
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
                                                                TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                            ident: `FermiMatchResult`,
                                                            token_idx: TokenIdx(
                                                                7,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
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
                                                data: [],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: VarType,
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
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Fugitive(
                        FugitiveNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ModuleItem(
                    ModuleItemNodeDecl::Fugitive(
                        FugitiveNodeDecl::Val(
                            ValNodeDecl {
                                node_path: FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 19,
                                colon_token: Ok(
                                    Some(
                                        ColonToken(
                                            TokenIdx(
                                                23,
                                            ),
                                        ),
                                    ),
                                ),
                                val_ty: Ok(
                                    Some(
                                        FormTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                ),
                                eq_token: Ok(
                                    EqToken(
                                        TokenIdx(
                                            26,
                                        ),
                                    ),
                                ),
                                expr: None,
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntityNodePath::ModuleItem(
                                                ModuleItemNodePath::Fugitive(
                                                    FugitiveNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
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
                                                                TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Prefix {
                                                    opr: Option,
                                                    opr_token_idx: TokenIdx(
                                                        24,
                                                    ),
                                                    opd: 0,
                                                },
                                            ],
                                        },
                                        principal_entity_path_expr_arena: Arena {
                                            data: [
                                                PrincipalEntityPathExpr::Root {
                                                    path_name_token: PathNameToken::Ident(
                                                        IdentToken {
                                                            ident: `MnistLabel`,
                                                            token_idx: TokenIdx(
                                                                25,
                                                            ),
                                                        },
                                                    ),
                                                    principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
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
                                                data: [],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: VarType,
                                                expr_idx: 1,
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
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Fugitive(
                        FugitiveNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
                                disambiguator: 0,
                            },
                        },
                    ),
                ),
                NodeDecl::ModuleItem(
                    ModuleItemNodeDecl::Fugitive(
                        FugitiveNodeDecl::Fn(
                            FnNodeDecl {
                                node_path: FugitiveNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 20,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    87,
                                                ),
                                                drained: false,
                                            },
                                        ),
                                    ),
                                ),
                                curry_token: Ok(
                                    None,
                                ),
                                return_ty: Ok(
                                    None,
                                ),
                                eol_colon: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedEolColon(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    87,
                                                ),
                                                drained: false,
                                            },
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            EntityNodePath::ModuleItem(
                                                ModuleItemNodePath::Fugitive(
                                                    FugitiveNodePath {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [],
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
            ),
        ],
    },
)