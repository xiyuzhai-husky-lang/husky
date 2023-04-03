Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Feature(
                            FeatureDefn {
                                path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                decl: FeatureDecl {
                                    path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                    ast_idx: 62,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                6,
                                            ),
                                        ),
                                    ),
                                    var_ty: Some(
                                        FormTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eq_token: EqToken(
                                        TokenIdx(
                                            8,
                                        ),
                                    ),
                                    expr_or_eol_token: Left(
                                        EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    9,
                                                ),
                                            },
                                        ),
                                    ),
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
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
                                                        token_idx: TokenIdx(
                                                            7,
                                                        ),
                                                        ident: `FermiMatchResult`,
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    expr: 0,
                                                },
                                            ],
                                        },
                                    },
                                },
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::downmost`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::upmost`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::hat`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        1..4,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        20,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 0,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        11,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        4..6,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            13,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        21,
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
                                                    token_idx: TokenIdx(
                                                        10,
                                                    ),
                                                    ident: `fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        12,
                                                    ),
                                                    ident: `major_concave_components`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        15,
                                                    ),
                                                    ident: `downmost`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::downmost`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        17,
                                                    ),
                                                    ident: `upmost`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::upmost`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        19,
                                                    ),
                                                    ident: `hat`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::hat`, `Fn`),
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
                                            pattern_infos: [],
                                            pattern_symbol_maps: [],
                                            pattern_symbol_arena: Arena {
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
                                                kind: BlockExpr,
                                                expr: 7,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    7,
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Feature(
                            FeatureDefn {
                                path: FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                decl: FeatureDecl {
                                    path: FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                    ast_idx: 63,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                25,
                                            ),
                                        ),
                                    ),
                                    var_ty: Some(
                                        FormTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                    eq_token: EqToken(
                                        TokenIdx(
                                            28,
                                        ),
                                    ),
                                    expr_or_eol_token: Left(
                                        EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    29,
                                                ),
                                            },
                                        ),
                                    ),
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
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
                                                            26,
                                                        ),
                                                        opd: 0,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            27,
                                                        ),
                                                        ident: `MnistLabel`,
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
                                                pattern_infos: [],
                                                pattern_symbol_maps: [],
                                                pattern_symbol_arena: Arena {
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
                                                    expr: 1,
                                                },
                                            ],
                                        },
                                    },
                                },
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                30,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        36,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 1,
                                                    dot_token_idx: TokenIdx(
                                                        39,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `max_hole_ilen`,
                                                        token_idx: TokenIdx(
                                                            40,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 0,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        31,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        2..6,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            35,
                                                        ),
                                                        TokenIdx(
                                                            37,
                                                        ),
                                                        TokenIdx(
                                                            41,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        43,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 6,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        44,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 5,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        52,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        9..9,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        53,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 8,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        49,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        9..11,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            51,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        54,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `simp_one_match`,
                                                    token_idx: TokenIdx(
                                                        56,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 12,
                                                    dot_token_idx: TokenIdx(
                                                        57,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            58,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        60,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 13,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        59,
                                                    ),
                                                    ropd: 14,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                62,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 8,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 7,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        68,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 17,
                                                    dot_token_idx: TokenIdx(
                                                        71,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `max_row_span`,
                                                        token_idx: TokenIdx(
                                                            72,
                                                        ),
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 16,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        63,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        18..21,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            67,
                                                        ),
                                                        TokenIdx(
                                                            69,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        73,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 21,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        74,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 9,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 23,
                                                    dot_token_idx: TokenIdx(
                                                        77,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `max_row_span`,
                                                        token_idx: TokenIdx(
                                                            78,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        80,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 24,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        79,
                                                    ),
                                                    ropd: 25,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 10,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 27,
                                                    dot_token_idx: TokenIdx(
                                                        84,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `max_hole_ilen`,
                                                        token_idx: TokenIdx(
                                                            85,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        87,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 28,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        86,
                                                    ),
                                                    ropd: 29,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 12,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 13,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 32,
                                                    dot_token_idx: TokenIdx(
                                                        95,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `max_hole_ilen`,
                                                        token_idx: TokenIdx(
                                                            96,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        98,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 33,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        97,
                                                    ),
                                                    ropd: 34,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 14,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        102,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 36,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        101,
                                                    ),
                                                    ropd: 37,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 15,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 39,
                                                    dot_token_idx: TokenIdx(
                                                        107,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            108,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        110,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 40,
                                                    lbox_token_idx: TokenIdx(
                                                        109,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        41..42,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        111,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 16,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 43,
                                                    dot_token_idx: TokenIdx(
                                                        116,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            117,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        119,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 44,
                                                    lbox_token_idx: TokenIdx(
                                                        118,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        45..46,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        120,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 17,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 47,
                                                    dot_token_idx: TokenIdx(
                                                        125,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            126,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        128,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 48,
                                                    lbox_token_idx: TokenIdx(
                                                        127,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        49..50,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        129,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        131,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 51,
                                                    be_token_idx: TokenIdx(
                                                        132,
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
                                                    ident: `simp_one_match`,
                                                    token_idx: TokenIdx(
                                                        136,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 53,
                                                    dot_token_idx: TokenIdx(
                                                        137,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            138,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        140,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 54,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        139,
                                                    ),
                                                    ropd: 55,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                141,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `simp_one_match`,
                                                    token_idx: TokenIdx(
                                                        149,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 58,
                                                    dot_token_idx: TokenIdx(
                                                        150,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `angle_change_norm`,
                                                        token_idx: TokenIdx(
                                                            151,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 19,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        147,
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 59,
                                                    dot_token_idx: TokenIdx(
                                                        152,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `abs`,
                                                        token_idx: TokenIdx(
                                                            153,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        154,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        60..60,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        155,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 57,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        142,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        60..63,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            146,
                                                        ),
                                                        TokenIdx(
                                                            148,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        156,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 63,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        157,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 20,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        159,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 65,
                                                    dot_token_idx: TokenIdx(
                                                        162,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `lower_mass`,
                                                        token_idx: TokenIdx(
                                                            163,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 21,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 66,
                                                    opr: Closed(
                                                        Mul,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        160,
                                                    ),
                                                    ropd: 67,
                                                },
                                                Expr::Field {
                                                    owner: 68,
                                                    dot_token_idx: TokenIdx(
                                                        166,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `upper_mass`,
                                                        token_idx: TokenIdx(
                                                            167,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 69,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        164,
                                                    ),
                                                    ropd: 70,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        169,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 71,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        168,
                                                    ),
                                                    ropd: 72,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 23,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        176,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 75,
                                                    dot_token_idx: TokenIdx(
                                                        177,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            178,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        182,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 77,
                                                    dot_token_idx: TokenIdx(
                                                        183,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            184,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 76,
                                                    dot_token_idx: TokenIdx(
                                                        179,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `end`,
                                                        token_idx: TokenIdx(
                                                            180,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 78,
                                                    dot_token_idx: TokenIdx(
                                                        185,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `start`,
                                                        token_idx: TokenIdx(
                                                            186,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 79,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        181,
                                                    ),
                                                    ropd: 80,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 24,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 82,
                                                    dot_token_idx: TokenIdx(
                                                        189,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            190,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        192,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 83,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        191,
                                                    ),
                                                    ropd: 84,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                193,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 27,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        205,
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 28,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 29,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 30,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 26,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        199,
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 87,
                                                    dot_token_idx: TokenIdx(
                                                        202,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `top_k_row_span_sum`,
                                                        token_idx: TokenIdx(
                                                            203,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        204,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        88..89,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        206,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 89,
                                                    dot_token_idx: TokenIdx(
                                                        209,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            210,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 90,
                                                    dot_token_idx: TokenIdx(
                                                        213,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `rel_norm`,
                                                        token_idx: TokenIdx(
                                                            214,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 91,
                                                    dot_token_idx: TokenIdx(
                                                        217,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `angle_change_norm`,
                                                        token_idx: TokenIdx(
                                                            218,
                                                        ),
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 86,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        194,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        92..98,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            198,
                                                        ),
                                                        TokenIdx(
                                                            200,
                                                        ),
                                                        TokenIdx(
                                                            207,
                                                        ),
                                                        TokenIdx(
                                                            211,
                                                        ),
                                                        TokenIdx(
                                                            215,
                                                        ),
                                                        TokenIdx(
                                                            219,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        220,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 98,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        221,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                222,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 33,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 101,
                                                    dot_token_idx: TokenIdx(
                                                        231,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            232,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        234,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 102,
                                                    lbox_token_idx: TokenIdx(
                                                        233,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        103..104,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        235,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 34,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 105,
                                                    dot_token_idx: TokenIdx(
                                                        240,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            241,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        243,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 106,
                                                    lbox_token_idx: TokenIdx(
                                                        242,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        107..108,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        244,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 108,
                                                    dot_token_idx: TokenIdx(
                                                        245,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `angle_change`,
                                                        token_idx: TokenIdx(
                                                            246,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 32,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        228,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 104,
                                                    dot_token_idx: TokenIdx(
                                                        236,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `rel_norm`,
                                                        token_idx: TokenIdx(
                                                            237,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 109,
                                                    dot_token_idx: TokenIdx(
                                                        247,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `abs`,
                                                        token_idx: TokenIdx(
                                                            248,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        249,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        110..110,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        250,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 100,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        223,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        110..114,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            227,
                                                        ),
                                                        TokenIdx(
                                                            229,
                                                        ),
                                                        TokenIdx(
                                                            238,
                                                        ),
                                                        TokenIdx(
                                                            251,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        252,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 114,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        253,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        255,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 116,
                                                    be_token_idx: TokenIdx(
                                                        256,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesPattern {
                                                            pattern_expr: 6,
                                                            variables: ArenaIdxRange(
                                                                6..7,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        260,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 118,
                                                    dot_token_idx: TokenIdx(
                                                        261,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            262,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        266,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 120,
                                                    dot_token_idx: TokenIdx(
                                                        267,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            268,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 119,
                                                    dot_token_idx: TokenIdx(
                                                        263,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `end`,
                                                        token_idx: TokenIdx(
                                                            264,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 121,
                                                    dot_token_idx: TokenIdx(
                                                        269,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `start`,
                                                        token_idx: TokenIdx(
                                                            270,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 122,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        265,
                                                    ),
                                                    ropd: 123,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        272,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 124,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        271,
                                                    ),
                                                    ropd: 125,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        276,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 127,
                                                    dot_token_idx: TokenIdx(
                                                        277,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            278,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        280,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 129,
                                                    dot_token_idx: TokenIdx(
                                                        281,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            282,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 130,
                                                    dot_token_idx: TokenIdx(
                                                        283,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `start`,
                                                        token_idx: TokenIdx(
                                                            284,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        286,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 131,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        285,
                                                    ),
                                                    ropd: 132,
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 128,
                                                    lbox_token_idx: TokenIdx(
                                                        279,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        133..134,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        287,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `long_vertical`,
                                                    token_idx: TokenIdx(
                                                        291,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 135,
                                                    dot_token_idx: TokenIdx(
                                                        292,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            293,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        294,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        136..136,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        295,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `long_vertical_dp`,
                                                    token_idx: TokenIdx(
                                                        297,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 137,
                                                    dot_token_idx: TokenIdx(
                                                        298,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            299,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        301,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 138,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        300,
                                                    ),
                                                    ropd: 139,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                302,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        310,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        314,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        318,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 144,
                                                    dot_token_idx: TokenIdx(
                                                        319,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `angle_change`,
                                                        token_idx: TokenIdx(
                                                            320,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 36,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        308,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 142,
                                                    dot_token_idx: TokenIdx(
                                                        311,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            312,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 143,
                                                    dot_token_idx: TokenIdx(
                                                        315,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `rel_norm`,
                                                        token_idx: TokenIdx(
                                                            316,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 145,
                                                    dot_token_idx: TokenIdx(
                                                        321,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `abs`,
                                                        token_idx: TokenIdx(
                                                            322,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        323,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        146..146,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        324,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 141,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        303,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        146..151,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            307,
                                                        ),
                                                        TokenIdx(
                                                            309,
                                                        ),
                                                        TokenIdx(
                                                            313,
                                                        ),
                                                        TokenIdx(
                                                            317,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        325,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 151,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        326,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                327,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `long_vertical_dp`,
                                                    token_idx: TokenIdx(
                                                        335,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `long_vertical_dp`,
                                                    token_idx: TokenIdx(
                                                        341,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `long_vertical_dp`,
                                                    token_idx: TokenIdx(
                                                        345,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 155,
                                                    dot_token_idx: TokenIdx(
                                                        342,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `x`,
                                                        token_idx: TokenIdx(
                                                            343,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 156,
                                                    dot_token_idx: TokenIdx(
                                                        346,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            347,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 38,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        333,
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 154,
                                                    dot_token_idx: TokenIdx(
                                                        336,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            337,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        338,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        155..155,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        339,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 157,
                                                    opr: Closed(
                                                        Div,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        344,
                                                    ),
                                                    ropd: 158,
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 153,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        328,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        159..163,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            332,
                                                        ),
                                                        TokenIdx(
                                                            334,
                                                        ),
                                                        TokenIdx(
                                                            340,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        348,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 163,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        349,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `hat`,
                                                    token_idx: TokenIdx(
                                                        351,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 165,
                                                    be_token_idx: TokenIdx(
                                                        352,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesPattern {
                                                            pattern_expr: 9,
                                                            variables: ArenaIdxRange(
                                                                9..10,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                355,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 41,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 168,
                                                    dot_token_idx: TokenIdx(
                                                        364,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            365,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        367,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 169,
                                                    lbox_token_idx: TokenIdx(
                                                        366,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        170..171,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        368,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 42,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 172,
                                                    dot_token_idx: TokenIdx(
                                                        373,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            374,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        376,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 173,
                                                    lbox_token_idx: TokenIdx(
                                                        375,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        174..175,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        377,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 43,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 176,
                                                    dot_token_idx: TokenIdx(
                                                        382,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            383,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        385,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 177,
                                                    lbox_token_idx: TokenIdx(
                                                        384,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        178..179,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        386,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 179,
                                                    dot_token_idx: TokenIdx(
                                                        387,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `angle_change`,
                                                        token_idx: TokenIdx(
                                                            388,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 40,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        361,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 171,
                                                    dot_token_idx: TokenIdx(
                                                        369,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            370,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 175,
                                                    dot_token_idx: TokenIdx(
                                                        378,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `rel_norm`,
                                                        token_idx: TokenIdx(
                                                            379,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 180,
                                                    dot_token_idx: TokenIdx(
                                                        389,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `abs`,
                                                        token_idx: TokenIdx(
                                                            390,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        391,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        181..181,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        392,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 167,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        356,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        181..186,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            360,
                                                        ),
                                                        TokenIdx(
                                                            362,
                                                        ),
                                                        TokenIdx(
                                                            371,
                                                        ),
                                                        TokenIdx(
                                                            380,
                                                        ),
                                                        TokenIdx(
                                                            393,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        394,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 186,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        395,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost_number_of_strokes`,
                                                    token_idx: TokenIdx(
                                                        397,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        399,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 188,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        398,
                                                    ),
                                                    ropd: 189,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        403,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 191,
                                                    dot_token_idx: TokenIdx(
                                                        404,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            405,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        407,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 193,
                                                    dot_token_idx: TokenIdx(
                                                        408,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            409,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 194,
                                                    dot_token_idx: TokenIdx(
                                                        410,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `start`,
                                                        token_idx: TokenIdx(
                                                            411,
                                                        ),
                                                    },
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 192,
                                                    lbox_token_idx: TokenIdx(
                                                        406,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        195..196,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        412,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost_hat`,
                                                    token_idx: TokenIdx(
                                                        416,
                                                    ),
                                                    current_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 197,
                                                    dot_token_idx: TokenIdx(
                                                        417,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            418,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        419,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        198..198,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        420,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        424,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 199,
                                                    dot_token_idx: TokenIdx(
                                                        425,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            426,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        428,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 201,
                                                    dot_token_idx: TokenIdx(
                                                        429,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            430,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 202,
                                                    dot_token_idx: TokenIdx(
                                                        431,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `start`,
                                                        token_idx: TokenIdx(
                                                            432,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        434,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 203,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        433,
                                                    ),
                                                    ropd: 204,
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 200,
                                                    lbox_token_idx: TokenIdx(
                                                        427,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        205..206,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        435,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost_feet`,
                                                    token_idx: TokenIdx(
                                                        439,
                                                    ),
                                                    current_symbol_idx: 12,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 12,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 207,
                                                    dot_token_idx: TokenIdx(
                                                        440,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            441,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        442,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        208..208,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        443,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                444,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `downmost_hat_dp`,
                                                    token_idx: TokenIdx(
                                                        452,
                                                    ),
                                                    current_symbol_idx: 11,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 11,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost_feet_dp`,
                                                    token_idx: TokenIdx(
                                                        456,
                                                    ),
                                                    current_symbol_idx: 13,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 13,
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 45,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        450,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 210,
                                                    dot_token_idx: TokenIdx(
                                                        453,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            454,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 211,
                                                    dot_token_idx: TokenIdx(
                                                        457,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            458,
                                                        ),
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 209,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        445,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        212..216,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            449,
                                                        ),
                                                        TokenIdx(
                                                            451,
                                                        ),
                                                        TokenIdx(
                                                            455,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        459,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 216,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        460,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost_number_of_strokes`,
                                                    token_idx: TokenIdx(
                                                        462,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        464,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 218,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        463,
                                                    ),
                                                    ropd: 219,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 46,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 221,
                                                    dot_token_idx: TokenIdx(
                                                        469,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `lower_mass`,
                                                        token_idx: TokenIdx(
                                                            470,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 47,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 223,
                                                    dot_token_idx: TokenIdx(
                                                        475,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `upper_mass`,
                                                        token_idx: TokenIdx(
                                                            476,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        480,
                                                    ),
                                                    current_symbol_idx: 14,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 14,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `b`,
                                                    token_idx: TokenIdx(
                                                        482,
                                                    ),
                                                    current_symbol_idx: 15,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 15,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 225,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        481,
                                                    ),
                                                    ropd: 226,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `c`,
                                                    token_idx: TokenIdx(
                                                        486,
                                                    ),
                                                    current_symbol_idx: 16,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 16,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        488,
                                                    ),
                                                    current_symbol_idx: 14,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 14,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 228,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        487,
                                                    ),
                                                    ropd: 229,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        493,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `c`,
                                                    token_idx: TokenIdx(
                                                        490,
                                                    ),
                                                    current_symbol_idx: 16,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 16,
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        492,
                                                    ),
                                                    opd: 231,
                                                },
                                                Expr::Binary {
                                                    lopd: 232,
                                                    opr: Comparison(
                                                        Geq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        491,
                                                    ),
                                                    ropd: 233,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `c`,
                                                    token_idx: TokenIdx(
                                                        495,
                                                    ),
                                                    current_symbol_idx: 16,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 16,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        497,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 235,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        496,
                                                    ),
                                                    ropd: 236,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 49,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        41..44,
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        32,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 0,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            33,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `One`,
                                                            token_idx: TokenIdx(
                                                                34,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        38,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        42,
                                                    ),
                                                    ident: `ignored_connected_components_row_span_sum_sum`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        48,
                                                    ),
                                                    ident: `fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        50,
                                                    ),
                                                    ident: `major_concave_components`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        64,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 6,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            65,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `One`,
                                                            token_idx: TokenIdx(
                                                                66,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        70,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        76,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        83,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        88,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 11,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            89,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `One`,
                                                            token_idx: TokenIdx(
                                                                90,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        94,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        100,
                                                    ),
                                                    ident: `ignored_connected_components_row_span_sum_sum`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        106,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        115,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        124,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        143,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 18,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            144,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `One`,
                                                            token_idx: TokenIdx(
                                                                145,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        161,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        165,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        170,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 22,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            171,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `One`,
                                                            token_idx: TokenIdx(
                                                                172,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        188,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        195,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 25,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            196,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `One`,
                                                            token_idx: TokenIdx(
                                                                197,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        201,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        208,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        212,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        216,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        224,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 31,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            225,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `One`,
                                                            token_idx: TokenIdx(
                                                                226,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        230,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        239,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        304,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 35,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            305,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `One`,
                                                            token_idx: TokenIdx(
                                                                306,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        329,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 37,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            330,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `One`,
                                                            token_idx: TokenIdx(
                                                                331,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        357,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 39,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            358,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `One`,
                                                            token_idx: TokenIdx(
                                                                359,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        363,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        372,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        381,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        446,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 44,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            447,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `One`,
                                                            token_idx: TokenIdx(
                                                                448,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        468,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        474,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        498,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 48,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            499,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `One`,
                                                            token_idx: TokenIdx(
                                                                500,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `One`,
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
                                                            82,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        30,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 22,
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                75,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            26,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        81,
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
                                                Stmt::Eval {
                                                    expr_idx: 31,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            135,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        56,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 64,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            158,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        73,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 74,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            259,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        126,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            273,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 7,
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
                                                                275,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        134,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            288,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 8,
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
                                                                290,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        136,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            296,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        140,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 152,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 164,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 187,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            93,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        35,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            99,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        38,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            103,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 1,
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
                                                                105,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        42,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            112,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 2,
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
                                                                114,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        46,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            121,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 3,
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
                                                                123,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        50,
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                130,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            52,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        134,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                4..8,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            173,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 5,
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
                                                                175,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        81,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            187,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        85,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 99,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 115,
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                254,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            117,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        258,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                8..14,
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
                                                                350,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            166,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        354,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                14..15,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            396,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        190,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            400,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 10,
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
                                                                402,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        196,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            413,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 11,
                                                            variables: ArenaIdxRange(
                                                                11..12,
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
                                                                415,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        198,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            421,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 12,
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
                                                                423,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        206,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            436,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 13,
                                                            variables: ArenaIdxRange(
                                                                13..14,
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
                                                                438,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        208,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 217,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            461,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        220,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            465,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 14,
                                                            variables: ArenaIdxRange(
                                                                14..15,
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
                                                                467,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        222,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            471,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 15,
                                                            variables: ArenaIdxRange(
                                                                15..16,
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
                                                                473,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        224,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            477,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 16,
                                                            variables: ArenaIdxRange(
                                                                16..17,
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
                                                                479,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        227,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            483,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 17,
                                                            variables: ArenaIdxRange(
                                                                17..18,
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
                                                                485,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        230,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            489,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        234,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            494,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        237,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 238,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 7,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            45,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 0,
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
                                                                47,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        11,
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                55,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            15,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        61,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                1..4,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: Some(
                                                        ElseBranch {
                                                            else_token: ElseToken {
                                                                token_idx: TokenIdx(
                                                                    91,
                                                                ),
                                                            },
                                                            eol_colon: Ok(
                                                                Colon(
                                                                    EolColonToken {
                                                                        token_idx: TokenIdx(
                                                                            92,
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                            block: Ok(
                                                                ArenaIdxRange(
                                                                    15..41,
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                },
                                            ],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `simp_one_match`,
                                                            token_idx: TokenIdx(
                                                                46,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `downmost`,
                                                            token_idx: TokenIdx(
                                                                104,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `upmost`,
                                                            token_idx: TokenIdx(
                                                                113,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `hat`,
                                                            token_idx: TokenIdx(
                                                                122,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                133,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `downmost_number_of_strokes`,
                                                            token_idx: TokenIdx(
                                                                174,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                257,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `long_vertical`,
                                                            token_idx: TokenIdx(
                                                                274,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `long_vertical_dp`,
                                                            token_idx: TokenIdx(
                                                                289,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                353,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `downmost_hat`,
                                                            token_idx: TokenIdx(
                                                                401,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `downmost_hat_dp`,
                                                            token_idx: TokenIdx(
                                                                414,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `downmost_feet`,
                                                            token_idx: TokenIdx(
                                                                422,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `downmost_feet_dp`,
                                                            token_idx: TokenIdx(
                                                                437,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `a`,
                                                            token_idx: TokenIdx(
                                                                466,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `b`,
                                                            token_idx: TokenIdx(
                                                                472,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `c`,
                                                            token_idx: TokenIdx(
                                                                478,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `d`,
                                                            token_idx: TokenIdx(
                                                                484,
                                                            ),
                                                        },
                                                    },
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
                                                Let,
                                                Let,
                                                Let,
                                                Let,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `simp_one_match`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `downmost`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `upmost`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `hat`,
                                                        3,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `none`,
                                                        4,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `downmost_number_of_strokes`,
                                                        5,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        6,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `long_vertical`,
                                                        7,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `long_vertical_dp`,
                                                        8,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        9,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `downmost_hat`,
                                                        10,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `downmost_hat_dp`,
                                                        11,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `downmost_feet`,
                                                        12,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `downmost_feet_dp`,
                                                        13,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `a`,
                                                        14,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `b`,
                                                        15,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `c`,
                                                        16,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `d`,
                                                        17,
                                                    ),
                                                ],
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
                                                    PatternSymbol::Atom(
                                                        14,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        15,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        16,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        17,
                                                    ),
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
                                                        access_start: TokenIdx(
                                                            47,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    501,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `simp_one_match`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            105,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    501,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `downmost`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            114,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    501,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `upmost`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            123,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    501,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `hat`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            134,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    173,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `none`,
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            175,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    501,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `downmost_number_of_strokes`,
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            258,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    350,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 6,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            275,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    350,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `long_vertical`,
                                                            pattern_symbol_idx: 7,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            290,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    350,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `long_vertical_dp`,
                                                            pattern_symbol_idx: 8,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            354,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    396,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 9,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            402,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    501,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `downmost_hat`,
                                                            pattern_symbol_idx: 10,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            415,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    501,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `downmost_hat_dp`,
                                                            pattern_symbol_idx: 11,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            423,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    501,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `downmost_feet`,
                                                            pattern_symbol_idx: 12,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            438,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    501,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `downmost_feet_dp`,
                                                            pattern_symbol_idx: 13,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            467,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    501,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `a`,
                                                            pattern_symbol_idx: 14,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            473,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    501,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `b`,
                                                            pattern_symbol_idx: 15,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            479,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    501,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `c`,
                                                            pattern_symbol_idx: 16,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            485,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    501,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `d`,
                                                            pattern_symbol_idx: 17,
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
                                                kind: BlockExpr,
                                                expr: 239,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    239,
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::digits::one::upmost`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::digits::one::upmost`, `Fn`),
                                decl: FnDecl {
                                    path: FormPath(`mnist_classifier::digits::one::upmost`, `Fn`),
                                    ast_idx: 64,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::upmost`, `Fn`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
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
                                                            506,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::EntityPath {
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
                                                            510,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            507,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            511,
                                                        ),
                                                        ident: `f32`,
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
                                                            modifier: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    504,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `cc`,
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
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
                                                            access_start: TokenIdx(
                                                                505,
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
                                                    ExplicitParameter {
                                                        pattern: 0,
                                                        ty: 1,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 3,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                503,
                                            ),
                                        ),
                                        self_parameter: None,
                                        regular_parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        505,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                508,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                509,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 3,
                                        },
                                    ),
                                    eol_colon: EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                512,
                                            ),
                                        },
                                    ),
                                },
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclRegionPath::Entity(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Form(
                                                                    FormPath(`mnist_classifier::digits::one::upmost`, `Fn`),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            Expr::EntityPath {
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
                                                                    506,
                                                                ),
                                                                opd: 0,
                                                            },
                                                            Expr::EntityPath {
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
                                                                    510,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    507,
                                                                ),
                                                                ident: `ConcaveComponent`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    511,
                                                                ),
                                                                ident: `f32`,
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
                                                                    modifier: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `cc`,
                                                                        token_idx: TokenIdx(
                                                                            504,
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        pattern_infos: [
                                                            Parameter,
                                                        ],
                                                        pattern_symbol_maps: [
                                                            [
                                                                (
                                                                    `cc`,
                                                                    0,
                                                                ),
                                                            ],
                                                        ],
                                                        pattern_symbol_arena: Arena {
                                                            data: [
                                                                PatternSymbol::Atom(
                                                                    0,
                                                                ),
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
                                                                    access_start: TokenIdx(
                                                                        505,
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
                                                            ExplicitParameter {
                                                                pattern: 0,
                                                                ty: 1,
                                                            },
                                                        ],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: ReturnType,
                                                            expr: 3,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::one::upmost`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        516,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        517,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            518,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        519,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        520,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        522,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        523,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            524,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        526,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 3,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        525,
                                                    ),
                                                    ropd: 4,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        527,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 6,
                                                    dot_token_idx: TokenIdx(
                                                        528,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            529,
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
                                                            513,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 0,
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
                                                                515,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        1,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            521,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        5,
                                                    ),
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
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `dp`,
                                                            token_idx: TokenIdx(
                                                                514,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Let,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `dp`,
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    PatternSymbol::Atom(
                                                        0,
                                                    ),
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
                                                        kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `cc`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            515,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    530,
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
                                                kind: BlockExpr,
                                                expr: 8,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    8,
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::digits::one::downmost`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::digits::one::downmost`, `Fn`),
                                decl: FnDecl {
                                    path: FormPath(`mnist_classifier::digits::one::downmost`, `Fn`),
                                    ast_idx: 65,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::downmost`, `Fn`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
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
                                                            535,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::EntityPath {
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
                                                            539,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            536,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            540,
                                                        ),
                                                        ident: `f32`,
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
                                                            modifier: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    533,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `cc`,
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
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
                                                            access_start: TokenIdx(
                                                                534,
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
                                                    ExplicitParameter {
                                                        pattern: 0,
                                                        ty: 1,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 3,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                532,
                                            ),
                                        ),
                                        self_parameter: None,
                                        regular_parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        534,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                537,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                538,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 3,
                                        },
                                    ),
                                    eol_colon: EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                541,
                                            ),
                                        },
                                    ),
                                },
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclRegionPath::Entity(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Form(
                                                                    FormPath(`mnist_classifier::digits::one::downmost`, `Fn`),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            Expr::EntityPath {
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
                                                                    535,
                                                                ),
                                                                opd: 0,
                                                            },
                                                            Expr::EntityPath {
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
                                                                    539,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    536,
                                                                ),
                                                                ident: `ConcaveComponent`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    540,
                                                                ),
                                                                ident: `f32`,
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
                                                                    modifier: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `cc`,
                                                                        token_idx: TokenIdx(
                                                                            533,
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        pattern_infos: [
                                                            Parameter,
                                                        ],
                                                        pattern_symbol_maps: [
                                                            [
                                                                (
                                                                    `cc`,
                                                                    0,
                                                                ),
                                                            ],
                                                        ],
                                                        pattern_symbol_arena: Arena {
                                                            data: [
                                                                PatternSymbol::Atom(
                                                                    0,
                                                                ),
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
                                                                    access_start: TokenIdx(
                                                                        534,
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
                                                            ExplicitParameter {
                                                                pattern: 0,
                                                                ty: 1,
                                                            },
                                                        ],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: ReturnType,
                                                            expr: 3,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::one::downmost`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        545,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        546,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            547,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        548,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        549,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        551,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        552,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            553,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        555,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 3,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        554,
                                                    ),
                                                    ropd: 4,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        557,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 6,
                                                    dot_token_idx: TokenIdx(
                                                        558,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `end`,
                                                        token_idx: TokenIdx(
                                                            559,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        560,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        7..7,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        561,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 7,
                                                    dot_token_idx: TokenIdx(
                                                        562,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            563,
                                                        ),
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        556,
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
                                        entity_path_expr_arena: Arena {
                                            data: [],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            542,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 0,
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
                                                                544,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        1,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            550,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        5,
                                                    ),
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
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `dp`,
                                                            token_idx: TokenIdx(
                                                                543,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Let,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `dp`,
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    PatternSymbol::Atom(
                                                        0,
                                                    ),
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
                                                        kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `cc`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            544,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    564,
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
                                                kind: BlockExpr,
                                                expr: 10,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    10,
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::digits::one::hat`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::digits::one::hat`, `Fn`),
                                decl: FnDecl {
                                    path: FormPath(`mnist_classifier::digits::one::hat`, `Fn`),
                                    ast_idx: 66,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::hat`, `Fn`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
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
                                                            569,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::EntityPath {
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
                                                            573,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            570,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            574,
                                                        ),
                                                        ident: `f32`,
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
                                                            modifier: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    567,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            `cc`,
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        PatternSymbol::Atom(
                                                            0,
                                                        ),
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
                                                            access_start: TokenIdx(
                                                                568,
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
                                                    ExplicitParameter {
                                                        pattern: 0,
                                                        ty: 1,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 3,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: None,
                                    parameter_decl_list: ExplicitParameterDeclList {
                                        lpar: LeftParenthesisToken(
                                            TokenIdx(
                                                566,
                                            ),
                                        ),
                                        self_parameter: None,
                                        regular_parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        568,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                571,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                572,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 3,
                                        },
                                    ),
                                    eol_colon: EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                575,
                                            ),
                                        },
                                    ),
                                },
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: Some(
                                            ExprRegion {
                                                data: ExprRegionData {
                                                    parent: None,
                                                    path: RegionPath::Decl(
                                                        DeclRegionPath::Entity(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Form(
                                                                    FormPath(`mnist_classifier::digits::one::hat`, `Fn`),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            Expr::EntityPath {
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
                                                                    569,
                                                                ),
                                                                opd: 0,
                                                            },
                                                            Expr::EntityPath {
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
                                                                    573,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    570,
                                                                ),
                                                                ident: `ConcaveComponent`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    574,
                                                                ),
                                                                ident: `f32`,
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
                                                                    modifier: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `cc`,
                                                                        token_idx: TokenIdx(
                                                                            567,
                                                                        ),
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        pattern_infos: [
                                                            Parameter,
                                                        ],
                                                        pattern_symbol_maps: [
                                                            [
                                                                (
                                                                    `cc`,
                                                                    0,
                                                                ),
                                                            ],
                                                        ],
                                                        pattern_symbol_arena: Arena {
                                                            data: [
                                                                PatternSymbol::Atom(
                                                                    0,
                                                                ),
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
                                                                    access_start: TokenIdx(
                                                                        568,
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
                                                            ExplicitParameter {
                                                                pattern: 0,
                                                                ty: 1,
                                                            },
                                                        ],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: ReturnType,
                                                            expr: 3,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::digits::one::hat`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        579,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        580,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            581,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        582,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        583,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        585,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        586,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            587,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        589,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 3,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        588,
                                                    ),
                                                    ropd: 4,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        591,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 6,
                                                    dot_token_idx: TokenIdx(
                                                        592,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `x`,
                                                        token_idx: TokenIdx(
                                                            593,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        595,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 7,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        594,
                                                    ),
                                                    ropd: 8,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        597,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 10,
                                                    dot_token_idx: TokenIdx(
                                                        598,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            599,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        601,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        596,
                                                    ),
                                                    opd: 11,
                                                },
                                                Expr::Field {
                                                    owner: 12,
                                                    dot_token_idx: TokenIdx(
                                                        602,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `x`,
                                                        token_idx: TokenIdx(
                                                            603,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 13,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        600,
                                                    ),
                                                    ropd: 14,
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        0..4,
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
                                                            576,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern_expr: 0,
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
                                                                578,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        1,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            584,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        5,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            590,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        9,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 15,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `dp`,
                                                            token_idx: TokenIdx(
                                                                577,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Let,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `dp`,
                                                        0,
                                                    ),
                                                ],
                                            ],
                                            pattern_symbol_arena: Arena {
                                                data: [
                                                    PatternSymbol::Atom(
                                                        0,
                                                    ),
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
                                                        kind: InheritedSymbolKind::ExplicitParameter {
                                                            ident: `cc`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            578,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    604,
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
                                                kind: BlockExpr,
                                                expr: 16,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    16,
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)