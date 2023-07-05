Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Val(
                    ValDefn {
                        path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                        decl: ValDecl {
                            path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                            val_ty: Some(
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
                                                        FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
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
                                                    comma_token_idx: None,
                                                },
                                            ],
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
                                                    expr_idx: 3,
                                                    comma_token_idx: None,
                                                },
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
                                                    ident: `big_mouth`,
                                                    token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
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
                        path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                        decl: ValDecl {
                            path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                            val_ty: Some(
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
                        body: Some(
                            28,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
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
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Be {
                                            src: 0,
                                            be_token_idx: TokenIdx(
                                                29,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                30,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 1,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Be {
                                            src: 2,
                                            be_token_idx: TokenIdx(
                                                33,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                34,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Be {
                                            src: 4,
                                            be_token_idx: TokenIdx(
                                                37,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                38,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Be {
                                            src: 6,
                                            be_token_idx: TokenIdx(
                                                41,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                42,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 4,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 5,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 8,
                                            dot_token_idx: TokenIdx(
                                                47,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    48,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 9,
                                            dot_token_idx: TokenIdx(
                                                51,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    52,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 10,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                49,
                                            ),
                                            ropd: 11,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 6,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 13,
                                            dot_token_idx: TokenIdx(
                                                55,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    56,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 14,
                                            dot_token_idx: TokenIdx(
                                                57,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    58,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                60,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 15,
                                            lbox_token_idx: TokenIdx(
                                                59,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 16,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                61,
                                            ),
                                        },
                                        Expr::Be {
                                            src: 17,
                                            be_token_idx: TokenIdx(
                                                62,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                63,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 7,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 19,
                                            dot_token_idx: TokenIdx(
                                                67,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    68,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 20,
                                            dot_token_idx: TokenIdx(
                                                69,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    70,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                72,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 21,
                                            lbox_token_idx: TokenIdx(
                                                71,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 22,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                73,
                                            ),
                                        },
                                        Expr::Be {
                                            src: 23,
                                            be_token_idx: TokenIdx(
                                                74,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                75,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                78,
                                            ),
                                            Literal::Bool(
                                                False,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                80,
                                            ),
                                            Literal::Bool(
                                                False,
                                            ),
                                        ),
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 9,
                                            opt_path: Some(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Eight`,
                                                    },
                                                ),
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                3..10,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_one`,
                                                    token_idx: TokenIdx(
                                                        28,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_six`,
                                                    token_idx: TokenIdx(
                                                        32,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_zero`,
                                                    token_idx: TokenIdx(
                                                        36,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_seven`,
                                                    token_idx: TokenIdx(
                                                        40,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        46,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        50,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        54,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        66,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `MnistLabel`,
                                                    token_idx: TokenIdx(
                                                        81,
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
                                            parent: 8,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    82,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Eight`,
                                                    token_idx: TokenIdx(
                                                        83,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Eight`,
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
                                                    77,
                                                ),
                                            },
                                            condition: 25,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        65,
                                                    ),
                                                },
                                                condition: Ok(
                                                    24,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                76,
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
                                                    79,
                                                ),
                                            },
                                            condition: 26,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    27,
                                                ),
                                            },
                                            condition: 1,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    31,
                                                ),
                                            },
                                            condition: 3,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    35,
                                                ),
                                            },
                                            condition: 5,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    39,
                                                ),
                                            },
                                            condition: 7,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    43,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                44,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        45,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 12,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        53,
                                                    ),
                                                },
                                                condition: Ok(
                                                    18,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                64,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        1..3,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 27,
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
                                        kind: Condition,
                                        expr_idx: 1,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 3,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 5,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 7,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 12,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 25,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 26,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 27,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 28,
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