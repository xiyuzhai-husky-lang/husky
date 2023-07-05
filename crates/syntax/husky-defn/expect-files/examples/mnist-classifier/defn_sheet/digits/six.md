Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Val(
                    ValDefn {
                        path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                        decl: ValDecl {
                            path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                            var_ty: Some(
                                FormTypeExpr {
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
                                                        path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `FermiMatchResult`,
                                                        token_idx: TokenIdx(
                                                            7,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
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
                        body: Some(
                            5,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
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
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 2,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 1,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::List {
                                            lbox_token_idx: TokenIdx(
                                                13,
                                            ),
                                            items: ArenaIdxRange(
                                                1..2,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                15,
                                            ),
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                10,
                                            ),
                                            items: ArenaIdxRange(
                                                2..4,
                                            ),
                                            commas: [
                                                TokenIdx(
                                                    12,
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                16,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `fermi_match`,
                                                    token_idx: TokenIdx(
                                                        9,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_concave_components`,
                                                    token_idx: TokenIdx(
                                                        11,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr_idx: 4,
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
                                        expr_idx: 4,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 5,
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
                        path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                        decl: ValDecl {
                            path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                            var_ty: Some(
                                FormTypeExpr {
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
                                                        path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `FermiMatchResult`,
                                                        token_idx: TokenIdx(
                                                            20,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
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
                        body: Some(
                            6,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
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
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 2,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 3,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 1,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::List {
                                            lbox_token_idx: TokenIdx(
                                                26,
                                            ),
                                            items: ArenaIdxRange(
                                                1..3,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                30,
                                            ),
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                23,
                                            ),
                                            items: ArenaIdxRange(
                                                3..5,
                                            ),
                                            commas: [
                                                TokenIdx(
                                                    25,
                                                ),
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                31,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `fermi_match`,
                                                    token_idx: TokenIdx(
                                                        22,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_concave_components`,
                                                    token_idx: TokenIdx(
                                                        24,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        27,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `bottom1`,
                                                    token_idx: TokenIdx(
                                                        29,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr_idx: 5,
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
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 6,
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
                        path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                        decl: ValDecl {
                            path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                            var_ty: Some(
                                FormTypeExpr {
                                    expr: 1,
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
                                                        path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    39,
                                                ),
                                                opd: 0,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `MnistLabel`,
                                                        token_idx: TokenIdx(
                                                            40,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
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
                        body: Some(
                            120,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
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
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Be {
                                            src: 0,
                                            be_token_idx: TokenIdx(
                                                44,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
                                                    pattern_expr: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                },
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 1,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                50,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    51,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                53,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 3,
                                            lbox_token_idx: TokenIdx(
                                                52,
                                            ),
                                            items: ArenaIdxRange(
                                                4..5,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                54,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upmost_match`,
                                            token_idx: TokenIdx(
                                                56,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::Be {
                                            src: 6,
                                            be_token_idx: TokenIdx(
                                                57,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
                                                    pattern_expr: 2,
                                                    variables: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                },
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 2,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 8,
                                            dot_token_idx: TokenIdx(
                                                63,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    64,
                                                ),
                                            },
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 3,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 4,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 10,
                                            dot_token_idx: TokenIdx(
                                                69,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    70,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 11,
                                            dot_token_idx: TokenIdx(
                                                73,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    74,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 12,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                71,
                                            ),
                                            ropd: 13,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `eff_holes`,
                                            token_idx: TokenIdx(
                                                76,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 15,
                                            dot_token_idx: TokenIdx(
                                                77,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    78,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                80,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 16,
                                            lbox_token_idx: TokenIdx(
                                                79,
                                            ),
                                            items: ArenaIdxRange(
                                                17..18,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                81,
                                            ),
                                        },
                                        Expr::Be {
                                            src: 18,
                                            be_token_idx: TokenIdx(
                                                82,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
                                                    pattern_expr: 5,
                                                    variables: ArenaIdxRange(
                                                        5..6,
                                                    ),
                                                },
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 5,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 6,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 7,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                97,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 21,
                                            dot_token_idx: TokenIdx(
                                                88,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    89,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 22,
                                            dot_token_idx: TokenIdx(
                                                92,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    93,
                                                ),
                                            },
                                        },
                                        Expr::FunctionCall {
                                            function: 20,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                86,
                                            ),
                                            items: [
                                                CallListItem {
                                                    kind: Argument,
                                                    separator: Comma(
                                                        TokenIdx(
                                                            90,
                                                        ),
                                                    ),
                                                    argument_expr_idx: 24,
                                                },
                                                CallListItem {
                                                    kind: Argument,
                                                    separator: Comma(
                                                        TokenIdx(
                                                            94,
                                                        ),
                                                    ),
                                                    argument_expr_idx: 25,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                98,
                                            ),
                                        },
                                        Expr::Suffix {
                                            opd: 26,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                99,
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 8,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 28,
                                            dot_token_idx: TokenIdx(
                                                104,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    105,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                107,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 29,
                                            lbox_token_idx: TokenIdx(
                                                106,
                                            ),
                                            items: ArenaIdxRange(
                                                30..31,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                108,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `bottom1_match`,
                                            token_idx: TokenIdx(
                                                112,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 6,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 32,
                                            dot_token_idx: TokenIdx(
                                                113,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    114,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                115,
                                            ),
                                            items: ArenaIdxRange(
                                                33..33,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                116,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `bottom1_match_dp`,
                                            token_idx: TokenIdx(
                                                120,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 34,
                                            dot_token_idx: TokenIdx(
                                                121,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    122,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upmost_match`,
                                            token_idx: TokenIdx(
                                                126,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 36,
                                            dot_token_idx: TokenIdx(
                                                127,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    128,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                129,
                                            ),
                                            items: ArenaIdxRange(
                                                37..37,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                130,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 37,
                                            dot_token_idx: TokenIdx(
                                                131,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    132,
                                                ),
                                            },
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 9,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 39,
                                            dot_token_idx: TokenIdx(
                                                137,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `others`,
                                                token_idx: TokenIdx(
                                                    138,
                                                ),
                                            },
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 10,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 41,
                                            dot_token_idx: TokenIdx(
                                                141,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    142,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                144,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 42,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                143,
                                            ),
                                            ropd: 43,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `bottom1_match`,
                                            token_idx: TokenIdx(
                                                146,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 6,
                                            },
                                        },
                                        Expr::Be {
                                            src: 45,
                                            be_token_idx: TokenIdx(
                                                147,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
                                                    pattern_expr: 11,
                                                    variables: ArenaIdxRange(
                                                        11..12,
                                                    ),
                                                },
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                154,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `bottom1_match_dp_y`,
                                            token_idx: TokenIdx(
                                                151,
                                            ),
                                            current_symbol_idx: 8,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 8,
                                            },
                                        },
                                        Expr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                153,
                                            ),
                                            opd: 47,
                                        },
                                        Expr::Binary {
                                            lopd: 48,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                152,
                                            ),
                                            ropd: 49,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 11,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                158,
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
                                                157,
                                            ),
                                            ropd: 52,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 12,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                165,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `upmost_match_dp_y`,
                                            token_idx: TokenIdx(
                                                161,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 9,
                                            },
                                        },
                                        Expr::FunctionCall {
                                            function: 54,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                160,
                                            ),
                                            items: [
                                                CallListItem {
                                                    kind: Argument,
                                                    separator: Comma(
                                                        TokenIdx(
                                                            162,
                                                        ),
                                                    ),
                                                    argument_expr_idx: 56,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                166,
                                            ),
                                        },
                                        Expr::Suffix {
                                            opd: 57,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                167,
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 14,
                                            path: Some(
                                                EntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Six`,
                                                    },
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 15,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 60,
                                            dot_token_idx: TokenIdx(
                                                176,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    177,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upmost_match`,
                                            token_idx: TokenIdx(
                                                181,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 62,
                                            dot_token_idx: TokenIdx(
                                                182,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    183,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                184,
                                            ),
                                            items: ArenaIdxRange(
                                                63..63,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                185,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 61,
                                            dot_token_idx: TokenIdx(
                                                178,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_point`,
                                                token_idx: TokenIdx(
                                                    179,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                180,
                                            ),
                                            items: ArenaIdxRange(
                                                63..64,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                186,
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 16,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 18,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                199,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                204,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::CurrentSymbol {
                                            ident: `upmost_match_dp_y`,
                                            token_idx: TokenIdx(
                                                189,
                                            ),
                                            current_symbol_idx: 9,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 9,
                                            },
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 17,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `lower_excess`,
                                            token_idx: TokenIdx(
                                                193,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 66,
                                            dot_token_idx: TokenIdx(
                                                196,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `top_k_row_span_sum`,
                                                token_idx: TokenIdx(
                                                    197,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                198,
                                            ),
                                            items: ArenaIdxRange(
                                                67..68,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                200,
                                            ),
                                        },
                                        Expr::FunctionCall {
                                            function: 65,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                188,
                                            ),
                                            items: [
                                                CallListItem {
                                                    kind: Argument,
                                                    separator: Comma(
                                                        TokenIdx(
                                                            190,
                                                        ),
                                                    ),
                                                    argument_expr_idx: 69,
                                                },
                                                CallListItem {
                                                    kind: Argument,
                                                    separator: Comma(
                                                        TokenIdx(
                                                            192,
                                                        ),
                                                    ),
                                                    argument_expr_idx: 70,
                                                },
                                                CallListItem {
                                                    kind: Argument,
                                                    separator: Comma(
                                                        TokenIdx(
                                                            194,
                                                        ),
                                                    ),
                                                    argument_expr_idx: 71,
                                                },
                                                CallListItem {
                                                    kind: Argument,
                                                    separator: Comma(
                                                        TokenIdx(
                                                            201,
                                                        ),
                                                    ),
                                                    argument_expr_idx: 72,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                205,
                                            ),
                                        },
                                        Expr::Suffix {
                                            opd: 73,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                206,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `rel_upmost_match_end`,
                                            token_idx: TokenIdx(
                                                208,
                                            ),
                                            current_symbol_idx: 12,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 75,
                                            dot_token_idx: TokenIdx(
                                                209,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    210,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                212,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 76,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                211,
                                            ),
                                            ropd: 77,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 20,
                                            path: Some(
                                                EntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Six`,
                                                    },
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 22,
                                            path: Some(
                                                EntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Six`,
                                                    },
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 23,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 24,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                230,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 82,
                                            dot_token_idx: TokenIdx(
                                                225,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    226,
                                                ),
                                            },
                                        },
                                        Expr::FunctionCall {
                                            function: 81,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                223,
                                            ),
                                            items: [
                                                CallListItem {
                                                    kind: Argument,
                                                    separator: Comma(
                                                        TokenIdx(
                                                            227,
                                                        ),
                                                    ),
                                                    argument_expr_idx: 84,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                231,
                                            ),
                                        },
                                        Expr::Suffix {
                                            opd: 85,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                232,
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 25,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 87,
                                            dot_token_idx: TokenIdx(
                                                235,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    236,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                238,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 88,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                237,
                                            ),
                                            ropd: 89,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 26,
                                            path: Some(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 91,
                                            dot_token_idx: TokenIdx(
                                                242,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    243,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                245,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 92,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                244,
                                            ),
                                            ropd: 93,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `eff_holes`,
                                            token_idx: TokenIdx(
                                                247,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 95,
                                            dot_token_idx: TokenIdx(
                                                248,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    249,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                251,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 96,
                                            lbox_token_idx: TokenIdx(
                                                250,
                                            ),
                                            items: ArenaIdxRange(
                                                97..98,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                252,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 98,
                                            dot_token_idx: TokenIdx(
                                                253,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    254,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 99,
                                            dot_token_idx: TokenIdx(
                                                255,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    256,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                257,
                                            ),
                                            items: ArenaIdxRange(
                                                100..100,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                258,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                260,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 100,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                259,
                                            ),
                                            ropd: 101,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `eff_holes`,
                                            token_idx: TokenIdx(
                                                262,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 103,
                                            dot_token_idx: TokenIdx(
                                                263,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    264,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                266,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 104,
                                            lbox_token_idx: TokenIdx(
                                                265,
                                            ),
                                            items: ArenaIdxRange(
                                                105..106,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                267,
                                            ),
                                        },
                                        Expr::Be {
                                            src: 106,
                                            be_token_idx: TokenIdx(
                                                268,
                                            ),
                                            target: Ok(
                                                BeVariablesPattern {
                                                    pattern_expr: 13,
                                                    variables: ArenaIdxRange(
                                                        13..14,
                                                    ),
                                                },
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `eff_holes`,
                                            token_idx: TokenIdx(
                                                272,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 3,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 108,
                                            dot_token_idx: TokenIdx(
                                                273,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    274,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                276,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 109,
                                            lbox_token_idx: TokenIdx(
                                                275,
                                            ),
                                            items: ArenaIdxRange(
                                                110..111,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                277,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 111,
                                            dot_token_idx: TokenIdx(
                                                278,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    279,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 112,
                                            dot_token_idx: TokenIdx(
                                                280,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    281,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                282,
                                            ),
                                            items: ArenaIdxRange(
                                                113..113,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                283,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                285,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 113,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                284,
                                            ),
                                            ropd: 114,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `lower_excess`,
                                            token_idx: TokenIdx(
                                                287,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                289,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 116,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                288,
                                            ),
                                            ropd: 117,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 28,
                                            path: Some(
                                                EntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Six`,
                                                    },
                                                ),
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                19..31,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_one`,
                                                    token_idx: TokenIdx(
                                                        43,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `six_match`,
                                                    token_idx: TokenIdx(
                                                        49,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        62,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        68,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        72,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        85,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `six_match`,
                                                    token_idx: TokenIdx(
                                                        87,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `six_match_refined1`,
                                                    token_idx: TokenIdx(
                                                        91,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `six_match_refined1`,
                                                    token_idx: TokenIdx(
                                                        103,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `six_match_refined1`,
                                                    token_idx: TokenIdx(
                                                        136,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `six_match_refined1`,
                                                    token_idx: TokenIdx(
                                                        140,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::six::six_match_refined1`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `ignored_connected_components_row_span_sum_sum`,
                                                    token_idx: TokenIdx(
                                                        156,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        159,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `MnistLabel`,
                                                    token_idx: TokenIdx(
                                                        169,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Subentity {
                                            parent: 13,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    170,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Six`,
                                                    token_idx: TokenIdx(
                                                        171,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                EntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Six`,
                                                    },
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        175,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        187,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `ignored_connected_components_row_span_sum_sum`,
                                                    token_idx: TokenIdx(
                                                        191,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        195,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `MnistLabel`,
                                                    token_idx: TokenIdx(
                                                        215,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Subentity {
                                            parent: 19,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    216,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Six`,
                                                    token_idx: TokenIdx(
                                                        217,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                EntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Six`,
                                                    },
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `MnistLabel`,
                                                    token_idx: TokenIdx(
                                                        219,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Subentity {
                                            parent: 21,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    220,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Six`,
                                                    token_idx: TokenIdx(
                                                        221,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                EntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Six`,
                                                    },
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        222,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `six_match`,
                                                    token_idx: TokenIdx(
                                                        224,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `six_match`,
                                                    token_idx: TokenIdx(
                                                        234,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `six_match`,
                                                    token_idx: TokenIdx(
                                                        241,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::six::six_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `MnistLabel`,
                                                    token_idx: TokenIdx(
                                                        290,
                                                    ),
                                                },
                                            ),
                                            entity_path: EntityPath::ModuleItem(
                                                ModuleItemPath::Type(
                                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                                ),
                                            ),
                                        },
                                        EntityPathExpr::Subentity {
                                            parent: 27,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    291,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Six`,
                                                    token_idx: TokenIdx(
                                                        292,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                EntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Six`,
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
                                                    150,
                                                ),
                                            },
                                            condition: 50,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    155,
                                                ),
                                            },
                                            condition: 53,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 58,
                                        },
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    168,
                                                ),
                                            },
                                            result: 59,
                                        },
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    214,
                                                ),
                                            },
                                            result: 79,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 27,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    100,
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
                                                        102,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 31,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    109,
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
                                                        111,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 33,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    117,
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
                                                        119,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 35,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    123,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 9,
                                                    variables: ArenaIdxRange(
                                                        9..10,
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
                                                        125,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 38,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    133,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 10,
                                                    variables: ArenaIdxRange(
                                                        10..11,
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
                                                        135,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 40,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    139,
                                                ),
                                            },
                                            condition: 44,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        145,
                                                    ),
                                                },
                                                condition: Ok(
                                                    46,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                149,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        0..4,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    172,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 12,
                                                    variables: ArenaIdxRange(
                                                        12..13,
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
                                                        174,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 64,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 74,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        207,
                                                    ),
                                                },
                                                condition: Ok(
                                                    78,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                213,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        4..5,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    218,
                                                ),
                                            },
                                            result: 80,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    240,
                                                ),
                                            },
                                            condition: 94,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    271,
                                                ),
                                            },
                                            condition: 115,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    42,
                                                ),
                                            },
                                            condition: 1,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    46,
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
                                                        48,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 5,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    55,
                                                ),
                                            },
                                            condition: 7,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    59,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 3,
                                                    variables: ArenaIdxRange(
                                                        3..4,
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
                                            initial_value: 9,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    65,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariableDecls {
                                                    pattern_expr_idx: 4,
                                                    variables: ArenaIdxRange(
                                                        4..5,
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
                                                        67,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 14,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        75,
                                                    ),
                                                },
                                                condition: Ok(
                                                    19,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                84,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        5..17,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 86,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        233,
                                                    ),
                                                },
                                                condition: Ok(
                                                    90,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                239,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        17..18,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    246,
                                                ),
                                            },
                                            condition: 102,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        261,
                                                    ),
                                                },
                                                condition: Ok(
                                                    107,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                270,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        18..19,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    286,
                                                ),
                                            },
                                            condition: 118,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 119,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        45,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `upmost_match`,
                                                    token_idx: TokenIdx(
                                                        47,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        58,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `eff_holes`,
                                                    token_idx: TokenIdx(
                                                        60,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `lower_excess`,
                                                    token_idx: TokenIdx(
                                                        66,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        83,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `bottom1_match`,
                                                    token_idx: TokenIdx(
                                                        101,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `bottom1_match_dp`,
                                                    token_idx: TokenIdx(
                                                        110,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `bottom1_match_dp_y`,
                                                    token_idx: TokenIdx(
                                                        118,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `upmost_match_dp_y`,
                                                    token_idx: TokenIdx(
                                                        124,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `others`,
                                                    token_idx: TokenIdx(
                                                        134,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        148,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `rel_upmost_match_end`,
                                                    token_idx: TokenIdx(
                                                        173,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        269,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
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
                                            PatternSymbol::Atom(
                                                9,
                                            ),
                                            PatternSymbol::Atom(
                                                10,
                                            ),
                                            PatternSymbol::Atom(
                                                11,
                                            ),
                                            PatternSymbol::Atom(
                                                12,
                                            ),
                                            PatternSymbol::Atom(
                                                13,
                                            ),
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `none`,
                                                0,
                                            ),
                                        ],
                                        [
                                            (
                                                `upmost_match`,
                                                1,
                                            ),
                                        ],
                                        [
                                            (
                                                `some`,
                                                2,
                                            ),
                                        ],
                                        [
                                            (
                                                `eff_holes`,
                                                3,
                                            ),
                                        ],
                                        [
                                            (
                                                `lower_excess`,
                                                4,
                                            ),
                                        ],
                                        [
                                            (
                                                `none`,
                                                5,
                                            ),
                                        ],
                                        [
                                            (
                                                `bottom1_match`,
                                                6,
                                            ),
                                        ],
                                        [
                                            (
                                                `bottom1_match_dp`,
                                                7,
                                            ),
                                        ],
                                        [
                                            (
                                                `bottom1_match_dp_y`,
                                                8,
                                            ),
                                        ],
                                        [
                                            (
                                                `upmost_match_dp_y`,
                                                9,
                                            ),
                                        ],
                                        [
                                            (
                                                `others`,
                                                10,
                                            ),
                                        ],
                                        [
                                            (
                                                `some`,
                                                11,
                                            ),
                                        ],
                                        [
                                            (
                                                `rel_upmost_match_end`,
                                                12,
                                            ),
                                        ],
                                        [
                                            (
                                                `some`,
                                                13,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
                                            Pure,
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
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    46,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            293,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `none`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    48,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            293,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `upmost_match`,
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    59,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            293,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `some`,
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    61,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            293,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `eff_holes`,
                                                    pattern_symbol_idx: 3,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    67,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            293,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `lower_excess`,
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    84,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            222,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `none`,
                                                    pattern_symbol_idx: 5,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    102,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            222,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `bottom1_match`,
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    111,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            222,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `bottom1_match_dp`,
                                                    pattern_symbol_idx: 7,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    119,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            222,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `bottom1_match_dp_y`,
                                                    pattern_symbol_idx: 8,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    125,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            222,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `upmost_match_dp_y`,
                                                    pattern_symbol_idx: 9,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    135,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            222,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `others`,
                                                    pattern_symbol_idx: 10,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    149,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            172,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `some`,
                                                    pattern_symbol_idx: 11,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    174,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            222,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `rel_upmost_match_end`,
                                                    pattern_symbol_idx: 12,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    270,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            286,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `some`,
                                                    pattern_symbol_idx: 13,
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
                                        expr_idx: 1,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 7,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 9,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 14,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 27,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 31,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 33,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 35,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 38,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 40,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 44,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 50,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 53,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 58,
                                    },
                                    ExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 59,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 64,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 74,
                                    },
                                    ExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 79,
                                    },
                                    ExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 80,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 86,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 94,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 102,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 115,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 118,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 119,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 120,
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
                        path: FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                        decl: FnDecl {
                            path: FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
                            implicit_parameters: [],
                            regular_parameters: [
                                RegularParameterDeclPattern {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        0..1,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            297,
                                        ),
                                    ),
                                    ty: 1,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeExpr {
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
                                                        path: FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
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
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    298,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    302,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConcaveComponent`,
                                                        token_idx: TokenIdx(
                                                            299,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            303,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
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
                                                    modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            296,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                Pure,
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
                                                Pure,
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
                                                    modifier: Pure,
                                                    access_start: TokenIdx(
                                                        297,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitParameter {
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
                                                ExplicitParameter {
                                                    pattern_expr: 0,
                                                    ty: 1,
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
                            8,
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
                                                                path: FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
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
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            298,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            302,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `ConcaveComponent`,
                                                                token_idx: TokenIdx(
                                                                    299,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    303,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
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
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    296,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        Pure,
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
                                                        Pure,
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
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                297,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
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
                                                        ExplicitParameter {
                                                            pattern_expr: 0,
                                                            ty: 1,
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
                                                    path: FugitivePath(`mnist_classifier::digits::six::upmost`, `Fn`),
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
                                                308,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                309,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    310,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                311,
                                            ),
                                            items: ArenaIdxRange(
                                                1..1,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                312,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                314,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                315,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    316,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                318,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                317,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                319,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 6,
                                            dot_token_idx: TokenIdx(
                                                320,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    321,
                                                ),
                                            },
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                0..3,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    305,
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
                                                        307,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 1,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    313,
                                                ),
                                            },
                                            condition: 5,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 7,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        306,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            Pure,
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
                                            Pure,
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
                                                modifier: Pure,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `cc`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    307,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            322,
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
                                        expr_idx: 7,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 8,
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
                        path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                        decl: FnDecl {
                            path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
                            implicit_parameters: [],
                            regular_parameters: [
                                RegularParameterDeclPattern {
                                    pattern: 0,
                                    variables: ArenaIdxRange(
                                        0..1,
                                    ),
                                    colon: ColonToken(
                                        TokenIdx(
                                            326,
                                        ),
                                    ),
                                    ty: 1,
                                },
                            ],
                            return_ty: Some(
                                ReturnTypeExpr {
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
                                                        path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
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
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Tilde,
                                                opr_token_idx: TokenIdx(
                                                    327,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::PrincipalEntityPath {
                                                entity_path_expr: 1,
                                                path: Some(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`core::num::f32`, `Extern`),
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Expr::Prefix {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    331,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `ConcaveComponent`,
                                                        token_idx: TokenIdx(
                                                            328,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
                                                    ModuleItemPath::Type(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                ),
                                            },
                                            EntityPathExpr::Root {
                                                path_name_token: PathNameToken::Ident(
                                                    IdentToken {
                                                        ident: `f32`,
                                                        token_idx: TokenIdx(
                                                            332,
                                                        ),
                                                    },
                                                ),
                                                entity_path: EntityPath::ModuleItem(
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
                                                    modifier_keyword_group: None,
                                                    ident_token: IdentToken {
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            325,
                                                        ),
                                                    },
                                                },
                                            ],
                                        },
                                        pattern_expr_contracts: ArenaMap {
                                            data: [
                                                Pure,
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
                                                Pure,
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
                                                    modifier: Pure,
                                                    access_start: TokenIdx(
                                                        326,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::ExplicitParameter {
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
                                                ExplicitParameter {
                                                    pattern_expr: 0,
                                                    ty: 1,
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
                            35,
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
                                                                path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
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
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            327,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::PrincipalEntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::f32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            331,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `ConcaveComponent`,
                                                                token_idx: TokenIdx(
                                                                    328,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        path_name_token: PathNameToken::Ident(
                                                            IdentToken {
                                                                ident: `f32`,
                                                                token_idx: TokenIdx(
                                                                    332,
                                                                ),
                                                            },
                                                        ),
                                                        entity_path: EntityPath::ModuleItem(
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
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    325,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_expr_contracts: ArenaMap {
                                                    data: [
                                                        Pure,
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
                                                        Pure,
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
                                                            modifier: Pure,
                                                            access_start: TokenIdx(
                                                                326,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::ExplicitParameter {
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
                                                        ExplicitParameter {
                                                            pattern_expr: 0,
                                                            ty: 1,
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
                                                    path: FugitivePath(`mnist_classifier::digits::six::bottom1`, `Fn`),
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
                                                337,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 0,
                                            dot_token_idx: TokenIdx(
                                                338,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    339,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                340,
                                            ),
                                            items: ArenaIdxRange(
                                                1..1,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                341,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                343,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                348,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                344,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    345,
                                                ),
                                            },
                                        },
                                        Expr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                347,
                                            ),
                                            opd: 3,
                                        },
                                        Expr::Binary {
                                            lopd: 4,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                346,
                                            ),
                                            ropd: 5,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                352,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                356,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 7,
                                            dot_token_idx: TokenIdx(
                                                353,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    354,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 8,
                                            dot_token_idx: TokenIdx(
                                                357,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    358,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 9,
                                            opr: Closed(
                                                Div,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                355,
                                            ),
                                            ropd: 10,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                351,
                                            ),
                                            item: 11,
                                            rpar_token_idx: TokenIdx(
                                                359,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 12,
                                            dot_token_idx: TokenIdx(
                                                360,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `abs`,
                                                token_idx: TokenIdx(
                                                    361,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                362,
                                            ),
                                            items: ArenaIdxRange(
                                                13..13,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                363,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                365,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 13,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                364,
                                            ),
                                            ropd: 14,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                367,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 16,
                                            dot_token_idx: TokenIdx(
                                                368,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    369,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 17,
                                            dot_token_idx: TokenIdx(
                                                370,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    371,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                372,
                                            ),
                                            items: ArenaIdxRange(
                                                18..18,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                373,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                375,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 18,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                374,
                                            ),
                                            ropd: 19,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                379,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 21,
                                            dot_token_idx: TokenIdx(
                                                380,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `line_segment_sketch`,
                                                token_idx: TokenIdx(
                                                    381,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 22,
                                            dot_token_idx: TokenIdx(
                                                382,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    383,
                                                ),
                                            },
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                387,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 24,
                                            dot_token_idx: TokenIdx(
                                                388,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    389,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                390,
                                            ),
                                            items: ArenaIdxRange(
                                                25..25,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                391,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 23,
                                            dot_token_idx: TokenIdx(
                                                384,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_point`,
                                                token_idx: TokenIdx(
                                                    385,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                386,
                                            ),
                                            items: ArenaIdxRange(
                                                25..26,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                392,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `relative_end`,
                                            token_idx: TokenIdx(
                                                394,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 27,
                                            dot_token_idx: TokenIdx(
                                                395,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    396,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                398,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 28,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                397,
                                            ),
                                            ropd: 29,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                400,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                ident: `cc`,
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 31,
                                            dot_token_idx: TokenIdx(
                                                401,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    402,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                403,
                                            ),
                                            items: ArenaIdxRange(
                                                32..32,
                                            ),
                                            commas: [],
                                            rpar_token_idx: TokenIdx(
                                                404,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 32,
                                            dot_token_idx: TokenIdx(
                                                405,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    406,
                                                ),
                                            },
                                        },
                                        Expr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                399,
                                            ),
                                            opd: 33,
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                1..7,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    350,
                                                ),
                                            },
                                            condition: 15,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    334,
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
                                                        336,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 1,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        342,
                                                    ),
                                                },
                                                condition: Ok(
                                                    6,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                349,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    366,
                                                ),
                                            },
                                            condition: 20,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    376,
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
                                                        378,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 26,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    393,
                                                ),
                                            },
                                            condition: 30,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 34,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        335,
                                                    ),
                                                },
                                            },
                                            PatternExpr::Ident {
                                                modifier_keyword_group: None,
                                                ident_token: IdentToken {
                                                    ident: `relative_end`,
                                                    token_idx: TokenIdx(
                                                        377,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                    pattern_expr_contracts: ArenaMap {
                                        data: [
                                            Pure,
                                            Pure,
                                        ],
                                    },
                                    pattern_infos: [
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
                                        ],
                                    },
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                `dp`,
                                                0,
                                            ),
                                        ],
                                        [
                                            (
                                                `relative_end`,
                                                1,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_modifiers: ArenaMap {
                                        data: [
                                            Pure,
                                            Pure,
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
                                                modifier: Pure,
                                                kind: InheritedSymbolKind::ExplicitParameter {
                                                    ident: `cc`,
                                                },
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    336,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            407,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `dp`,
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                modifier: Pure,
                                                access_start: TokenIdx(
                                                    378,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            407,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    ident: `relative_end`,
                                                    pattern_symbol_idx: 1,
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
                                        expr_idx: 15,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 20,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 26,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 30,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 34,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 35,
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