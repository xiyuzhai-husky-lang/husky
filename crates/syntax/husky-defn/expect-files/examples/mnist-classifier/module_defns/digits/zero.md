Ok(
    [
        Defn::ModuleItem(
            ModuleItemDefn::Fugitive(
                FugitiveDefn::Val(
                    ValDefn {
                        path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                        decl: ValDecl {
                            path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                                                        path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                                                    path: FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
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
                                                        FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
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
                                                    ident: `almost_closed`,
                                                    token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
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
                        path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                        decl: ValDecl {
                            path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                                                        path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                                                    51,
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
                                                            52,
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
                            89,
                        ),
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    EntityNodePath::ModuleItem(
                                        ModuleItemNodePath::Fugitive(
                                            FugitiveNodePath {
                                                maybe_ambiguous_path: MaybeAmbiguousPath {
                                                    path: FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
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
                                                56,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                57,
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
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 2,
                                            dot_token_idx: TokenIdx(
                                                60,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `raw_contours`,
                                                token_idx: TokenIdx(
                                                    61,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 3,
                                            dot_token_idx: TokenIdx(
                                                62,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    63,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                64,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                65,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                67,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 4,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                66,
                                            ),
                                            ropd: 5,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 2,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 7,
                                            dot_token_idx: TokenIdx(
                                                73,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    74,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        76,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 436,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                78,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 9,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                77,
                                            ),
                                            ropd: 10,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 3,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 12,
                                            dot_token_idx: TokenIdx(
                                                81,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    82,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                84,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 13,
                                            lbox_token_idx: TokenIdx(
                                                83,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 14,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                85,
                                            ),
                                        },
                                        Expr::Be {
                                            src: 15,
                                            be_token_idx: TokenIdx(
                                                86,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                87,
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
                                                        FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 17,
                                            dot_token_idx: TokenIdx(
                                                90,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    91,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                92,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                93,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                95,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 18,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                94,
                                            ),
                                            ropd: 19,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 5,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 21,
                                            dot_token_idx: TokenIdx(
                                                100,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    101,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                103,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 22,
                                            lbox_token_idx: TokenIdx(
                                                102,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 23,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                104,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 24,
                                            dot_token_idx: TokenIdx(
                                                105,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    106,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                107,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                108,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 25,
                                            dot_token_idx: TokenIdx(
                                                109,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    110,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                111,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                112,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        114,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 439,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                116,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 27,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                115,
                                            ),
                                            ropd: 28,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 7,
                                            opt_path: Some(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Zero`,
                                                    },
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 8,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 9,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::List {
                                            lbox_token_idx: TokenIdx(
                                                128,
                                            ),
                                            items: [],
                                            rbox_token_idx: TokenIdx(
                                                129,
                                            ),
                                        },
                                        Expr::FunctionApplicationOrCall {
                                            function: 31,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                125,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 32,
                                                    comma_token_idx: Some(
                                                        TokenIdx(
                                                            127,
                                                        ),
                                                    ),
                                                },
                                                CommaListItem {
                                                    expr_idx: 33,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                130,
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 10,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`malamute::narrow_down`, `Gn`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        133,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 441,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 36,
                                            dot_token_idx: TokenIdx(
                                                134,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    135,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        137,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 441,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 38,
                                            dot_token_idx: TokenIdx(
                                                138,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `rel_norm`,
                                                token_idx: TokenIdx(
                                                    139,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        141,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 441,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 40,
                                            dot_token_idx: TokenIdx(
                                                142,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `angle_change_norm`,
                                                token_idx: TokenIdx(
                                                    143,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                147,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::FunctionCall {
                                            function: 35,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                132,
                                            ),
                                            items: [
                                                CallListItem {
                                                    kind: Argument,
                                                    argument_expr_idx: 37,
                                                    separator: Comma(
                                                        TokenIdx(
                                                            136,
                                                        ),
                                                    ),
                                                },
                                                CallListItem {
                                                    kind: Argument,
                                                    argument_expr_idx: 39,
                                                    separator: Comma(
                                                        TokenIdx(
                                                            140,
                                                        ),
                                                    ),
                                                },
                                                CallListItem {
                                                    kind: Argument,
                                                    argument_expr_idx: 41,
                                                    separator: Comma(
                                                        TokenIdx(
                                                            144,
                                                        ),
                                                    ),
                                                },
                                                CallListItem {
                                                    kind: KeyedArgument {
                                                        key_token_idx: TokenIdx(
                                                            145,
                                                        ),
                                                        key: Ident(
                                                            Word(
                                                                Id {
                                                                    value: 443,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    argument_expr_idx: 42,
                                                    separator: None,
                                                },
                                            ],
                                            rpar_token_idx: TokenIdx(
                                                148,
                                            ),
                                        },
                                        Expr::Suffix {
                                            opd: 43,
                                            opr: UnveilOrComposeWithOption,
                                            opr_token_idx: TokenIdx(
                                                149,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        151,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 441,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 45,
                                            dot_token_idx: TokenIdx(
                                                152,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    153,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                155,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 46,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                154,
                                            ),
                                            ropd: 47,
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
                                        Expr::Field {
                                            owner: 49,
                                            dot_token_idx: TokenIdx(
                                                158,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    159,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 50,
                                            dot_token_idx: TokenIdx(
                                                160,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    161,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                163,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 51,
                                            lbox_token_idx: TokenIdx(
                                                162,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 52,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                164,
                                            ),
                                        },
                                        Expr::Be {
                                            src: 53,
                                            be_token_idx: TokenIdx(
                                                165,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                166,
                                                            ),
                                                            drained: false,
                                                        },
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
                                            owner: 55,
                                            dot_token_idx: TokenIdx(
                                                169,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    170,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 56,
                                            dot_token_idx: TokenIdx(
                                                171,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    172,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                174,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 57,
                                            lbox_token_idx: TokenIdx(
                                                173,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 58,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                175,
                                            ),
                                        },
                                        Expr::Be {
                                            src: 59,
                                            be_token_idx: TokenIdx(
                                                176,
                                            ),
                                            target: Err(
                                                ExprError::Original(
                                                    ExpectedBeVariablesPattern(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                177,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 13,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 61,
                                            dot_token_idx: TokenIdx(
                                                182,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    183,
                                                ),
                                            },
                                        },
                                        Expr::Field {
                                            owner: 62,
                                            dot_token_idx: TokenIdx(
                                                184,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    185,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                187,
                                            ),
                                            Literal::Integer(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::IndexOrCompositionWithList {
                                            owner: 63,
                                            lbox_token_idx: TokenIdx(
                                                186,
                                            ),
                                            items: [
                                                CommaListItem {
                                                    expr_idx: 64,
                                                    comma_token_idx: None,
                                                },
                                            ],
                                            rbox_token_idx: TokenIdx(
                                                188,
                                            ),
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        192,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 444,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 66,
                                            dot_token_idx: TokenIdx(
                                                193,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    194,
                                                ),
                                            },
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        200,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 444,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 68,
                                            dot_token_idx: TokenIdx(
                                                201,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    202,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 67,
                                            dot_token_idx: TokenIdx(
                                                195,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    196,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                197,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                198,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 69,
                                            dot_token_idx: TokenIdx(
                                                203,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    204,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                205,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                206,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 70,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                199,
                                            ),
                                            ropd: 71,
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 14,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 73,
                                            dot_token_idx: TokenIdx(
                                                211,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    212,
                                                ),
                                            },
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 15,
                                            opt_path: Some(
                                                PrincipalEntityPath::ModuleItem(
                                                    ModuleItemPath::Fugitive(
                                                        FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 75,
                                            dot_token_idx: TokenIdx(
                                                219,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `bounding_box`,
                                                token_idx: TokenIdx(
                                                    220,
                                                ),
                                            },
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 74,
                                            dot_token_idx: TokenIdx(
                                                213,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    214,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                215,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                216,
                                            ),
                                        },
                                        Expr::MethodApplicationOrCall {
                                            self_argument: 76,
                                            dot_token_idx: TokenIdx(
                                                221,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    222,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                223,
                                            ),
                                            items: [],
                                            rpar_token_idx: TokenIdx(
                                                224,
                                            ),
                                        },
                                        Expr::Binary {
                                            lopd: 77,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                217,
                                            ),
                                            ropd: 78,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        228,
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
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        230,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 282,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 80,
                                            opr: Closed(
                                                Div,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                229,
                                            ),
                                            ropd: 81,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        232,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 446,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                234,
                                            ),
                                            Literal::Float(
                                                Unspecified,
                                            ),
                                        ),
                                        Expr::Binary {
                                            lopd: 83,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                233,
                                            ),
                                            ropd: 84,
                                        },
                                        Expr::Err(
                                            ExprError::Original(
                                                UnrecognizedIdent {
                                                    token_idx: TokenIdx(
                                                        238,
                                                    ),
                                                    ident: Ident(
                                                        Word(
                                                            Id {
                                                                value: 441,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 86,
                                            dot_token_idx: TokenIdx(
                                                239,
                                            ),
                                            ident_token: IdentToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    240,
                                                ),
                                            },
                                        },
                                        Expr::PrincipalEntityPath {
                                            entity_path_expr: 17,
                                            opt_path: Some(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Zero`,
                                                    },
                                                ),
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                7..21,
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
                                                        55,
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
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        59,
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
                                                    ident: `open_one_match`,
                                                    token_idx: TokenIdx(
                                                        72,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `open_one_match`,
                                                    token_idx: TokenIdx(
                                                        80,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `connected_components`,
                                                    token_idx: TokenIdx(
                                                        89,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::connected_components`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `open_one_match`,
                                                    token_idx: TokenIdx(
                                                        99,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `MnistLabel`,
                                                    token_idx: TokenIdx(
                                                        118,
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
                                            parent: 6,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    119,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Zero`,
                                                    token_idx: TokenIdx(
                                                        120,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Zero`,
                                                    },
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `fermi_match`,
                                                    token_idx: TokenIdx(
                                                        124,
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
                                                        126,
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
                                                    ident: `narrow_down`,
                                                    token_idx: TokenIdx(
                                                        131,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`malamute::narrow_down`, `Gn`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_connected_component`,
                                                    token_idx: TokenIdx(
                                                        157,
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
                                                        168,
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
                                                        181,
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
                                                    ident: `major_line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        210,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `major_line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        218,
                                                    ),
                                                },
                                            ),
                                            principal_entity_path: PrincipalEntityPath::ModuleItem(
                                                ModuleItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                                ),
                                            ),
                                        },
                                        PrincipalEntityPathExpr::Root {
                                            path_name_token: PathNameToken::Ident(
                                                IdentToken {
                                                    ident: `MnistLabel`,
                                                    token_idx: TokenIdx(
                                                        241,
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
                                            parent: 16,
                                            scope_resolution_token: ScopeResolutionToken(
                                                TokenIdx(
                                                    242,
                                                ),
                                            ),
                                            ident_token: Ok(
                                                IdentToken {
                                                    ident: `Zero`,
                                                    token_idx: TokenIdx(
                                                        243,
                                                    ),
                                                },
                                            ),
                                            path: Ok(
                                                PrincipalEntityPath::TypeVariant(
                                                    TypeVariantPath {
                                                        parent_ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ident: `Zero`,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    69,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                70,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        71,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 8,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    75,
                                                ),
                                            },
                                            condition: 11,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    79,
                                                ),
                                            },
                                            condition: 16,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    88,
                                                ),
                                            },
                                            condition: 20,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    96,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                97,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        98,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 26,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    113,
                                                ),
                                            },
                                            condition: 29,
                                        },
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    117,
                                                ),
                                            },
                                            result: 30,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    54,
                                                ),
                                            },
                                            condition: 1,
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        58,
                                                    ),
                                                },
                                                condition: Ok(
                                                    6,
                                                ),
                                                eol_colon: Ok(
                                                    Colon(
                                                        EolColonToken {
                                                            token_idx: TokenIdx(
                                                                68,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        0..7,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    121,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                122,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        123,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 34,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 44,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    150,
                                                ),
                                            },
                                            condition: 48,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    156,
                                                ),
                                            },
                                            condition: 54,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    167,
                                                ),
                                            },
                                            condition: 60,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    178,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                179,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        180,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 65,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    189,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                190,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        191,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 72,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    207,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                208,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        209,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 79,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    225,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                226,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        227,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 82,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    231,
                                                ),
                                            },
                                            condition: 85,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    235,
                                                ),
                                            },
                                            let_variable_pattern: Err(
                                                ExprError::Original(
                                                    ExpectedLetVariableDecls(
                                                        TokenStreamState {
                                                            next_token_idx: TokenIdx(
                                                                236,
                                                            ),
                                                            drained: false,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            assign_token: Ok(
                                                EqToken(
                                                    TokenIdx(
                                                        237,
                                                    ),
                                                ),
                                            ),
                                            initial_value: 87,
                                        },
                                        Stmt::Eval {
                                            expr_idx: 88,
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
                                        kind: LetStmtInitialValue,
                                        expr_idx: 8,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 11,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 16,
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
                                        expr_idx: 29,
                                    },
                                    ExprRoot {
                                        kind: ReturnExpr,
                                        expr_idx: 30,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 34,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 44,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 48,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 54,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 60,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 65,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 72,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 79,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 82,
                                    },
                                    ExprRoot {
                                        kind: Condition,
                                        expr_idx: 85,
                                    },
                                    ExprRoot {
                                        kind: LetStmtInitialValue,
                                        expr_idx: 87,
                                    },
                                    ExprRoot {
                                        kind: EvalExpr,
                                        expr_idx: 88,
                                    },
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr_idx: 89,
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