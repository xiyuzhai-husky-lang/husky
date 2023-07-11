Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Val(
                    ValDefn {
                        path: FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                        decl: ValDecl {
                            path: FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                            return_ty: Some(
                                ReturnTypeExprBeforeEq {
                                    expr: 0,
                                },
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
                                                        path: FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
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
                                            kind: ReturnType,
                                            expr_idx: 0,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            7,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntityNodePath::ModuleItem(
                                                    ModuleItemNodePath::Fugitive(
                                                        FugitiveNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
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
                                                    kind: ReturnType,
                                                    expr_idx: 0,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
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
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::three::downarc`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::three::uparc`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 4,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::three::back`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::List {
                                            lbox_token_idx: TokenIdx(
                                                13,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 2,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            15,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 3,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            17,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 4,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                19,
                                            ),
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                10,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 1,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            12,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 5,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                20,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `fermi_match`,
                                                    token_idx: TokenIdx(
                                                        9,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_concave_components`,
                                                    token_idx: TokenIdx(
                                                        11,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `downarc`,
                                                    token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::three::downarc`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `uparc`,
                                                    token_idx: TokenIdx(
                                                        16,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::three::uparc`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `back`,
                                                    token_idx: TokenIdx(
                                                        18,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::three::back`, `Fn`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr_idx: 6,
                                        },
                                    ],
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
                                        kind: EvalExpr,
                                        expr_idx: 6,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 7,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Val(
                    ValDefn {
                        path: FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                        decl: ValDecl {
                            path: FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                            return_ty: Some(
                                ReturnTypeExprBeforeEq {
                                    expr: 4,
                                },
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
                                                        path: FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
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
                                                            TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::ExplicitApplication {
                                                function_expr_idx: 0,
                                                argument_expr_idx: 1,
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 3,
                                                opt_path: Some(
                                                    PrincipalEntityPath::TypeVariant(
                                                        TypeVariantPath {
                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ident: `Three`,
                                                        },
                                                    ),
                                                ),
                                            },
                                            Expr::ExplicitApplication {
                                                function_expr_idx: 2,
                                                argument_expr_idx: 3,
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `OneVsAll`,
                                                        token_idx: TokenIdx(
                                                            28,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`malamute::OneVsAll`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `MnistLabel`,
                                                        token_idx: TokenIdx(
                                                            29,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist::MnistLabel`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `MnistLabel`,
                                                        token_idx: TokenIdx(
                                                            30,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist::MnistLabel`, `Enum`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Subentity {
                                                parent: 2,
                                                scope_resolution_token: ScopeResolutionToken(
                                                    TokenIdx(
                                                        31,
                                                    ),
                                                ),
                                                ident_token: Ok(
                                                    IdentToken {
                                                        ident: `Three`,
                                                        token_idx: TokenIdx(
                                                            32,
                                                        ),
                                                    },
                                                ),
                                                path: Ok(
                                                    PrincipalEntityPath::TypeVariant(
                                                        TypeVariantPath {
                                                            parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ident: `Three`,
                                                        },
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
                                            kind: ReturnType,
                                            expr_idx: 4,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            60,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntityNodePath::ModuleItem(
                                                    ModuleItemNodePath::Fugitive(
                                                        FugitiveNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
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
                                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function_expr_idx: 0,
                                                        argument_expr_idx: 1,
                                                    },
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 3,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::TypeVariant(
                                                                TypeVariantPath {
                                                                    parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                    ident: `Three`,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function_expr_idx: 2,
                                                        argument_expr_idx: 3,
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `OneVsAll`,
                                                                token_idx: TokenIdx(
                                                                    28,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`malamute::OneVsAll`, `Enum`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `MnistLabel`,
                                                                token_idx: TokenIdx(
                                                                    29,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `MnistLabel`,
                                                                token_idx: TokenIdx(
                                                                    30,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist::MnistLabel`, `Enum`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Subentity {
                                                        parent: 2,
                                                        scope_resolution_token: ScopeResolutionToken(
                                                            TokenIdx(
                                                                31,
                                                            ),
                                                        ),
                                                        ident_token: Ok(
                                                            IdentToken {
                                                                ident: `Three`,
                                                                token_idx: TokenIdx(
                                                                    32,
                                                                ),
                                                            },
                                                        ),
                                                        path: Ok(
                                                            PrincipalEntityPath::TypeVariant(
                                                                TypeVariantPath {
                                                                    parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                    ident: `Three`,
                                                                },
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
                                                    kind: ReturnType,
                                                    expr_idx: 4,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
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
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                36,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    37,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                38,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                39,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                41,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 1,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                40,
                                            ),
                                            ropd: 2,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 4,
                                            dot_token_idx: TokenIdx(
                                                44,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    45,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                46,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                47,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                49,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    4,
                                                ),
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 5,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                48,
                                            ),
                                            ropd: 6,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 8,
                                            dot_token_idx: TokenIdx(
                                                54,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    55,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                57,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    0,
                                                ),
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 9,
                                            lbox_token_idx: TokenIdx(
                                                56,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 10,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                58,
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 12,
                                            dot_token_idx: TokenIdx(
                                                63,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    64,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                66,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    1,
                                                ),
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 13,
                                            lbox_token_idx: TokenIdx(
                                                65,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 14,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                67,
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 4,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 16,
                                            dot_token_idx: TokenIdx(
                                                72,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    73,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                75,
                                            ),
                                            Literal::Integer(
                                                UnspecifiedRegular(
                                                    2,
                                                ),
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 17,
                                            lbox_token_idx: TokenIdx(
                                                74,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 18,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                76,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downarc`,
                                            token_idx: TokenIdx(
                                                78,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Be {
                                            src: 20,
                                            be_token_idx: TokenIdx(
                                                79,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
                                                    pattern_expr: 3,
                                                    variables: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                },
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downarc`,
                                            token_idx: TokenIdx(
                                                82,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 22,
                                            dot_token_idx: TokenIdx(
                                                83,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    84,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                86,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 23,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                85,
                                            ),
                                            ropd: 24,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `uparc`,
                                            token_idx: TokenIdx(
                                                88,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::Be {
                                            src: 26,
                                            be_token_idx: TokenIdx(
                                                89,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
                                                    pattern_expr: 4,
                                                    variables: ArenaIdxRange(
                                                        4..5,
                                                    ),
                                                },
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downarc`,
                                            token_idx: TokenIdx(
                                                94,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 28,
                                            dot_token_idx: TokenIdx(
                                                95,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end_tangent`,
                                                token_idx: TokenIdx(
                                                    96,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                97,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                98,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                102,
                                            ),
                                            Literal::Bool(
                                                True,
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 29,
                                            dot_token_idx: TokenIdx(
                                                99,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle`,
                                                token_idx: TokenIdx(
                                                    100,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                101,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 30,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                103,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `de`,
                                            token_idx: TokenIdx(
                                                105,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                107,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                112,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `de`,
                                            token_idx: TokenIdx(
                                                109,
                                            ),
                                            current_symbol_idx: 5,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 5,
                                            },
                                        },
                                        Expr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                111,
                                            ),
                                            opd: 34,
                                        },
                                        Expr::Binary {
                                            lopd: 32,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                106,
                                            ),
                                            ropd: 33,
                                        },
                                        Expr::Binary {
                                            lopd: 35,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                110,
                                            ),
                                            ropd: 36,
                                        },
                                        Expr::Binary {
                                            lopd: 37,
                                            opr: ShortCircuitLogic(
                                                Or,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                108,
                                            ),
                                            ropd: 38,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downarc`,
                                            token_idx: TokenIdx(
                                                116,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 40,
                                            dot_token_idx: TokenIdx(
                                                117,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    118,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                119,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                120,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `uparc`,
                                            token_idx: TokenIdx(
                                                124,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 42,
                                            dot_token_idx: TokenIdx(
                                                125,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `start`,
                                                token_idx: TokenIdx(
                                                    126,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                127,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                128,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downarc_enpoint`,
                                            token_idx: TokenIdx(
                                                132,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 6,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `uparc_startpoint`,
                                            token_idx: TokenIdx(
                                                136,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 44,
                                            dot_token_idx: TokenIdx(
                                                133,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `dist`,
                                                token_idx: TokenIdx(
                                                    134,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                135,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 45,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                137,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `distance`,
                                            token_idx: TokenIdx(
                                                139,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                141,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 47,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                140,
                                            ),
                                            ropd: 48,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 5,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 50,
                                            dot_token_idx: TokenIdx(
                                                144,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    145,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                147,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 51,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                146,
                                            ),
                                            ropd: 52,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `downarc`,
                                            token_idx: TokenIdx(
                                                149,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                154,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 54,
                                            dot_token_idx: TokenIdx(
                                                150,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    151,
                                                ),
                                            },
                                        },
                                        Expr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                153,
                                            ),
                                            opd: 55,
                                        },
                                        Expr::Binary {
                                            lopd: 56,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                152,
                                            ),
                                            ropd: 57,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 7,
                                            opt_path: Some(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ident: `Yes`,
                                                    },
                                                ),
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..17,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_concave_components`,
                                                    token_idx: TokenIdx(
                                                        35,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_concave_components`,
                                                    token_idx: TokenIdx(
                                                        43,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `three_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        53,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `three_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        62,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `three_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        71,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `three_fermi_match`,
                                                    token_idx: TokenIdx(
                                                        143,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::three::three_fermi_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `OneVsAll`,
                                                    token_idx: TokenIdx(
                                                        155,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Subentity {
                                            parent: 6,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    156,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Yes`,
                                                    token_idx: TokenIdx(
                                                        157,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                        ident: `Yes`,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    34,
                                                ),
                                            },
                                            condition: 3,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    42,
                                                ),
                                            },
                                            condition: 7,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    50,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        52,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 11,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    59,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 1,
                                                    variables: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        61,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 15,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    68,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 2,
                                                    variables: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        70,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 19,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    77,
                                                ),
                                            },
                                            condition: 21,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    81,
                                                ),
                                            },
                                            condition: 25,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    87,
                                                ),
                                            },
                                            condition: 27,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    91,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 5,
                                                    variables: ArenaIdxRange(
                                                        5..6,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        93,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 31,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    104,
                                                ),
                                            },
                                            condition: 39,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    113,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 6,
                                                    variables: ArenaIdxRange(
                                                        6..7,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        115,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 41,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    121,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 7,
                                                    variables: ArenaIdxRange(
                                                        7..8,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        123,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 43,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    129,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 8,
                                                    variables: ArenaIdxRange(
                                                        8..9,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        131,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 46,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    138,
                                                ),
                                            },
                                            condition: 49,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    142,
                                                ),
                                            },
                                            condition: 53,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    148,
                                                ),
                                            },
                                            condition: 58,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 59,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `downarc`,
                                                    token_idx: TokenIdx(
                                                        51,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `uparc`,
                                                    token_idx: TokenIdx(
                                                        60,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `back`,
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        80,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        90,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `de`,
                                                    token_idx: TokenIdx(
                                                        92,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `downarc_enpoint`,
                                                    token_idx: TokenIdx(
                                                        114,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `uparc_startpoint`,
                                                    token_idx: TokenIdx(
                                                        122,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `distance`,
                                                    token_idx: TokenIdx(
                                                        130,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                        ],
                                    },
                                    pattern_infos: [
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            PatternSymbol::Atom(
                                                0,
                                            ),
                                            PatternSymbol::Atom(
                                                1,
                                            ),
                                            PatternSymbol::Atom(
                                                2,
                                            ),
                                            PatternSymbol::Atom(
                                                3,
                                            ),
                                            PatternSymbol::Atom(
                                                4,
                                            ),
                                            PatternSymbol::Atom(
                                                5,
                                            ),
                                            PatternSymbol::Atom(
                                                6,
                                            ),
                                            PatternSymbol::Atom(
                                                7,
                                            ),
                                            PatternSymbol::Atom(
                                                8,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `downarc`,
                                                0,
                                            ),
                                        ],
                                        [
                                            (
                                                `uparc`,
                                                1,
                                            ),
                                        ],
                                        [
                                            (
                                                `back`,
                                                2,
                                            ),
                                        ],
                                        [
                                            (
                                                `some`,
                                                3,
                                            ),
                                        ],
                                        [
                                            (
                                                `some`,
                                                4,
                                            ),
                                        ],
                                        [
                                            (
                                                `de`,
                                                5,
                                            ),
                                        ],
                                        [
                                            (
                                                `downarc_enpoint`,
                                                6,
                                            ),
                                        ],
                                        [
                                            (
                                                `uparc_startpoint`,
                                                7,
                                            ),
                                        ],
                                        [
                                            (
                                                `distance`,
                                                8,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
                                            None,
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
                                                    52,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            158,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `downarc`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    61,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            158,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `uparc`,
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    70,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            158,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `back`,
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    81,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            158,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `some`,
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    91,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            158,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `some`,
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    93,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            158,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `de`,
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    115,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            158,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `downarc_enpoint`,
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    123,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            158,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `uparc_startpoint`,
                                                    pattern_symbol_idx: 7,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    131,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            158,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `distance`,
                                                    pattern_symbol_idx: 8,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 3,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 7,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 11,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 15,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 19,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 21,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 25,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 27,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 31,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 39,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 41,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 43,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 46,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 49,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 53,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 58,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 59,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 60,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Fn(
                    FnDefn {
                        path: FugitivePath(`mnist_classifier::digits::three::uparc`, `Fn`),
                        decl: FnDecl {
                            path: FugitivePath(`mnist_classifier::digits::three::uparc`, `Fn`),
                            implicit_parameters: [],
                            explicit_parameters: [
                                ExplicitParameterDecl::Regular {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        0..1,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            162,
                                        ),
                                    ),
                                    ty: 1,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeExprBeforeColon {
                                    expr: 3,
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
                                                        path: FugitivePath(`mnist_classifier::digits::three::uparc`, `Fn`),
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    163,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    167,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConcaveComponent`,
                                                        token_idx: TokenIdx(
                                                            164,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            168,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
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
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            161,
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
                                                    `cc`,
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
                                                        162,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                        ident: `cc`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 0,
                                                    ty_expr_idx: 1,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 1,
                                        },
                                        ExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 3,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            10,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntityNodePath::ModuleItem(
                                                    ModuleItemNodePath::Fugitive(
                                                        FugitiveNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::digits::three::uparc`, `Fn`),
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
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            163,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            167,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `ConcaveComponent`,
                                                                token_idx: TokenIdx(
                                                                    164,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    168,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
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
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    161,
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
                                                            `cc`,
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
                                                                162,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                                ident: `cc`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 0,
                                                            ty_expr_idx: 1,
                                                        },
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 1,
                                                },
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 3,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::three::uparc`, `Fn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                173,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                174,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    175,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                176,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                177,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                179,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                180,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    181,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                183,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                182,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                185,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                186,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    187,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 7,
                                            dot_token_idx: TokenIdx(
                                                188,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    189,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                190,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                191,
                                            ),
                                        },
                                        Expr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                184,
                                            ),
                                            opd: 8,
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..3,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    170,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        172,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 1,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    178,
                                                ),
                                            },
                                            condition: 5,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 9,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        171,
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
                                        Let,
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
                                                `dp`,
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
                                        data: [
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: None,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `cc`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    172,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            192,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `dp`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 1,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 9,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 10,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Fn(
                    FnDefn {
                        path: FugitivePath(`mnist_classifier::digits::three::downarc`, `Fn`),
                        decl: FnDecl {
                            path: FugitivePath(`mnist_classifier::digits::three::downarc`, `Fn`),
                            implicit_parameters: [],
                            explicit_parameters: [
                                ExplicitParameterDecl::Regular {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        0..1,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            196,
                                        ),
                                    ),
                                    ty: 1,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeExprBeforeColon {
                                    expr: 3,
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
                                                        path: FugitivePath(`mnist_classifier::digits::three::downarc`, `Fn`),
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    197,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    201,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConcaveComponent`,
                                                        token_idx: TokenIdx(
                                                            198,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            202,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
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
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            195,
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
                                                    `cc`,
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
                                                        196,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                        ident: `cc`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 0,
                                                    ty_expr_idx: 1,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 1,
                                        },
                                        ExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 3,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            10,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntityNodePath::ModuleItem(
                                                    ModuleItemNodePath::Fugitive(
                                                        FugitiveNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::digits::three::downarc`, `Fn`),
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
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            197,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            201,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `ConcaveComponent`,
                                                                token_idx: TokenIdx(
                                                                    198,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    202,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
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
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    195,
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
                                                            `cc`,
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
                                                                196,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                                ident: `cc`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 0,
                                                            ty_expr_idx: 1,
                                                        },
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 1,
                                                },
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 3,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::three::downarc`, `Fn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                207,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                208,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    209,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                210,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                211,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                213,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                214,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    215,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                217,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                216,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                219,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                220,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    221,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 7,
                                            dot_token_idx: TokenIdx(
                                                222,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    223,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                224,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                225,
                                            ),
                                        },
                                        Expr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                218,
                                            ),
                                            opd: 8,
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..3,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    204,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        206,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 1,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    212,
                                                ),
                                            },
                                            condition: 5,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 9,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        205,
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
                                        Let,
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
                                                `dp`,
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
                                        data: [
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: None,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `cc`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    206,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            226,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `dp`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 1,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 9,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 10,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Fn(
                    FnDefn {
                        path: FugitivePath(`mnist_classifier::digits::three::back`, `Fn`),
                        decl: FnDecl {
                            path: FugitivePath(`mnist_classifier::digits::three::back`, `Fn`),
                            implicit_parameters: [],
                            explicit_parameters: [
                                ExplicitParameterDecl::Regular {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        0..1,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            230,
                                        ),
                                    ),
                                    ty: 1,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeExprBeforeColon {
                                    expr: 3,
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
                                                        path: FugitivePath(`mnist_classifier::digits::three::back`, `Fn`),
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
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    231,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                opt_path: Some(
                                                    PrincipalEntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    235,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    principal_entity_path_expr_arena: Arena {
                                        data: [
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConcaveComponent`,
                                                        token_idx: TokenIdx(
                                                            232,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            PrincipalEntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            236,
                                                        ),
                                                    },
                                                ),
                                                principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`core::num::f32`, `Extern`),
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
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            229,
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
                                                    `cc`,
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
                                                        230,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                        ident: `cc`,
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        pattern_ty_constraints: [
                                            (
                                                ExplicitRegularParameter {
                                                    pattern_expr_idx: 0,
                                                    ty_expr_idx: 1,
                                                },
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: ExplicitParameterType,
                                            expr_idx: 1,
                                        },
                                        ExprRoot {
                                            kind: ReturnType,
                                            expr_idx: 3,
                                        },
                                    ],
                                },
                            },
                        },
                        body: Some(
                            10,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: Some(
                                    ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                EntityNodePath::ModuleItem(
                                                    ModuleItemNodePath::Fugitive(
                                                        FugitiveNodePath {
                                                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                                                path: FugitivePath(`mnist_classifier::digits::three::back`, `Fn`),
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
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            231,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 1,
                                                        opt_path: Some(
                                                            PrincipalEntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            235,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            principal_entity_path_expr_arena: Arena {
                                                data: [
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `ConcaveComponent`,
                                                                token_idx: TokenIdx(
                                                                    232,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    PrincipalEntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    236,
                                                                ),
                                                            },
                                                        ),
                                                        principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::f32`, `Extern`),
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
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    229,
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
                                                            `cc`,
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
                                                                230,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitRegularParameter {
                                                                ident: `cc`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    (
                                                        ExplicitRegularParameter {
                                                            pattern_expr_idx: 0,
                                                            ty_expr_idx: 1,
                                                        },
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ExplicitParameterType,
                                                    expr_idx: 1,
                                                },
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr_idx: 3,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::three::back`, `Fn`),
                                                    disambiguator: 0,
                                                },
                                            },
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                241,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                242,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    243,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                244,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                245,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                247,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                248,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    249,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                251,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                250,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                253,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                254,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    255,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 7,
                                            dot_token_idx: TokenIdx(
                                                256,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    257,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                258,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                259,
                                            ),
                                        },
                                        Expr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                252,
                                            ),
                                            opd: 8,
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..3,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    238,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon_token: Ok(
                                                        None,
                                                    ),
                                                    ty: None,
                                                },
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        240,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 1,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    246,
                                                ),
                                            },
                                            condition: 5,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 9,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                symbol_modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        239,
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
                                        Let,
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
                                                `dp`,
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
                                        data: [
                                            InheritedSymbol {
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                modifier: None,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `cc`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: None,
                                                access_start: TokenIdx(
                                                    240,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            260,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `dp`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 1,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 9,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 10,
                                    },
                                ],
                            },
                        },
                    },
                ),
            ),
        ),
    ],
)