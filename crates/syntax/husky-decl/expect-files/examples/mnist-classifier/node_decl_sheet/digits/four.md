Ok(
    NodeDeclSheet {
        [salsa id]: 29,
        decls: [
            (
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Fugitive(
                        FugitiveNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::four::left_components`, `Val`),
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
                                        path: FugitivePath(`mnist_classifier::digits::four::left_components`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 38,
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
                                                            path: FugitivePath(`mnist_classifier::digits::four::left_components`, `Val`),
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
                                path: FugitivePath(`mnist_classifier::digits::four::left_coordinate_max`, `Fn`),
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
                                        path: FugitivePath(`mnist_classifier::digits::four::left_coordinate_max`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 39,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    22,
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
                                                    22,
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
                                                            path: FugitivePath(`mnist_classifier::digits::four::left_coordinate_max`, `Fn`),
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
            (
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Fugitive(
                        FugitiveNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
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
                                        path: FugitivePath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 40,
                                colon_token: Ok(
                                    Some(
                                        ColonToken(
                                            TokenIdx(
                                                40,
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
                                            42,
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
                                                            path: FugitivePath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
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
                                                                41,
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
                                path: FugitivePath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
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
                                        path: FugitivePath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 41,
                                colon_token: Ok(
                                    Some(
                                        ColonToken(
                                            TokenIdx(
                                                53,
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
                                            55,
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
                                                            path: FugitivePath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
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
                                                                54,
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
                                path: FugitivePath(`mnist_classifier::digits::four::is_four`, `Val`),
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
                                        path: FugitivePath(`mnist_classifier::digits::four::is_four`, `Val`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 42,
                                colon_token: Ok(
                                    Some(
                                        ColonToken(
                                            TokenIdx(
                                                66,
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
                                            69,
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
                                                            path: FugitivePath(`mnist_classifier::digits::four::is_four`, `Val`),
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
                                                        67,
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
                                                                68,
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
                                path: FugitivePath(`mnist_classifier::digits::four::displacement_downwards`, `Fn`),
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
                                        path: FugitivePath(`mnist_classifier::digits::four::displacement_downwards`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 43,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    250,
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
                                                    250,
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
                                                            path: FugitivePath(`mnist_classifier::digits::four::displacement_downwards`, `Fn`),
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
            (
                EntityNodePath::ModuleItem(
                    ModuleItemNodePath::Fugitive(
                        FugitiveNodePath {
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: FugitivePath(`mnist_classifier::digits::four::cc_box_heights`, `Fn`),
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
                                        path: FugitivePath(`mnist_classifier::digits::four::cc_box_heights`, `Fn`),
                                        disambiguator: 0,
                                    },
                                },
                                ast_idx: 44,
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                explicit_parameter_decl_list: Err(
                                    NodeDeclError::Original(
                                        OriginalNodeDeclError::ExpectedRightParenthesisInParameterList(
                                            TokenStreamState {
                                                next_token_idx: TokenIdx(
                                                    279,
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
                                                    279,
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
                                                            path: FugitivePath(`mnist_classifier::digits::four::cc_box_heights`, `Fn`),
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