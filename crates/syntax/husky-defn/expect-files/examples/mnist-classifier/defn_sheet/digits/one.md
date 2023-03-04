Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Feature(
                            FeatureDefn {
                                path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                decl: FeatureDecl {
                                    path: FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                    ast_idx: 68,
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                60,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                62,
                                            ),
                                        ),
                                    ),
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::UnrecognizedIdentifier {
                                                                token_idx: TokenIdx(
                                                                    61,
                                                                ),
                                                                ident: `FermiMatchResult`,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
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
                                                    kind: ReturnType,
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
                                                        FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        67,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        1..4,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        73,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 0,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        64,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        4..6,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            66,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        74,
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
                                                        63,
                                                    ),
                                                    ident: `fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        65,
                                                    ),
                                                    ident: `major_concave_components`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        68,
                                                    ),
                                                    ident: `downmost`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        70,
                                                    ),
                                                    ident: `upmost`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        72,
                                                    ),
                                                    ident: `hat`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::hat`, `Function`),
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
                                body: Ok(
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
                            FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Feature(
                            FeatureDefn {
                                path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                decl: FeatureDecl {
                                    path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                    ast_idx: 69,
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                78,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                81,
                                            ),
                                        ),
                                    ),
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::UnrecognizedIdentifier {
                                                                token_idx: TokenIdx(
                                                                    80,
                                                                ),
                                                                ident: `MnistLabel`,
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            79,
                                                        ),
                                                        opd: 0,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
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
                                                    kind: ReturnType,
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
                                                        FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                82,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                84,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                86,
                                                            ),
                                                            ident: `One`,
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 1,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        85,
                                                    ),
                                                    ropd: 2,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        88,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 3,
                                                    dot_token_idx: TokenIdx(
                                                        91,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `max_hole_ilen`,
                                                        token_idx: TokenIdx(
                                                            92,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 0,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        83,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        4..8,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            87,
                                                        ),
                                                        TokenIdx(
                                                            89,
                                                        ),
                                                        TokenIdx(
                                                            93,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        95,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 8,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        96,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        104,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        11..11,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        105,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                99,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 10,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        101,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        11..13,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            103,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        106,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 13,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        99,
                                                    ),
                                                    ropd: 14,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `simp_one_match`,
                                                    token_idx: TokenIdx(
                                                        108,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 16,
                                                    dot_token_idx: TokenIdx(
                                                        109,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            110,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        112,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 17,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        111,
                                                    ),
                                                    ropd: 18,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                114,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                116,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                118,
                                                            ),
                                                            ident: `One`,
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 21,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        117,
                                                    ),
                                                    ropd: 22,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        120,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 23,
                                                    dot_token_idx: TokenIdx(
                                                        123,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `max_row_span`,
                                                        token_idx: TokenIdx(
                                                            124,
                                                        ),
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 20,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        115,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        24..27,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            119,
                                                        ),
                                                        TokenIdx(
                                                            121,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        125,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 27,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        126,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 5,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 29,
                                                    dot_token_idx: TokenIdx(
                                                        129,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `max_row_span`,
                                                        token_idx: TokenIdx(
                                                            130,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        132,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 30,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        131,
                                                    ),
                                                    ropd: 31,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 6,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 33,
                                                    dot_token_idx: TokenIdx(
                                                        136,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `max_hole_ilen`,
                                                        token_idx: TokenIdx(
                                                            137,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        139,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 34,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        138,
                                                    ),
                                                    ropd: 35,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                140,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                142,
                                                            ),
                                                            ident: `One`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 37,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        141,
                                                    ),
                                                    ropd: 38,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 7,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 40,
                                                    dot_token_idx: TokenIdx(
                                                        147,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `max_hole_ilen`,
                                                        token_idx: TokenIdx(
                                                            148,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        150,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 41,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        149,
                                                    ),
                                                    ropd: 42,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 8,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        154,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 44,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        153,
                                                    ),
                                                    ropd: 45,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 9,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 47,
                                                    dot_token_idx: TokenIdx(
                                                        159,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            160,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        162,
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                157,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 48,
                                                    lbox_token_idx: TokenIdx(
                                                        161,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        49..50,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        163,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 50,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        157,
                                                    ),
                                                    ropd: 51,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 10,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 53,
                                                    dot_token_idx: TokenIdx(
                                                        168,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            169,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        171,
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                166,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 54,
                                                    lbox_token_idx: TokenIdx(
                                                        170,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        55..56,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        172,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 56,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        166,
                                                    ),
                                                    ropd: 57,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 11,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 59,
                                                    dot_token_idx: TokenIdx(
                                                        177,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            178,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        180,
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                175,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 60,
                                                    lbox_token_idx: TokenIdx(
                                                        179,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        61..62,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        181,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 62,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        175,
                                                    ),
                                                    ropd: 63,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        183,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 65,
                                                    be_token_idx: TokenIdx(
                                                        184,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 4,
                                                        },
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `simp_one_match`,
                                                    token_idx: TokenIdx(
                                                        188,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 67,
                                                    dot_token_idx: TokenIdx(
                                                        189,
                                                    ),
                                                    ident_token: IdentifierToken {
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
                                                    lopd: 68,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        191,
                                                    ),
                                                    ropd: 69,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                193,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                195,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                197,
                                                            ),
                                                            ident: `One`,
                                                        },
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `simp_one_match`,
                                                    token_idx: TokenIdx(
                                                        201,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 74,
                                                    dot_token_idx: TokenIdx(
                                                        202,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `angle_change_norm`,
                                                        token_idx: TokenIdx(
                                                            203,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 72,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        196,
                                                    ),
                                                    ropd: 73,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        199,
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 75,
                                                    dot_token_idx: TokenIdx(
                                                        204,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `abs`,
                                                        token_idx: TokenIdx(
                                                            205,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        206,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        76..76,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        207,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 71,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        194,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        76..79,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            198,
                                                        ),
                                                        TokenIdx(
                                                            200,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        208,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 79,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        209,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 12,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        211,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 81,
                                                    dot_token_idx: TokenIdx(
                                                        214,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `lower_mass`,
                                                        token_idx: TokenIdx(
                                                            215,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 13,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 82,
                                                    opr: PureClosed(
                                                        Mul,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        212,
                                                    ),
                                                    ropd: 83,
                                                },
                                                Expr::Field {
                                                    owner: 84,
                                                    dot_token_idx: TokenIdx(
                                                        218,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `upper_mass`,
                                                        token_idx: TokenIdx(
                                                            219,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 85,
                                                    opr: PureClosed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        216,
                                                    ),
                                                    ropd: 86,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        221,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 87,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        220,
                                                    ),
                                                    ropd: 88,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                222,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                224,
                                                            ),
                                                            ident: `One`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 90,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        223,
                                                    ),
                                                    ropd: 91,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        228,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 93,
                                                    dot_token_idx: TokenIdx(
                                                        229,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            230,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        234,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 95,
                                                    dot_token_idx: TokenIdx(
                                                        235,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            236,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 94,
                                                    dot_token_idx: TokenIdx(
                                                        231,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `end`,
                                                        token_idx: TokenIdx(
                                                            232,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 96,
                                                    dot_token_idx: TokenIdx(
                                                        237,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `start`,
                                                        token_idx: TokenIdx(
                                                            238,
                                                        ),
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                227,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 97,
                                                    opr: PureClosed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        233,
                                                    ),
                                                    ropd: 98,
                                                },
                                                Expr::Binary {
                                                    lopd: 99,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        227,
                                                    ),
                                                    ropd: 100,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 14,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 102,
                                                    dot_token_idx: TokenIdx(
                                                        241,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            242,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        244,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 103,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        243,
                                                    ),
                                                    ropd: 104,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                245,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                247,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                249,
                                                            ),
                                                            ident: `One`,
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 15,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        257,
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 16,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 17,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 18,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 107,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        248,
                                                    ),
                                                    ropd: 108,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        251,
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 109,
                                                    dot_token_idx: TokenIdx(
                                                        254,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `top_k_row_span_sum`,
                                                        token_idx: TokenIdx(
                                                            255,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        256,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        110..111,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        258,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 111,
                                                    dot_token_idx: TokenIdx(
                                                        261,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            262,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 112,
                                                    dot_token_idx: TokenIdx(
                                                        265,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `rel_norm`,
                                                        token_idx: TokenIdx(
                                                            266,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 113,
                                                    dot_token_idx: TokenIdx(
                                                        269,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `angle_change_norm`,
                                                        token_idx: TokenIdx(
                                                            270,
                                                        ),
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 106,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        246,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        114..120,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            250,
                                                        ),
                                                        TokenIdx(
                                                            252,
                                                        ),
                                                        TokenIdx(
                                                            259,
                                                        ),
                                                        TokenIdx(
                                                            263,
                                                        ),
                                                        TokenIdx(
                                                            267,
                                                        ),
                                                        TokenIdx(
                                                            271,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        272,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 120,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        273,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                274,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                276,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                278,
                                                            ),
                                                            ident: `One`,
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 19,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 125,
                                                    dot_token_idx: TokenIdx(
                                                        283,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `matches`,
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
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 126,
                                                    lbox_token_idx: TokenIdx(
                                                        285,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        127..128,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        287,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 20,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 129,
                                                    dot_token_idx: TokenIdx(
                                                        292,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            293,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        295,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 130,
                                                    lbox_token_idx: TokenIdx(
                                                        294,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        131..132,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        296,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 132,
                                                    dot_token_idx: TokenIdx(
                                                        297,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `angle_change`,
                                                        token_idx: TokenIdx(
                                                            298,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 123,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        277,
                                                    ),
                                                    ropd: 124,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        280,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 128,
                                                    dot_token_idx: TokenIdx(
                                                        288,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `rel_norm`,
                                                        token_idx: TokenIdx(
                                                            289,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 133,
                                                    dot_token_idx: TokenIdx(
                                                        299,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `abs`,
                                                        token_idx: TokenIdx(
                                                            300,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        301,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        134..134,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        302,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 122,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        275,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        134..138,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            279,
                                                        ),
                                                        TokenIdx(
                                                            281,
                                                        ),
                                                        TokenIdx(
                                                            290,
                                                        ),
                                                        TokenIdx(
                                                            303,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        304,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 138,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        305,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        307,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 140,
                                                    be_token_idx: TokenIdx(
                                                        308,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 6,
                                                        },
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        312,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 142,
                                                    dot_token_idx: TokenIdx(
                                                        313,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            314,
                                                        ),
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
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            320,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 143,
                                                    dot_token_idx: TokenIdx(
                                                        315,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `end`,
                                                        token_idx: TokenIdx(
                                                            316,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 145,
                                                    dot_token_idx: TokenIdx(
                                                        321,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `start`,
                                                        token_idx: TokenIdx(
                                                            322,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 146,
                                                    opr: PureClosed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        317,
                                                    ),
                                                    ropd: 147,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        324,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 148,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        323,
                                                    ),
                                                    ropd: 149,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        328,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 151,
                                                    dot_token_idx: TokenIdx(
                                                        329,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            330,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        332,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 153,
                                                    dot_token_idx: TokenIdx(
                                                        333,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            334,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 154,
                                                    dot_token_idx: TokenIdx(
                                                        335,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `start`,
                                                        token_idx: TokenIdx(
                                                            336,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        338,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 155,
                                                    opr: PureClosed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        337,
                                                    ),
                                                    ropd: 156,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                327,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 152,
                                                    lbox_token_idx: TokenIdx(
                                                        331,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        157..158,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        339,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 158,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        327,
                                                    ),
                                                    ropd: 159,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `long_vertical`,
                                                    token_idx: TokenIdx(
                                                        343,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                342,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 161,
                                                    dot_token_idx: TokenIdx(
                                                        344,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            345,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        346,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        162..162,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        347,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 162,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        342,
                                                    ),
                                                    ropd: 163,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `long_vertical_dp`,
                                                    token_idx: TokenIdx(
                                                        349,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 165,
                                                    dot_token_idx: TokenIdx(
                                                        350,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            351,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        353,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 166,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        352,
                                                    ),
                                                    ropd: 167,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                354,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                356,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                358,
                                                            ),
                                                            ident: `One`,
                                                        },
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        362,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        366,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upmost`,
                                                    token_idx: TokenIdx(
                                                        370,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 174,
                                                    dot_token_idx: TokenIdx(
                                                        371,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `angle_change`,
                                                        token_idx: TokenIdx(
                                                            372,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 170,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        357,
                                                    ),
                                                    ropd: 171,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        360,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 172,
                                                    dot_token_idx: TokenIdx(
                                                        363,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            364,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 173,
                                                    dot_token_idx: TokenIdx(
                                                        367,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `rel_norm`,
                                                        token_idx: TokenIdx(
                                                            368,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 175,
                                                    dot_token_idx: TokenIdx(
                                                        373,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `abs`,
                                                        token_idx: TokenIdx(
                                                            374,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        375,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        176..176,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        376,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 169,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        355,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        176..181,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            359,
                                                        ),
                                                        TokenIdx(
                                                            361,
                                                        ),
                                                        TokenIdx(
                                                            365,
                                                        ),
                                                        TokenIdx(
                                                            369,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        377,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 181,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        378,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                379,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                381,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                383,
                                                            ),
                                                            ident: `One`,
                                                        },
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `long_vertical_dp`,
                                                    token_idx: TokenIdx(
                                                        387,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `long_vertical_dp`,
                                                    token_idx: TokenIdx(
                                                        393,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `long_vertical_dp`,
                                                    token_idx: TokenIdx(
                                                        397,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 187,
                                                    dot_token_idx: TokenIdx(
                                                        394,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `x`,
                                                        token_idx: TokenIdx(
                                                            395,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 188,
                                                    dot_token_idx: TokenIdx(
                                                        398,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            399,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 184,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        382,
                                                    ),
                                                    ropd: 185,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        385,
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 186,
                                                    dot_token_idx: TokenIdx(
                                                        388,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            389,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        390,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        187..187,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        391,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 189,
                                                    opr: PureClosed(
                                                        Div,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        396,
                                                    ),
                                                    ropd: 190,
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 183,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        380,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        191..195,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            384,
                                                        ),
                                                        TokenIdx(
                                                            386,
                                                        ),
                                                        TokenIdx(
                                                            392,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        400,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 195,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        401,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `hat`,
                                                    token_idx: TokenIdx(
                                                        403,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 197,
                                                    be_token_idx: TokenIdx(
                                                        404,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 9,
                                                        },
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                407,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                409,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                411,
                                                            ),
                                                            ident: `One`,
                                                        },
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 21,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 202,
                                                    dot_token_idx: TokenIdx(
                                                        416,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            417,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        419,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 203,
                                                    lbox_token_idx: TokenIdx(
                                                        418,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        204..205,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        420,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 22,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 206,
                                                    dot_token_idx: TokenIdx(
                                                        425,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            426,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        428,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 207,
                                                    lbox_token_idx: TokenIdx(
                                                        427,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        208..209,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        429,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 23,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 210,
                                                    dot_token_idx: TokenIdx(
                                                        434,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            435,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        437,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 211,
                                                    lbox_token_idx: TokenIdx(
                                                        436,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        212..213,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        438,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 213,
                                                    dot_token_idx: TokenIdx(
                                                        439,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `angle_change`,
                                                        token_idx: TokenIdx(
                                                            440,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 200,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        410,
                                                    ),
                                                    ropd: 201,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        413,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 205,
                                                    dot_token_idx: TokenIdx(
                                                        421,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            422,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 209,
                                                    dot_token_idx: TokenIdx(
                                                        430,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `rel_norm`,
                                                        token_idx: TokenIdx(
                                                            431,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 214,
                                                    dot_token_idx: TokenIdx(
                                                        441,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `abs`,
                                                        token_idx: TokenIdx(
                                                            442,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        443,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        215..215,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        444,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 199,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        408,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        215..220,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            412,
                                                        ),
                                                        TokenIdx(
                                                            414,
                                                        ),
                                                        TokenIdx(
                                                            423,
                                                        ),
                                                        TokenIdx(
                                                            432,
                                                        ),
                                                        TokenIdx(
                                                            445,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        446,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 220,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        447,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost_number_of_strokes`,
                                                    token_idx: TokenIdx(
                                                        449,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        451,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 222,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        450,
                                                    ),
                                                    ropd: 223,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        455,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 225,
                                                    dot_token_idx: TokenIdx(
                                                        456,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            457,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        459,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 227,
                                                    dot_token_idx: TokenIdx(
                                                        460,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            461,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 228,
                                                    dot_token_idx: TokenIdx(
                                                        462,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `start`,
                                                        token_idx: TokenIdx(
                                                            463,
                                                        ),
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                454,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 226,
                                                    lbox_token_idx: TokenIdx(
                                                        458,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        229..230,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        464,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 230,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        454,
                                                    ),
                                                    ropd: 231,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost_hat`,
                                                    token_idx: TokenIdx(
                                                        468,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                467,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 233,
                                                    dot_token_idx: TokenIdx(
                                                        469,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            470,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        471,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        234..234,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        472,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 234,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        467,
                                                    ),
                                                    ropd: 235,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        476,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 237,
                                                    dot_token_idx: TokenIdx(
                                                        477,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            478,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost`,
                                                    token_idx: TokenIdx(
                                                        480,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 239,
                                                    dot_token_idx: TokenIdx(
                                                        481,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            482,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 240,
                                                    dot_token_idx: TokenIdx(
                                                        483,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `start`,
                                                        token_idx: TokenIdx(
                                                            484,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        486,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 241,
                                                    opr: PureClosed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        485,
                                                    ),
                                                    ropd: 242,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                475,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 238,
                                                    lbox_token_idx: TokenIdx(
                                                        479,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        243..244,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        487,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 244,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        475,
                                                    ),
                                                    ropd: 245,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost_feet`,
                                                    token_idx: TokenIdx(
                                                        491,
                                                    ),
                                                    current_symbol_idx: 9,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 12,
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                490,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 247,
                                                    dot_token_idx: TokenIdx(
                                                        492,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            493,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        494,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        248..248,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        495,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 248,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        490,
                                                    ),
                                                    ropd: 249,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                496,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                498,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                500,
                                                            ),
                                                            ident: `One`,
                                                        },
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `downmost_hat_dp`,
                                                    token_idx: TokenIdx(
                                                        504,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 11,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost_feet_dp`,
                                                    token_idx: TokenIdx(
                                                        508,
                                                    ),
                                                    current_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 13,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 252,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        499,
                                                    ),
                                                    ropd: 253,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        502,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 254,
                                                    dot_token_idx: TokenIdx(
                                                        505,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            506,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 255,
                                                    dot_token_idx: TokenIdx(
                                                        509,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            510,
                                                        ),
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 251,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        497,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        256..260,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            501,
                                                        ),
                                                        TokenIdx(
                                                            503,
                                                        ),
                                                        TokenIdx(
                                                            507,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        511,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 260,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        512,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `downmost_number_of_strokes`,
                                                    token_idx: TokenIdx(
                                                        514,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        516,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 262,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        515,
                                                    ),
                                                    ropd: 263,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 24,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                519,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 265,
                                                    dot_token_idx: TokenIdx(
                                                        521,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `lower_mass`,
                                                        token_idx: TokenIdx(
                                                            522,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 266,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        519,
                                                    ),
                                                    ropd: 267,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 25,
                                                    entity_path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                525,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 269,
                                                    dot_token_idx: TokenIdx(
                                                        527,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `upper_mass`,
                                                        token_idx: TokenIdx(
                                                            528,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 270,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        525,
                                                    ),
                                                    ropd: 271,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        532,
                                                    ),
                                                    current_symbol_idx: 11,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 14,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `b`,
                                                    token_idx: TokenIdx(
                                                        534,
                                                    ),
                                                    current_symbol_idx: 12,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 15,
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                531,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 273,
                                                    opr: PureClosed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        533,
                                                    ),
                                                    ropd: 274,
                                                },
                                                Expr::Binary {
                                                    lopd: 275,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        531,
                                                    ),
                                                    ropd: 276,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `c`,
                                                    token_idx: TokenIdx(
                                                        538,
                                                    ),
                                                    current_symbol_idx: 13,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 16,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        540,
                                                    ),
                                                    current_symbol_idx: 11,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 14,
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                537,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 278,
                                                    opr: PureClosed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        539,
                                                    ),
                                                    ropd: 279,
                                                },
                                                Expr::Binary {
                                                    lopd: 280,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        537,
                                                    ),
                                                    ropd: 281,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        545,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `c`,
                                                    token_idx: TokenIdx(
                                                        542,
                                                    ),
                                                    current_symbol_idx: 13,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 16,
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        544,
                                                    ),
                                                    opd: 283,
                                                },
                                                Expr::Binary {
                                                    lopd: 284,
                                                    opr: Comparison(
                                                        Geq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        543,
                                                    ),
                                                    ropd: 285,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `c`,
                                                    token_idx: TokenIdx(
                                                        547,
                                                    ),
                                                    current_symbol_idx: 13,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 16,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        549,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 287,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        548,
                                                    ),
                                                    ropd: 288,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                550,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                552,
                                                            ),
                                                            ident: `One`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 290,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        551,
                                                    ),
                                                    ropd: 291,
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
                                                        90,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        94,
                                                    ),
                                                    ident: `ignored_connected_components_row_span_sum_sum`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        100,
                                                    ),
                                                    ident: `fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        102,
                                                    ),
                                                    ident: `major_concave_components`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        122,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        128,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        135,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        146,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        152,
                                                    ),
                                                    ident: `ignored_connected_components_row_span_sum_sum`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        158,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        167,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        176,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        213,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        217,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        240,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        253,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        260,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        264,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        268,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        282,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        291,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        415,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        424,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        433,
                                                    ),
                                                    ident: `one_fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::one_fermi_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        520,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        526,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
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
                                                            134,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        36,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 28,
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                127,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            32,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonToken(
                                                                TokenIdx(
                                                                    133,
                                                                ),
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
                                                    expr_idx: 39,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            187,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        70,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 80,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            210,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        89,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 92,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            311,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        150,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            325,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 7,
                                                            variables: ArenaIdxRange(
                                                                5..6,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    327,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        160,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            340,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 8,
                                                            variables: ArenaIdxRange(
                                                                6..7,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    342,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        164,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            348,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        168,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 182,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 196,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 221,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            145,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        43,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            151,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        46,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            155,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 1,
                                                            variables: ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    157,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        52,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            164,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 2,
                                                            variables: ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    166,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        58,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            173,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 3,
                                                            variables: ArenaIdxRange(
                                                                3..4,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    175,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        64,
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                182,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            66,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonToken(
                                                                TokenIdx(
                                                                    186,
                                                                ),
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
                                                            225,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 5,
                                                            variables: ArenaIdxRange(
                                                                4..5,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    227,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        101,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            239,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        105,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 121,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 139,
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                306,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            141,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonToken(
                                                                TokenIdx(
                                                                    310,
                                                                ),
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
                                                                402,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            198,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonToken(
                                                                TokenIdx(
                                                                    406,
                                                                ),
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
                                                            448,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        224,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            452,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 10,
                                                            variables: ArenaIdxRange(
                                                                7..8,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    454,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        232,
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
                                                            pattern: 11,
                                                            variables: ArenaIdxRange(
                                                                8..9,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    467,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        236,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            473,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 12,
                                                            variables: ArenaIdxRange(
                                                                9..10,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    475,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        246,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            488,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 13,
                                                            variables: ArenaIdxRange(
                                                                10..11,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    490,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        250,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 261,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            513,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        264,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            517,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 14,
                                                            variables: ArenaIdxRange(
                                                                11..12,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    519,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        268,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            523,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 15,
                                                            variables: ArenaIdxRange(
                                                                12..13,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    525,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        272,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            529,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 16,
                                                            variables: ArenaIdxRange(
                                                                13..14,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    531,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        277,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            535,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 17,
                                                            variables: ArenaIdxRange(
                                                                14..15,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    537,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        282,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            541,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        286,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            546,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        289,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 292,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 9,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            97,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 0,
                                                            variables: ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    99,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        15,
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                107,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            19,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonToken(
                                                                TokenIdx(
                                                                    113,
                                                                ),
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
                                                                    143,
                                                                ),
                                                            },
                                                            eol_colon: Ok(
                                                                EolColonToken(
                                                                    TokenIdx(
                                                                        144,
                                                                    ),
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
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `simp_one_match`,
                                                            token_idx: TokenIdx(
                                                                98,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `downmost`,
                                                            token_idx: TokenIdx(
                                                                156,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `upmost`,
                                                            token_idx: TokenIdx(
                                                                165,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `hat`,
                                                            token_idx: TokenIdx(
                                                                174,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                185,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `downmost_number_of_strokes`,
                                                            token_idx: TokenIdx(
                                                                226,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                309,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `long_vertical`,
                                                            token_idx: TokenIdx(
                                                                326,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `long_vertical_dp`,
                                                            token_idx: TokenIdx(
                                                                341,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                405,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `downmost_hat`,
                                                            token_idx: TokenIdx(
                                                                453,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `downmost_hat_dp`,
                                                            token_idx: TokenIdx(
                                                                466,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `downmost_feet`,
                                                            token_idx: TokenIdx(
                                                                474,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `downmost_feet_dp`,
                                                            token_idx: TokenIdx(
                                                                489,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `a`,
                                                            token_idx: TokenIdx(
                                                                518,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `b`,
                                                            token_idx: TokenIdx(
                                                                524,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `c`,
                                                            token_idx: TokenIdx(
                                                                530,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `d`,
                                                            token_idx: TokenIdx(
                                                                536,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Let,
                                                Let,
                                                Let,
                                                Let,
                                                Be,
                                                Let,
                                                Be,
                                                Let,
                                                Let,
                                                Be,
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
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 423,
                                                                },
                                                            ),
                                                        ),
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 418,
                                                                },
                                                            ),
                                                        ),
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 419,
                                                                },
                                                            ),
                                                        ),
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 420,
                                                                },
                                                            ),
                                                        ),
                                                        3,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 406,
                                                                },
                                                            ),
                                                        ),
                                                        4,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 424,
                                                                },
                                                            ),
                                                        ),
                                                        5,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 409,
                                                                },
                                                            ),
                                                        ),
                                                        6,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 425,
                                                                },
                                                            ),
                                                        ),
                                                        7,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 426,
                                                                },
                                                            ),
                                                        ),
                                                        8,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 409,
                                                                },
                                                            ),
                                                        ),
                                                        9,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 427,
                                                                },
                                                            ),
                                                        ),
                                                        10,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 428,
                                                                },
                                                            ),
                                                        ),
                                                        11,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 429,
                                                                },
                                                            ),
                                                        ),
                                                        12,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 430,
                                                                },
                                                            ),
                                                        ),
                                                        13,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 28,
                                                                },
                                                            ),
                                                        ),
                                                        14,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 239,
                                                                },
                                                            ),
                                                        ),
                                                        15,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 411,
                                                                },
                                                            ),
                                                        ),
                                                        16,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 431,
                                                                },
                                                            ),
                                                        ),
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
                                                            99,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    553,
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
                                                            157,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    553,
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
                                                            166,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    553,
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
                                                            175,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    553,
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
                                                            227,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    553,
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
                                                            327,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    402,
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
                                                            342,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    402,
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
                                                            454,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    553,
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
                                                            467,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    553,
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
                                                            475,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    553,
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
                                                            490,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    553,
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
                                                            519,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    553,
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
                                                            525,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    553,
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
                                                            531,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    553,
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
                                                            537,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    553,
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
                                                expr: 293,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    293,
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
                            FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                                decl: FunctionDecl {
                                    path: FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                                    ast_idx: 70,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            558,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
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
                                                            562,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            559,
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
                                                            563,
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
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    556,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 223,
                                                                    },
                                                                ),
                                                            ),
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
                                                                557,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                ident: `cc`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    RegularParameter {
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
                                    implicit_parameter_decl_list: Ok(
                                        None,
                                    ),
                                    parameter_decl_list: Ok(
                                        ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    555,
                                                ),
                                            ),
                                            parameters: [
                                                ExplicitParameterDeclPattern {
                                                    pattern: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon: ColonToken(
                                                        TokenIdx(
                                                            557,
                                                        ),
                                                    ),
                                                    ty: 1,
                                                },
                                            ],
                                            commas: [],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rpar: Ok(
                                                RightParenthesisToken(
                                                    TokenIdx(
                                                        560,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                561,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 3,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                564,
                                            ),
                                        ),
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
                                                                    FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            Expr::EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::Prefix {
                                                                opr: Ref,
                                                                opr_token_idx: TokenIdx(
                                                                    558,
                                                                ),
                                                                opd: 0,
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 1,
                                                                entity_path: Some(
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
                                                                    562,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    559,
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
                                                                    563,
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
                                                                PatternExpr::Identifier {
                                                                    ident_token: IdentifierToken {
                                                                        ident: `cc`,
                                                                        token_idx: TokenIdx(
                                                                            556,
                                                                        ),
                                                                    },
                                                                    liason: None,
                                                                },
                                                            ],
                                                        },
                                                        pattern_infos: [
                                                            Parameter,
                                                        ],
                                                        pattern_symbol_maps: [
                                                            [
                                                                (
                                                                    Identifier(
                                                                        Word(
                                                                            Id {
                                                                                value: 223,
                                                                            },
                                                                        ),
                                                                    ),
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
                                                                        557,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                                        ident: `cc`,
                                                                        pattern_symbol_idx: 0,
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        allow_self_type: False,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [
                                                            RegularParameter {
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
                                                        FormPath(`mnist_classifier::digits::one::upmost`, `Function`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        568,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                567,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        569,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            570,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        571,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        572,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 1,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        567,
                                                    ),
                                                    ropd: 2,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        574,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 4,
                                                    dot_token_idx: TokenIdx(
                                                        575,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            576,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        578,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 5,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        577,
                                                    ),
                                                    ropd: 6,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        579,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 8,
                                                    dot_token_idx: TokenIdx(
                                                        580,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            581,
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
                                                            565,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 0,
                                                            variables: ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    567,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        3,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            573,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        7,
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
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `dp`,
                                                            token_idx: TokenIdx(
                                                                566,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Let,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 343,
                                                                },
                                                            ),
                                                        ),
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
                                                        kind: InheritedSymbolKind::RegularParameter {
                                                            ident: `cc`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            567,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    582,
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
                                body: Ok(
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
                            FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                                decl: FunctionDecl {
                                    path: FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                                    ast_idx: 71,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            587,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
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
                                                            591,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            588,
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
                                                            592,
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
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    585,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 223,
                                                                    },
                                                                ),
                                                            ),
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
                                                                586,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                ident: `cc`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    RegularParameter {
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
                                    implicit_parameter_decl_list: Ok(
                                        None,
                                    ),
                                    parameter_decl_list: Ok(
                                        ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    584,
                                                ),
                                            ),
                                            parameters: [
                                                ExplicitParameterDeclPattern {
                                                    pattern: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon: ColonToken(
                                                        TokenIdx(
                                                            586,
                                                        ),
                                                    ),
                                                    ty: 1,
                                                },
                                            ],
                                            commas: [],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rpar: Ok(
                                                RightParenthesisToken(
                                                    TokenIdx(
                                                        589,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                590,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 3,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                593,
                                            ),
                                        ),
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
                                                                    FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            Expr::EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::Prefix {
                                                                opr: Ref,
                                                                opr_token_idx: TokenIdx(
                                                                    587,
                                                                ),
                                                                opd: 0,
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 1,
                                                                entity_path: Some(
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
                                                                    591,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    588,
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
                                                                    592,
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
                                                                PatternExpr::Identifier {
                                                                    ident_token: IdentifierToken {
                                                                        ident: `cc`,
                                                                        token_idx: TokenIdx(
                                                                            585,
                                                                        ),
                                                                    },
                                                                    liason: None,
                                                                },
                                                            ],
                                                        },
                                                        pattern_infos: [
                                                            Parameter,
                                                        ],
                                                        pattern_symbol_maps: [
                                                            [
                                                                (
                                                                    Identifier(
                                                                        Word(
                                                                            Id {
                                                                                value: 223,
                                                                            },
                                                                        ),
                                                                    ),
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
                                                                        586,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                                        ident: `cc`,
                                                                        pattern_symbol_idx: 0,
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        allow_self_type: False,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [
                                                            RegularParameter {
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
                                                        FormPath(`mnist_classifier::digits::one::downmost`, `Function`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        597,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                596,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        598,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            599,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        600,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        601,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 1,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        596,
                                                    ),
                                                    ropd: 2,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        603,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 4,
                                                    dot_token_idx: TokenIdx(
                                                        604,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            605,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        607,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 5,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        606,
                                                    ),
                                                    ropd: 6,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        609,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 8,
                                                    dot_token_idx: TokenIdx(
                                                        610,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `end`,
                                                        token_idx: TokenIdx(
                                                            611,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        612,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        9..9,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        613,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 9,
                                                    dot_token_idx: TokenIdx(
                                                        614,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            615,
                                                        ),
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        608,
                                                    ),
                                                    opd: 10,
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
                                                            594,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 0,
                                                            variables: ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    596,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        3,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            602,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        7,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 11,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `dp`,
                                                            token_idx: TokenIdx(
                                                                595,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Let,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 343,
                                                                },
                                                            ),
                                                        ),
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
                                                        kind: InheritedSymbolKind::RegularParameter {
                                                            ident: `cc`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            596,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    616,
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
                                                expr: 12,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    12,
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
                            FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                                decl: FunctionDecl {
                                    path: FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                                    ast_idx: 72,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::Prefix {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            621,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
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
                                                            625,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            622,
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
                                                            626,
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
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    619,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 223,
                                                                    },
                                                                ),
                                                            ),
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
                                                                620,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                ident: `cc`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    RegularParameter {
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
                                    implicit_parameter_decl_list: Ok(
                                        None,
                                    ),
                                    parameter_decl_list: Ok(
                                        ExplicitParameterDeclList {
                                            lpar: LeftParenthesisToken(
                                                TokenIdx(
                                                    618,
                                                ),
                                            ),
                                            parameters: [
                                                ExplicitParameterDeclPattern {
                                                    pattern: 0,
                                                    variables: ArenaIdxRange(
                                                        0..1,
                                                    ),
                                                    colon: ColonToken(
                                                        TokenIdx(
                                                            620,
                                                        ),
                                                    ),
                                                    ty: 1,
                                                },
                                            ],
                                            commas: [],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rpar: Ok(
                                                RightParenthesisToken(
                                                    TokenIdx(
                                                        623,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                624,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 3,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                627,
                                            ),
                                        ),
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
                                                                    FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    expr_arena: Arena {
                                                        data: [
                                                            Expr::EntityPath {
                                                                entity_path_expr: 0,
                                                                entity_path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::Prefix {
                                                                opr: Ref,
                                                                opr_token_idx: TokenIdx(
                                                                    621,
                                                                ),
                                                                opd: 0,
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 1,
                                                                entity_path: Some(
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
                                                                    625,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    622,
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
                                                                    626,
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
                                                                PatternExpr::Identifier {
                                                                    ident_token: IdentifierToken {
                                                                        ident: `cc`,
                                                                        token_idx: TokenIdx(
                                                                            619,
                                                                        ),
                                                                    },
                                                                    liason: None,
                                                                },
                                                            ],
                                                        },
                                                        pattern_infos: [
                                                            Parameter,
                                                        ],
                                                        pattern_symbol_maps: [
                                                            [
                                                                (
                                                                    Identifier(
                                                                        Word(
                                                                            Id {
                                                                                value: 223,
                                                                            },
                                                                        ),
                                                                    ),
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
                                                                        620,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                                        ident: `cc`,
                                                                        pattern_symbol_idx: 0,
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        allow_self_type: False,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [
                                                            RegularParameter {
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
                                                        FormPath(`mnist_classifier::digits::one::hat`, `Function`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        631,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                630,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        632,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            633,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        634,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        635,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 1,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        630,
                                                    ),
                                                    ropd: 2,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        637,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 4,
                                                    dot_token_idx: TokenIdx(
                                                        638,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            639,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        641,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 5,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        640,
                                                    ),
                                                    ropd: 6,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        643,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 8,
                                                    dot_token_idx: TokenIdx(
                                                        644,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `x`,
                                                        token_idx: TokenIdx(
                                                            645,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        647,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 9,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        646,
                                                    ),
                                                    ropd: 10,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        649,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 12,
                                                    dot_token_idx: TokenIdx(
                                                        650,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            651,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        653,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        648,
                                                    ),
                                                    opd: 13,
                                                },
                                                Expr::Field {
                                                    owner: 14,
                                                    dot_token_idx: TokenIdx(
                                                        654,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `x`,
                                                        token_idx: TokenIdx(
                                                            655,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 15,
                                                    opr: PureClosed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        652,
                                                    ),
                                                    ropd: 16,
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
                                                            628,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 0,
                                                            variables: ArenaIdxRange(
                                                                0..1,
                                                            ),
                                                            colon_token: Ok(
                                                                None,
                                                            ),
                                                            ty: None,
                                                        },
                                                    ),
                                                    assign_token: Err(
                                                        Original(
                                                            ExpectAssign(
                                                                TokenIdx(
                                                                    630,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        3,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            636,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        7,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            642,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        11,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 17,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `dp`,
                                                            token_idx: TokenIdx(
                                                                629,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Let,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 343,
                                                                },
                                                            ),
                                                        ),
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
                                                        kind: InheritedSymbolKind::RegularParameter {
                                                            ident: `cc`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            630,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    656,
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
                                                expr: 18,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    18,
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)