Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Val(
                    ValDefn {
                        path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                        decl: ValDecl {
                            path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                                                        path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                            7,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                                                        FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 4,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
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
                                                    ident: `left_cc_pattern`,
                                                    token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `right_cc_pattern`,
                                                    token_idx: TokenIdx(
                                                        16,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `down_cc_pattern`,
                                                    token_idx: TokenIdx(
                                                        18,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
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
                        path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                        decl: ValDecl {
                            path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
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
                                                        path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
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
                                                    115,
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
                                                            116,
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
                            111,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
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
                                                        FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Be {
                                            src: 0,
                                            be_token_idx: TokenIdx(
                                                120,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                121,
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
                                                        FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Be {
                                            src: 2,
                                            be_token_idx: TokenIdx(
                                                124,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                125,
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
                                                        FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Be {
                                            src: 4,
                                            be_token_idx: TokenIdx(
                                                128,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                129,
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
                                                        FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Be {
                                            src: 6,
                                            be_token_idx: TokenIdx(
                                                132,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                133,
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
                                                        FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Be {
                                            src: 8,
                                            be_token_idx: TokenIdx(
                                                136,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                137,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 5,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Be {
                                            src: 10,
                                            be_token_idx: TokenIdx(
                                                140,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                141,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 6,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 12,
                                            dot_token_idx: TokenIdx(
                                                146,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    147,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                148,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                149,
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
                                            owner: 14,
                                            dot_token_idx: TokenIdx(
                                                154,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    155,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        157,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 230,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 16,
                                            dot_token_idx: TokenIdx(
                                                158,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    159,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                161,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 17,
                                            lbox_token_idx: TokenIdx(
                                                160,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 18,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                162,
                                            ),
                                        },
                                        Expr::Be {
                                            src: 19,
                                            be_token_idx: TokenIdx(
                                                163,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                164,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 8,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 21,
                                            dot_token_idx: TokenIdx(
                                                169,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    170,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                172,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 22,
                                            lbox_token_idx: TokenIdx(
                                                171,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 23,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                173,
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 9,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 25,
                                            dot_token_idx: TokenIdx(
                                                178,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    179,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                181,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 26,
                                            lbox_token_idx: TokenIdx(
                                                180,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 27,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                182,
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 10,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 29,
                                            dot_token_idx: TokenIdx(
                                                187,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    188,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                190,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 30,
                                            lbox_token_idx: TokenIdx(
                                                189,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 31,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                191,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        193,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 517,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                195,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 33,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                194,
                                            ),
                                            ropd: 34,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 11,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 12,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 36,
                                            dot_token_idx: TokenIdx(
                                                200,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    201,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 37,
                                            dot_token_idx: TokenIdx(
                                                204,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    205,
                                                ),
                                            },
                                        },
                                        Expr::Binary {
                                            lopd: 38,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                202,
                                            ),
                                            ropd: 39,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        207,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 466,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                209,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 41,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                208,
                                            ),
                                            ropd: 42,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        211,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 517,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                213,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 44,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                212,
                                            ),
                                            ropd: 45,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        216,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 518,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Be {
                                            src: 47,
                                            be_token_idx: TokenIdx(
                                                217,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                218,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        220,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 519,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Be {
                                            src: 49,
                                            be_token_idx: TokenIdx(
                                                221,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                222,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        226,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 519,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 51,
                                            dot_token_idx: TokenIdx(
                                                227,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    228,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                233,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        230,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 55,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Prefix {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                232,
                                            ),
                                            opd: 53,
                                        },
                                        Expr::Binary {
                                            lopd: 54,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                231,
                                            ),
                                            ropd: 55,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        237,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 518,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 57,
                                            dot_token_idx: TokenIdx(
                                                238,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end_tangent`,
                                                token_idx: TokenIdx(
                                                    239,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                240,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                241,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                245,
                                            ),
                                            Literal::Bool(
                                                True,
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 58,
                                            dot_token_idx: TokenIdx(
                                                242,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle`,
                                                token_idx: TokenIdx(
                                                    243,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                244,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 59,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                246,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        250,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 518,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 61,
                                            dot_token_idx: TokenIdx(
                                                251,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end_tangent`,
                                                token_idx: TokenIdx(
                                                    252,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                253,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                254,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 62,
                                            dot_token_idx: TokenIdx(
                                                255,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `x`,
                                                token_idx: TokenIdx(
                                                    256,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        260,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 518,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 64,
                                            dot_token_idx: TokenIdx(
                                                261,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `end_tangent`,
                                                token_idx: TokenIdx(
                                                    262,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                263,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                264,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 65,
                                            dot_token_idx: TokenIdx(
                                                265,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    266,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        270,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 518,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 67,
                                            dot_token_idx: TokenIdx(
                                                271,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    272,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 68,
                                            dot_token_idx: TokenIdx(
                                                273,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    274,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                275,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                276,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        280,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 518,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 70,
                                            dot_token_idx: TokenIdx(
                                                281,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    282,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 71,
                                            dot_token_idx: TokenIdx(
                                                283,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    284,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                285,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                286,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        291,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 522,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        293,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 523,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 73,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                292,
                                            ),
                                            ropd: 74,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                290,
                                            ),
                                            item: 75,
                                            rpar_token_idx: TokenIdx(
                                                294,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                296,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 76,
                                            opr: Closed(
                                                Div,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                295,
                                            ),
                                            ropd: 77,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        300,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 519,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 79,
                                            dot_token_idx: TokenIdx(
                                                301,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    302,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 80,
                                            dot_token_idx: TokenIdx(
                                                303,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    304,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                305,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                306,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        310,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 519,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 82,
                                            dot_token_idx: TokenIdx(
                                                311,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    312,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 83,
                                            dot_token_idx: TokenIdx(
                                                313,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    314,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                315,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                316,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        321,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 525,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        323,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 526,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 85,
                                            opr: Closed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                322,
                                            ),
                                            ropd: 86,
                                        },
                                        Expr::Bracketed {
                                            lpar_token_idx: TokenIdx(
                                                320,
                                            ),
                                            item: 87,
                                            rpar_token_idx: TokenIdx(
                                                324,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                326,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 88,
                                            opr: Closed(
                                                Div,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                325,
                                            ),
                                            ropd: 89,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        328,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 524,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        330,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 527,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 91,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                329,
                                            ),
                                            ropd: 92,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        332,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 517,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                334,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 94,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                333,
                                            ),
                                            ropd: 95,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        337,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 518,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Be {
                                            src: 97,
                                            be_token_idx: TokenIdx(
                                                338,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                339,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        341,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 519,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Be {
                                            src: 99,
                                            be_token_idx: TokenIdx(
                                                342,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                343,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        345,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 520,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Be {
                                            src: 101,
                                            be_token_idx: TokenIdx(
                                                346,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                347,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        349,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 520,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 103,
                                            dot_token_idx: TokenIdx(
                                                350,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    351,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 104,
                                            dot_token_idx: TokenIdx(
                                                352,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    353,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                354,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                355,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                357,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 105,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                356,
                                            ),
                                            ropd: 106,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        361,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 520,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 108,
                                            dot_token_idx: TokenIdx(
                                                362,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    363,
                                                ),
                                            },
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 14,
                                            opt_path: Some(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Two`,
                                                    },
                                                ),
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                19..37,
                                            ),
                                        },
                                    ],
                                },
                                principal_entity_path_expr_arena: Arena {
                                    data: [
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_zero`,
                                                    token_idx: TokenIdx(
                                                        119,
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
                                                    ident: `is_three`,
                                                    token_idx: TokenIdx(
                                                        123,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::three::is_three`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_seven`,
                                                    token_idx: TokenIdx(
                                                        127,
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
                                                    ident: `is_one`,
                                                    token_idx: TokenIdx(
                                                        131,
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
                                                    ident: `is_nine`,
                                                    token_idx: TokenIdx(
                                                        135,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `is_six`,
                                                    token_idx: TokenIdx(
                                                        139,
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
                                                    ident: `major_concave_components`,
                                                    token_idx: TokenIdx(
                                                        145,
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
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        153,
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
                                                    ident: `two_match`,
                                                    token_idx: TokenIdx(
                                                        168,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `two_match`,
                                                    token_idx: TokenIdx(
                                                        177,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `two_match`,
                                                    token_idx: TokenIdx(
                                                        186,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        199,
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
                                                        203,
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
                                                        364,
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
                                            parent: 13,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    365,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Two`,
                                                    token_idx: TokenIdx(
                                                        366,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Two`,
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
                                                    215,
                                                ),
                                            },
                                            condition: 48,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    219,
                                                ),
                                            },
                                            condition: 50,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    223,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                224,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        225,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 52,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    229,
                                                ),
                                            },
                                            condition: 56,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    234,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                235,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        236,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 60,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    247,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                248,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        249,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 63,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    257,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                258,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        259,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 66,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    267,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                268,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        269,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 69,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    277,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                278,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        279,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 72,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    287,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                288,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        289,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 78,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    297,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                298,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        299,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 81,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    307,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                308,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        309,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 84,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    317,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                318,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        319,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 90,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    327,
                                                ),
                                            },
                                            condition: 93,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    336,
                                                ),
                                            },
                                            condition: 98,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    340,
                                                ),
                                            },
                                            condition: 100,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    344,
                                                ),
                                            },
                                            condition: 102,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    348,
                                                ),
                                            },
                                            condition: 107,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    358,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                359,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        360,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 109,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    118,
                                                ),
                                            },
                                            condition: 1,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    122,
                                                ),
                                            },
                                            condition: 3,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    126,
                                                ),
                                            },
                                            condition: 5,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    130,
                                                ),
                                            },
                                            condition: 7,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    134,
                                                ),
                                            },
                                            condition: 9,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    138,
                                                ),
                                            },
                                            condition: 11,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    142,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                143,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        144,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 13,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    150,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                151,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        152,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 15,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    156,
                                                ),
                                            },
                                            condition: 20,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    165,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                166,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        167,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 24,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    174,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                175,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        176,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 28,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    183,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                184,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        185,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 32,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    192,
                                                ),
                                            },
                                            condition: 35,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    196,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                197,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        198,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 40,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    206,
                                                ),
                                            },
                                            condition: 43,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        210,
                                                    ),
                                                },
                                                condition: Ok(
                                                    46,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                214,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        0..14,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        331,
                                                    ),
                                                },
                                                condition: Ok(
                                                    96,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                335,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        14..19,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 110,
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
                                        kind: Condition,
                                        expr_idx: 9,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 11,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 13,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 15,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 20,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 24,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 28,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 32,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 35,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 40,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 43,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 48,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 50,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 52,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 56,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 60,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 63,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 66,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 69,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 72,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 78,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 81,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 84,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 90,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 93,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 98,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 100,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 102,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 107,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 109,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 110,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 111,
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