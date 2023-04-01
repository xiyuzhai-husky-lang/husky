Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::digits::two::two_match`, `Feature`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Feature(
                            FeatureDefn {
                                path: FormPath(`mnist_classifier::digits::two::two_match`, `Feature`),
                                decl: FeatureDecl {
                                    path: FormPath(`mnist_classifier::digits::two::two_match`, `Feature`),
                                    ast_idx: 54,
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                48,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 0,
                                        },
                                    ),
                                    eol_colon: EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                50,
                                            ),
                                        },
                                    ),
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::two::two_match`, `Feature`),
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
                                                                    49,
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
                                                        FormPath(`mnist_classifier::digits::two::two_match`, `Feature`),
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
                                                                FormPath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        55,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        1..4,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        61,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 0,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        52,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        4..6,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            54,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        62,
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
                                                        51,
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
                                                        53,
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
                                                        56,
                                                    ),
                                                    ident: `left_cc_pattern`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        58,
                                                    ),
                                                    ident: `right_cc_pattern`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        60,
                                                    ),
                                                    ident: `down_cc_pattern`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
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
                            FormPath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                                decl: FnDecl {
                                    path: FormPath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                                    ast_idx: 55,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
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
                                                            68,
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
                                                            72,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            69,
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
                                                            73,
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
                                                                    66,
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
                                                                67,
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
                                                65,
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
                                                        67,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                70,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                71,
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
                                                74,
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
                                                                    FormPath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
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
                                                                    68,
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
                                                                    72,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    69,
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
                                                                    73,
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
                                                                            66,
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
                                                                        67,
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
                                                        FormPath(`mnist_classifier::digits::two::left_cc_pattern`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        78,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        79,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            80,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        81,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        82,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        84,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        85,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            86,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        88,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 3,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        87,
                                                    ),
                                                    ropd: 4,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        89,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 6,
                                                    dot_token_idx: TokenIdx(
                                                        90,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            91,
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
                                                            75,
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
                                                    assign_token: Ok(
                                                        AssignToken(
                                                            TokenIdx(
                                                                77,
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
                                                            83,
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
                                                                76,
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
                                                            77,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    92,
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
                                body: Ok(
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
                            FormPath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                                decl: FnDecl {
                                    path: FormPath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                                    ast_idx: 56,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
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
                                                            97,
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
                                                            101,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            98,
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
                                                            102,
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
                                                                    95,
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
                                                                96,
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
                                                94,
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
                                                        96,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                99,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                100,
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
                                                103,
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
                                                                    FormPath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
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
                                                                    97,
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
                                                                    101,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    98,
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
                                                                    102,
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
                                                                            95,
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
                                                                        96,
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
                                                        FormPath(`mnist_classifier::digits::two::right_cc_pattern`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        107,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        108,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            109,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        110,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        111,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        113,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        114,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            115,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        117,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 3,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        116,
                                                    ),
                                                    ropd: 4,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        118,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 6,
                                                    dot_token_idx: TokenIdx(
                                                        119,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            120,
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
                                                            104,
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
                                                    assign_token: Ok(
                                                        AssignToken(
                                                            TokenIdx(
                                                                106,
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
                                                            112,
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
                                                                105,
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
                                                            106,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    121,
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
                                body: Ok(
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
                            FormPath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
                                decl: FnDecl {
                                    path: FormPath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
                                    ast_idx: 57,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
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
                                                            126,
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
                                                            130,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            127,
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
                                                            131,
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
                                                                    124,
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
                                                                125,
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
                                                123,
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
                                                        125,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                128,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                129,
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
                                                132,
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
                                                                    FormPath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
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
                                                                    126,
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
                                                                    130,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    127,
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
                                                                    131,
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
                                                                            124,
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
                                                                        125,
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
                                                        FormPath(`mnist_classifier::digits::two::down_cc_pattern`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        136,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        137,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            138,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        139,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        140,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        142,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        143,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `x`,
                                                        token_idx: TokenIdx(
                                                            144,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        146,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 3,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        145,
                                                    ),
                                                    ropd: 4,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        147,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 6,
                                                    dot_token_idx: TokenIdx(
                                                        148,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `x`,
                                                        token_idx: TokenIdx(
                                                            149,
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
                                                            133,
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
                                                    assign_token: Ok(
                                                        AssignToken(
                                                            TokenIdx(
                                                                135,
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
                                                            141,
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
                                                                134,
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
                                                            135,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    150,
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
                                body: Ok(
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
                            FormPath(`mnist_classifier::digits::two::is_two`, `Feature`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Feature(
                            FeatureDefn {
                                path: FormPath(`mnist_classifier::digits::two::is_two`, `Feature`),
                                decl: FeatureDecl {
                                    path: FormPath(`mnist_classifier::digits::two::is_two`, `Feature`),
                                    ast_idx: 58,
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                156,
                                            ),
                                        ),
                                    ),
                                    return_ty: Some(
                                        ReturnTypeExpr {
                                            expr: 1,
                                        },
                                    ),
                                    eol_colon: EolToken::Colon(
                                        EolColonToken {
                                            token_idx: TokenIdx(
                                                159,
                                            ),
                                        },
                                    ),
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::two::is_two`, `Feature`),
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
                                                            157,
                                                        ),
                                                        opd: 0,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            158,
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
                                                        FormPath(`mnist_classifier::digits::two::is_two`, `Feature`),
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
                                                                161,
                                                            ),
                                                            ident: `is_zero`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Be {
                                                    src: 0,
                                                    be_token_idx: TokenIdx(
                                                        162,
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
                                                                165,
                                                            ),
                                                            ident: `is_three`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Be {
                                                    src: 2,
                                                    be_token_idx: TokenIdx(
                                                        166,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 1,
                                                        },
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                169,
                                                            ),
                                                            ident: `is_seven`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Be {
                                                    src: 4,
                                                    be_token_idx: TokenIdx(
                                                        170,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 2,
                                                        },
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 6,
                                                    be_token_idx: TokenIdx(
                                                        174,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 3,
                                                        },
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                177,
                                                            ),
                                                            ident: `is_nine`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Be {
                                                    src: 8,
                                                    be_token_idx: TokenIdx(
                                                        178,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 4,
                                                        },
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdent {
                                                            token_idx: TokenIdx(
                                                                181,
                                                            ),
                                                            ident: `is_six`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Be {
                                                    src: 10,
                                                    be_token_idx: TokenIdx(
                                                        182,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 5,
                                                        },
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 12,
                                                    dot_token_idx: TokenIdx(
                                                        188,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ilen`,
                                                        token_idx: TokenIdx(
                                                            189,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        190,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        13..13,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        191,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 14,
                                                    dot_token_idx: TokenIdx(
                                                        196,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `eff_holes`,
                                                        token_idx: TokenIdx(
                                                            197,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `eff_holes`,
                                                    token_idx: TokenIdx(
                                                        199,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 16,
                                                    dot_token_idx: TokenIdx(
                                                        200,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            201,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        203,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 17,
                                                    lbox_token_idx: TokenIdx(
                                                        202,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        18..19,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        204,
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 19,
                                                    be_token_idx: TokenIdx(
                                                        205,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 8,
                                                        },
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::two::two_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 21,
                                                    dot_token_idx: TokenIdx(
                                                        211,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            212,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        214,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 22,
                                                    lbox_token_idx: TokenIdx(
                                                        213,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        23..24,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        215,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::two::two_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 25,
                                                    dot_token_idx: TokenIdx(
                                                        220,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            221,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        223,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 26,
                                                    lbox_token_idx: TokenIdx(
                                                        222,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        27..28,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        224,
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 5,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::two::two_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 29,
                                                    dot_token_idx: TokenIdx(
                                                        229,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            230,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        232,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 30,
                                                    lbox_token_idx: TokenIdx(
                                                        231,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        31..32,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        233,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `cc_num`,
                                                    token_idx: TokenIdx(
                                                        235,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        237,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 33,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        236,
                                                    ),
                                                    ropd: 34,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 6,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 7,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 36,
                                                    dot_token_idx: TokenIdx(
                                                        242,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `lower_mass`,
                                                        token_idx: TokenIdx(
                                                            243,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 37,
                                                    dot_token_idx: TokenIdx(
                                                        246,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `upper_mass`,
                                                        token_idx: TokenIdx(
                                                            247,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 38,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        244,
                                                    ),
                                                    ropd: 39,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `lower_excess`,
                                                    token_idx: TokenIdx(
                                                        249,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 12,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        251,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 41,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        250,
                                                    ),
                                                    ropd: 42,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `cc_num`,
                                                    token_idx: TokenIdx(
                                                        253,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        255,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 44,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        254,
                                                    ),
                                                    ropd: 45,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `left_cc`,
                                                    token_idx: TokenIdx(
                                                        258,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 47,
                                                    be_token_idx: TokenIdx(
                                                        259,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 13,
                                                        },
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `right_cc`,
                                                    token_idx: TokenIdx(
                                                        262,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 49,
                                                    be_token_idx: TokenIdx(
                                                        263,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 14,
                                                        },
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `right_cc`,
                                                    token_idx: TokenIdx(
                                                        268,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 51,
                                                    dot_token_idx: TokenIdx(
                                                        269,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `angle_change`,
                                                        token_idx: TokenIdx(
                                                            270,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        275,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        272,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 15,
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        274,
                                                    ),
                                                    opd: 53,
                                                },
                                                Expr::Binary {
                                                    lopd: 54,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        273,
                                                    ),
                                                    ropd: 55,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `left_cc`,
                                                    token_idx: TokenIdx(
                                                        279,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 57,
                                                    dot_token_idx: TokenIdx(
                                                        280,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `end_tangent`,
                                                        token_idx: TokenIdx(
                                                            281,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        282,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        58..58,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        283,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        287,
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 58,
                                                    dot_token_idx: TokenIdx(
                                                        284,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `angle`,
                                                        token_idx: TokenIdx(
                                                            285,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        286,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        59..60,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        288,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `left_cc`,
                                                    token_idx: TokenIdx(
                                                        292,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 61,
                                                    dot_token_idx: TokenIdx(
                                                        293,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `end_tangent`,
                                                        token_idx: TokenIdx(
                                                            294,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        295,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        62..62,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        296,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 62,
                                                    dot_token_idx: TokenIdx(
                                                        297,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `x`,
                                                        token_idx: TokenIdx(
                                                            298,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `left_cc`,
                                                    token_idx: TokenIdx(
                                                        302,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 64,
                                                    dot_token_idx: TokenIdx(
                                                        303,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `end_tangent`,
                                                        token_idx: TokenIdx(
                                                            304,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        305,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        65..65,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        306,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 65,
                                                    dot_token_idx: TokenIdx(
                                                        307,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            308,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `left_cc`,
                                                    token_idx: TokenIdx(
                                                        312,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 67,
                                                    dot_token_idx: TokenIdx(
                                                        313,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_bounding_box`,
                                                        token_idx: TokenIdx(
                                                            314,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 68,
                                                    dot_token_idx: TokenIdx(
                                                        315,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            316,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        317,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        69..69,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        318,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `left_cc`,
                                                    token_idx: TokenIdx(
                                                        322,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 70,
                                                    dot_token_idx: TokenIdx(
                                                        323,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_bounding_box`,
                                                        token_idx: TokenIdx(
                                                            324,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 71,
                                                    dot_token_idx: TokenIdx(
                                                        325,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymin`,
                                                        token_idx: TokenIdx(
                                                            326,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        327,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        72..72,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        328,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `left_ymax`,
                                                    token_idx: TokenIdx(
                                                        333,
                                                    ),
                                                    current_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 19,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `left_ymin`,
                                                    token_idx: TokenIdx(
                                                        335,
                                                    ),
                                                    current_symbol_idx: 11,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 20,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 73,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        334,
                                                    ),
                                                    ropd: 74,
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        332,
                                                    ),
                                                    item: 75,
                                                    rpar_token_idx: TokenIdx(
                                                        336,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        338,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 76,
                                                    opr: Closed(
                                                        Div,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        337,
                                                    ),
                                                    ropd: 77,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `right_cc`,
                                                    token_idx: TokenIdx(
                                                        342,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 79,
                                                    dot_token_idx: TokenIdx(
                                                        343,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_bounding_box`,
                                                        token_idx: TokenIdx(
                                                            344,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 80,
                                                    dot_token_idx: TokenIdx(
                                                        345,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            346,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        347,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        81..81,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        348,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `right_cc`,
                                                    token_idx: TokenIdx(
                                                        352,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 82,
                                                    dot_token_idx: TokenIdx(
                                                        353,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_bounding_box`,
                                                        token_idx: TokenIdx(
                                                            354,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 83,
                                                    dot_token_idx: TokenIdx(
                                                        355,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymin`,
                                                        token_idx: TokenIdx(
                                                            356,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        357,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        84..84,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        358,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `right_ymax`,
                                                    token_idx: TokenIdx(
                                                        363,
                                                    ),
                                                    current_symbol_idx: 13,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 22,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `right_ymin`,
                                                    token_idx: TokenIdx(
                                                        365,
                                                    ),
                                                    current_symbol_idx: 14,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 23,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 85,
                                                    opr: Closed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        364,
                                                    ),
                                                    ropd: 86,
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        362,
                                                    ),
                                                    item: 87,
                                                    rpar_token_idx: TokenIdx(
                                                        366,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        368,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 88,
                                                    opr: Closed(
                                                        Div,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        367,
                                                    ),
                                                    ropd: 89,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `left_mid_y`,
                                                    token_idx: TokenIdx(
                                                        370,
                                                    ),
                                                    current_symbol_idx: 12,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 21,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `right_mid_y`,
                                                    token_idx: TokenIdx(
                                                        372,
                                                    ),
                                                    current_symbol_idx: 15,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 24,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 91,
                                                    opr: Comparison(
                                                        Geq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        371,
                                                    ),
                                                    ropd: 92,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `cc_num`,
                                                    token_idx: TokenIdx(
                                                        374,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        376,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 94,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        375,
                                                    ),
                                                    ropd: 95,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `left_cc`,
                                                    token_idx: TokenIdx(
                                                        379,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 97,
                                                    be_token_idx: TokenIdx(
                                                        380,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 25,
                                                        },
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `right_cc`,
                                                    token_idx: TokenIdx(
                                                        383,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 10,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 99,
                                                    be_token_idx: TokenIdx(
                                                        384,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 26,
                                                        },
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `down_cc`,
                                                    token_idx: TokenIdx(
                                                        387,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 11,
                                                    },
                                                },
                                                Expr::Be {
                                                    src: 101,
                                                    be_token_idx: TokenIdx(
                                                        388,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 27,
                                                        },
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `down_cc`,
                                                    token_idx: TokenIdx(
                                                        391,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 11,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 103,
                                                    dot_token_idx: TokenIdx(
                                                        392,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_bounding_box`,
                                                        token_idx: TokenIdx(
                                                            393,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 104,
                                                    dot_token_idx: TokenIdx(
                                                        394,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymin`,
                                                        token_idx: TokenIdx(
                                                            395,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        396,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        105..105,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        397,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        399,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 105,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        398,
                                                    ),
                                                    ropd: 106,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `down_cc`,
                                                    token_idx: TokenIdx(
                                                        403,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 11,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 108,
                                                    dot_token_idx: TokenIdx(
                                                        404,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `angle_change`,
                                                        token_idx: TokenIdx(
                                                            405,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 9,
                                                    path: None,
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        19..37,
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        173,
                                                    ),
                                                    ident: `is_one`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        187,
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
                                                        195,
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
                                                        210,
                                                    ),
                                                    ident: `two_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::two::two_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        219,
                                                    ),
                                                    ident: `two_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::two::two_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        228,
                                                    ),
                                                    ident: `two_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::two::two_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        241,
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
                                                        245,
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
                                                        406,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 8,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            407,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Two`,
                                                            token_idx: TokenIdx(
                                                                408,
                                                            ),
                                                        },
                                                    ),
                                                    path: Err(
                                                        EntityPathExprError::Original(
                                                            OriginalEntityPathExprError::EntityTree {
                                                                token_idx: TokenIdx(
                                                                    408,
                                                                ),
                                                                error: EntityTreeError::Original(
                                                                    OriginalEntityTreeError::NoSubentity,
                                                                ),
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
                                                            257,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        48,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            261,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        50,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            265,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 15,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                267,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        52,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            271,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        56,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            276,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 16,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                278,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        60,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            289,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 17,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                291,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        63,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            299,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 18,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                301,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        66,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            309,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 19,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                311,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        69,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            319,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 20,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                321,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        72,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            329,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 21,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                331,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        78,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            339,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 22,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                341,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        81,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            349,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 23,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                351,
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
                                                            359,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 24,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                361,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        90,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            369,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        93,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            378,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        98,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            382,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        100,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            386,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        102,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            390,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        107,
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
                                                            pattern: 28,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                402,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        109,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            160,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        1,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            164,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        3,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            168,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        5,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            172,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        7,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            176,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        9,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            180,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        11,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            184,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 6,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                186,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        13,
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
                                                            pattern: 7,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                194,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        15,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            198,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        20,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            207,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 9,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                209,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        24,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            216,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 10,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                218,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        28,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            225,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 11,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                227,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        32,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            234,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        35,
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
                                                            pattern: 12,
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
                                                        AssignToken(
                                                            TokenIdx(
                                                                240,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        40,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            248,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        43,
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                252,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            46,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        256,
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
                                                                373,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            96,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        377,
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
                                                data: [
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                163,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                167,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                171,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                175,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                179,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                183,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `cc_num`,
                                                            token_idx: TokenIdx(
                                                                185,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `eff_holes`,
                                                            token_idx: TokenIdx(
                                                                193,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                206,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `left_cc`,
                                                            token_idx: TokenIdx(
                                                                208,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `right_cc`,
                                                            token_idx: TokenIdx(
                                                                217,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `down_cc`,
                                                            token_idx: TokenIdx(
                                                                226,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `lower_excess`,
                                                            token_idx: TokenIdx(
                                                                239,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                260,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                264,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `a`,
                                                            token_idx: TokenIdx(
                                                                266,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `end_tan`,
                                                            token_idx: TokenIdx(
                                                                277,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `x`,
                                                            token_idx: TokenIdx(
                                                                290,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `y`,
                                                            token_idx: TokenIdx(
                                                                300,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `left_ymax`,
                                                            token_idx: TokenIdx(
                                                                310,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `left_ymin`,
                                                            token_idx: TokenIdx(
                                                                320,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `left_mid_y`,
                                                            token_idx: TokenIdx(
                                                                330,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `right_ymax`,
                                                            token_idx: TokenIdx(
                                                                340,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `right_ymin`,
                                                            token_idx: TokenIdx(
                                                                350,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `right_mid_y`,
                                                            token_idx: TokenIdx(
                                                                360,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                381,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                385,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                389,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier: None,
                                                        ident_token: IdentToken {
                                                            ident: `a`,
                                                            token_idx: TokenIdx(
                                                                401,
                                                            ),
                                                        },
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Be,
                                                Be,
                                                Be,
                                                Be,
                                                Be,
                                                Be,
                                                Let,
                                                Let,
                                                Be,
                                                Let,
                                                Let,
                                                Let,
                                                Let,
                                                Be,
                                                Be,
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
                                                Be,
                                                Be,
                                                Be,
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
                                                        `none`,
                                                        1,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `none`,
                                                        2,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `none`,
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
                                                        `none`,
                                                        5,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `cc_num`,
                                                        6,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `eff_holes`,
                                                        7,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `none`,
                                                        8,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `left_cc`,
                                                        9,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `right_cc`,
                                                        10,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `down_cc`,
                                                        11,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `lower_excess`,
                                                        12,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        13,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        14,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `a`,
                                                        15,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `end_tan`,
                                                        16,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `x`,
                                                        17,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `y`,
                                                        18,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `left_ymax`,
                                                        19,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `left_ymin`,
                                                        20,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `left_mid_y`,
                                                        21,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `right_ymax`,
                                                        22,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `right_ymin`,
                                                        23,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `right_mid_y`,
                                                        24,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        25,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        26,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `some`,
                                                        27,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `a`,
                                                        28,
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
                                                    PatternSymbol::Atom(
                                                        18,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        19,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        20,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        21,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        22,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        23,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        24,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        25,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        26,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        27,
                                                    ),
                                                    PatternSymbol::Atom(
                                                        28,
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
                                                            186,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    409,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `cc_num`,
                                                            pattern_symbol_idx: 6,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            194,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    409,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `eff_holes`,
                                                            pattern_symbol_idx: 7,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            209,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    409,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `left_cc`,
                                                            pattern_symbol_idx: 9,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            218,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    409,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `right_cc`,
                                                            pattern_symbol_idx: 10,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            227,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    409,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `down_cc`,
                                                            pattern_symbol_idx: 11,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            240,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    409,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `lower_excess`,
                                                            pattern_symbol_idx: 12,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            267,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    373,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `a`,
                                                            pattern_symbol_idx: 15,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            278,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    373,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `end_tan`,
                                                            pattern_symbol_idx: 16,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            291,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    373,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `x`,
                                                            pattern_symbol_idx: 17,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            301,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    373,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `y`,
                                                            pattern_symbol_idx: 18,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            311,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    373,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `left_ymax`,
                                                            pattern_symbol_idx: 19,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            321,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    373,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `left_ymin`,
                                                            pattern_symbol_idx: 20,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            331,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    373,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `left_mid_y`,
                                                            pattern_symbol_idx: 21,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            341,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    373,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `right_ymax`,
                                                            pattern_symbol_idx: 22,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            351,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    373,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `right_ymin`,
                                                            pattern_symbol_idx: 23,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            361,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    373,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `right_mid_y`,
                                                            pattern_symbol_idx: 24,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            402,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    406,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `a`,
                                                            pattern_symbol_idx: 28,
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
                                                expr: 111,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    111,
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)