Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::digits::zero::open_one_match`, `Fn`),
                        ),
                    ),
                ),
                Err(
                    DeclError::Original(
                        OriginalDeclError::Expr(
                            OriginalDeclExprError::ExpectParameterDeclList(
                                TokenIdx(
                                    6,
                                ),
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
                                decl: FnDecl {
                                    path: FormPath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
                                    ast_idx: 27,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
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
                                                                    23,
                                                                ),
                                                                ident: `ConcaveComponent`,
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Prefix {
                                                        opr: Tilde,
                                                        opr_token_idx: TokenIdx(
                                                            22,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
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
                                                            26,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            27,
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
                                                                    20,
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
                                                                21,
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
                                                19,
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
                                                        21,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                24,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                25,
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
                                                28,
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
                                                                    FormPath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
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
                                                                            23,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
                                                                    },
                                                                ),
                                                            ),
                                                            Expr::Prefix {
                                                                opr: Tilde,
                                                                opr_token_idx: TokenIdx(
                                                                    22,
                                                                ),
                                                                opd: 0,
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 0,
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
                                                                    26,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    27,
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
                                                                            20,
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
                                                                        21,
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
                                                        FormPath(`mnist_classifier::digits::zero::almost_closed`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        30,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 0,
                                                    dot_token_idx: TokenIdx(
                                                        31,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `angle_change`,
                                                        token_idx: TokenIdx(
                                                            32,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        34,
                                                    ),
                                                ),
                                                Expr::Literal(
                                                    TokenIdx(
                                                        37,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 1,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        33,
                                                    ),
                                                    ropd: 2,
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        36,
                                                    ),
                                                    opd: 3,
                                                },
                                                Expr::Binary {
                                                    lopd: 4,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        35,
                                                    ),
                                                    ropd: 5,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        39,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 7,
                                                    dot_token_idx: TokenIdx(
                                                        40,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `angle_change`,
                                                        token_idx: TokenIdx(
                                                            41,
                                                        ),
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        38,
                                                    ),
                                                    opd: 8,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        43,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 9,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        42,
                                                    ),
                                                    ropd: 10,
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        0..2,
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
                                                            29,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        6,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 11,
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
                                                data: [],
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
                            FormPath(`mnist_classifier::digits::zero::is_zero`, `Var`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Feature(
                            FeatureDefn {
                                path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Var`),
                                decl: FeatureDecl {
                                    path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Var`),
                                    ast_idx: 28,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                50,
                                            ),
                                        ),
                                    ),
                                    var_ty: Some(
                                        VarTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                    eq_token: EqToken(
                                        TokenIdx(
                                            53,
                                        ),
                                    ),
                                    expr_or_eol_token: Left(
                                        EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    54,
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
                                                            FormPath(`mnist_classifier::digits::zero::is_zero`, `Var`),
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
                                                                    52,
                                                                ),
                                                                ident: `MnistLabel`,
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            51,
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
                                                        FormPath(`mnist_classifier::digits::zero::is_zero`, `Var`),
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
                                                                56,
                                                            ),
                                                            ident: `is_one`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Be {
                                                    src: 0,
                                                    be_token_idx: TokenIdx(
                                                        57,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 0,
                                                        },
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                60,
                                                            ),
                                                            ident: `major_connected_component`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        61,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `raw_contours`,
                                                        token_idx: TokenIdx(
                                                            62,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 3,
                                                    dot_token_idx: TokenIdx(
                                                        63,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ilen`,
                                                        token_idx: TokenIdx(
                                                            64,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        65,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        4..4,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        66,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        68,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 4,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        67,
                                                    ),
                                                    ropd: 5,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::zero::open_one_match`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 7,
                                                    dot_token_idx: TokenIdx(
                                                        74,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            75,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `n`,
                                                    token_idx: TokenIdx(
                                                        77,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        79,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 9,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        78,
                                                    ),
                                                    ropd: 10,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::zero::open_one_match`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 12,
                                                    dot_token_idx: TokenIdx(
                                                        82,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            83,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        85,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 13,
                                                    lbox_token_idx: TokenIdx(
                                                        84,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        14..15,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        86,
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 15,
                                                    be_token_idx: TokenIdx(
                                                        87,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 2,
                                                        },
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                90,
                                                            ),
                                                            ident: `connected_components`,
                                                        },
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 17,
                                                    dot_token_idx: TokenIdx(
                                                        91,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ilen`,
                                                        token_idx: TokenIdx(
                                                            92,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        93,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        18..18,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        94,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        96,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 18,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        95,
                                                    ),
                                                    ropd: 19,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::zero::open_one_match`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 21,
                                                    dot_token_idx: TokenIdx(
                                                        101,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            102,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        104,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 22,
                                                    lbox_token_idx: TokenIdx(
                                                        103,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        23..24,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        105,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 24,
                                                    dot_token_idx: TokenIdx(
                                                        106,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            107,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        108,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        25..25,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        109,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 25,
                                                    dot_token_idx: TokenIdx(
                                                        110,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            111,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        112,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        26..26,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        113,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `c`,
                                                    token_idx: TokenIdx(
                                                        115,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        117,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 27,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        116,
                                                    ),
                                                    ropd: 28,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                118,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                120,
                                                            ),
                                                            ident: `Zero`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 30,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        119,
                                                    ),
                                                    ropd: 31,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                124,
                                                            ),
                                                            ident: `fermi_match`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                126,
                                                            ),
                                                            ident: `major_concave_components`,
                                                        },
                                                    ),
                                                ),
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        128,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        34..34,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        129,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 33,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        125,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        34..36,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            127,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        130,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                131,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                133,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                135,
                                                            ),
                                                            ident: `Zero`,
                                                        },
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `simp_zero_match`,
                                                    token_idx: TokenIdx(
                                                        139,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `simp_zero_match`,
                                                    token_idx: TokenIdx(
                                                        143,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `simp_zero_match`,
                                                    token_idx: TokenIdx(
                                                        147,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 38,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        134,
                                                    ),
                                                    ropd: 39,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        137,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 40,
                                                    dot_token_idx: TokenIdx(
                                                        140,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            141,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 41,
                                                    dot_token_idx: TokenIdx(
                                                        144,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `rel_norm`,
                                                        token_idx: TokenIdx(
                                                            145,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 42,
                                                    dot_token_idx: TokenIdx(
                                                        148,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `angle_change_norm`,
                                                        token_idx: TokenIdx(
                                                            149,
                                                        ),
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 37,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        132,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        43..48,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            136,
                                                        ),
                                                        TokenIdx(
                                                            138,
                                                        ),
                                                        TokenIdx(
                                                            142,
                                                        ),
                                                        TokenIdx(
                                                            146,
                                                        ),
                                                        TokenIdx(
                                                            150,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        151,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 48,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        152,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `simp_zero_match`,
                                                    token_idx: TokenIdx(
                                                        154,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 50,
                                                    dot_token_idx: TokenIdx(
                                                        155,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            156,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        158,
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
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                160,
                                                            ),
                                                            ident: `major_connected_component`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 54,
                                                    dot_token_idx: TokenIdx(
                                                        161,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `eff_holes`,
                                                        token_idx: TokenIdx(
                                                            162,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 55,
                                                    dot_token_idx: TokenIdx(
                                                        163,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            164,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        166,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 56,
                                                    lbox_token_idx: TokenIdx(
                                                        165,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        57..58,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        167,
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 58,
                                                    be_token_idx: TokenIdx(
                                                        168,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 5,
                                                        },
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                171,
                                                            ),
                                                            ident: `major_connected_component`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 60,
                                                    dot_token_idx: TokenIdx(
                                                        172,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `eff_holes`,
                                                        token_idx: TokenIdx(
                                                            173,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 61,
                                                    dot_token_idx: TokenIdx(
                                                        174,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            175,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        177,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 62,
                                                    lbox_token_idx: TokenIdx(
                                                        176,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        63..64,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        178,
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 64,
                                                    be_token_idx: TokenIdx(
                                                        179,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 6,
                                                        },
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                184,
                                                            ),
                                                            ident: `major_connected_component`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 66,
                                                    dot_token_idx: TokenIdx(
                                                        185,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `eff_holes`,
                                                        token_idx: TokenIdx(
                                                            186,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 67,
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
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 68,
                                                    lbox_token_idx: TokenIdx(
                                                        189,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        69..70,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        191,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `major_hole`,
                                                    token_idx: TokenIdx(
                                                        195,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 71,
                                                    dot_token_idx: TokenIdx(
                                                        196,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `bounding_box`,
                                                        token_idx: TokenIdx(
                                                            197,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `major_hole`,
                                                    token_idx: TokenIdx(
                                                        203,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 73,
                                                    dot_token_idx: TokenIdx(
                                                        204,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `bounding_box`,
                                                        token_idx: TokenIdx(
                                                            205,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 72,
                                                    dot_token_idx: TokenIdx(
                                                        198,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            199,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        200,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        73..73,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        201,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 74,
                                                    dot_token_idx: TokenIdx(
                                                        206,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymin`,
                                                        token_idx: TokenIdx(
                                                            207,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        208,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        75..75,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        209,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 75,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        202,
                                                    ),
                                                    ropd: 76,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                213,
                                                            ),
                                                            ident: `major_line_segment_sketch`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 78,
                                                    dot_token_idx: TokenIdx(
                                                        214,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `bounding_box`,
                                                        token_idx: TokenIdx(
                                                            215,
                                                        ),
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                221,
                                                            ),
                                                            ident: `major_line_segment_sketch`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 80,
                                                    dot_token_idx: TokenIdx(
                                                        222,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `bounding_box`,
                                                        token_idx: TokenIdx(
                                                            223,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 79,
                                                    dot_token_idx: TokenIdx(
                                                        216,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            217,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        218,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        80..80,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        219,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 81,
                                                    dot_token_idx: TokenIdx(
                                                        224,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymin`,
                                                        token_idx: TokenIdx(
                                                            225,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        226,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        82..82,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        227,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 82,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        220,
                                                    ),
                                                    ropd: 83,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        231,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `b`,
                                                    token_idx: TokenIdx(
                                                        233,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 85,
                                                    opr: Closed(
                                                        Div,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        232,
                                                    ),
                                                    ropd: 86,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `ratio`,
                                                    token_idx: TokenIdx(
                                                        235,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        237,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 88,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        236,
                                                    ),
                                                    ropd: 89,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `simp_zero_match`,
                                                    token_idx: TokenIdx(
                                                        241,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
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
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                244,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                246,
                                                            ),
                                                            ident: `Zero`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 93,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        245,
                                                    ),
                                                    ropd: 94,
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        7..21,
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        73,
                                                    ),
                                                    ident: `open_one_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::zero::open_one_match`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        81,
                                                    ),
                                                    ident: `open_one_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::zero::open_one_match`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        100,
                                                    ),
                                                    ident: `open_one_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::zero::open_one_match`, `Fn`),
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
                                                            70,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 1,
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
                                                                72,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        8,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            76,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        11,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            80,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        16,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            89,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        20,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            97,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 3,
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
                                                                99,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        26,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            114,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        29,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 32,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            55,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        1,
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                59,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            6,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        69,
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
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 4,
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
                                                                123,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        36,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 49,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            153,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        53,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            159,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        59,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            170,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        65,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            181,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 7,
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
                                                                183,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        70,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            192,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 8,
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
                                                                194,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        77,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            210,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 9,
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
                                                                212,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        84,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            228,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 10,
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
                                                                230,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        87,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            234,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        90,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            238,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 11,
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
                                                                240,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        92,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 95,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                58,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `n`,
                                                            token_idx: TokenIdx(
                                                                71,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                88,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `c`,
                                                            token_idx: TokenIdx(
                                                                98,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `simp_zero_match`,
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
                                                                169,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                180,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `major_hole`,
                                                            token_idx: TokenIdx(
                                                                182,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `a`,
                                                            token_idx: TokenIdx(
                                                                193,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `b`,
                                                            token_idx: TokenIdx(
                                                                211,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `ratio`,
                                                            token_idx: TokenIdx(
                                                                229,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `a`,
                                                            token_idx: TokenIdx(
                                                                239,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Be,
                                                Let,
                                                Be,
                                                Let,
                                                Let,
                                                Be,
                                                Be,
                                                Let,
                                                Let,
                                                Let,
                                                Let,
                                                Let,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        `none`,
                                                        0,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `n`,
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
                                                        `c`,
                                                        3,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `simp_zero_match`,
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
                                                        `some`,
                                                        6,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `major_hole`,
                                                        7,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `a`,
                                                        8,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `b`,
                                                        9,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `ratio`,
                                                        10,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `a`,
                                                        11,
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
                                                            72,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    121,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `n`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            99,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    121,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `c`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            123,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    247,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `simp_zero_match`,
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            183,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    247,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `major_hole`,
                                                            pattern_symbol_idx: 7,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            194,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    247,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `a`,
                                                            pattern_symbol_idx: 8,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            212,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    247,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `b`,
                                                            pattern_symbol_idx: 9,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            230,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    247,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `ratio`,
                                                            pattern_symbol_idx: 10,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            240,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    247,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `a`,
                                                            pattern_symbol_idx: 11,
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
                                                expr: 96,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    96,
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)