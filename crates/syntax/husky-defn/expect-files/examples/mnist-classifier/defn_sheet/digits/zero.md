Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Feature(
                            FeatureDefn {
                                path: FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                                decl: FeatureDecl {
                                    path: FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                                    ast_idx: 33,
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                66,
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
                                                68,
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
                                                            FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
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
                                                                    67,
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
                                                        FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
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
                                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
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
                                                        73,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        75,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 0,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        70,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        2..4,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            72,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        76,
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
                                                        69,
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
                                                        71,
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
                                                        74,
                                                    ),
                                                    ident: `almost_closed`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
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
                                                expr: 5,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    5,
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
                            FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
                                decl: FunctionDecl {
                                    path: FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
                                    ast_idx: 34,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
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
                                                        opr: BitNotOrLeash,
                                                        opr_token_idx: TokenIdx(
                                                            82,
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
                                                            86,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            83,
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
                                                            87,
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
                                                                    80,
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
                                                                        value: 225,
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
                                                                81,
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
                                                    79,
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
                                                            81,
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
                                                        84,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                85,
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
                                                88,
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
                                                                    FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
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
                                                                opr: BitNotOrLeash,
                                                                opr_token_idx: TokenIdx(
                                                                    82,
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
                                                                    86,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    83,
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
                                                                    87,
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
                                                                            80,
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
                                                                                value: 225,
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
                                                                        81,
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
                                                        FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        90,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 0,
                                                    dot_token_idx: TokenIdx(
                                                        91,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `angle_change`,
                                                        token_idx: TokenIdx(
                                                            92,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        94,
                                                    ),
                                                ),
                                                Expr::Literal(
                                                    TokenIdx(
                                                        97,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 1,
                                                    opr: PureClosed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        93,
                                                    ),
                                                    ropd: 2,
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        96,
                                                    ),
                                                    opd: 3,
                                                },
                                                Expr::Binary {
                                                    lopd: 4,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        95,
                                                    ),
                                                    ropd: 5,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        99,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 7,
                                                    dot_token_idx: TokenIdx(
                                                        100,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `angle_change`,
                                                        token_idx: TokenIdx(
                                                            101,
                                                        ),
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        98,
                                                    ),
                                                    opd: 8,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        103,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 9,
                                                    opr: PureClosed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        102,
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
                                                            89,
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
                                                        kind: InheritedSymbolKind::RegularParameter {
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
                            FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Feature(
                            FeatureDefn {
                                path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                                decl: FeatureDecl {
                                    path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                                    ast_idx: 35,
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                106,
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
                                                109,
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
                                                            FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
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
                                                                    108,
                                                                ),
                                                                ident: `MnistLabel`,
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Prefix {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            107,
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
                                                        FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
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
                                                                111,
                                                            ),
                                                            ident: `is_one`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Be {
                                                    src: 0,
                                                    be_token_idx: TokenIdx(
                                                        112,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 0,
                                                        },
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 0,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        116,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `raw_contours`,
                                                        token_idx: TokenIdx(
                                                            117,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 3,
                                                    dot_token_idx: TokenIdx(
                                                        118,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `ilen`,
                                                        token_idx: TokenIdx(
                                                            119,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        120,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        4..4,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        121,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        123,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 4,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        122,
                                                    ),
                                                    ropd: 5,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                125,
                                                            ),
                                                            ident: `n`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 7,
                                                    dot_token_idx: TokenIdx(
                                                        128,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            129,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 8,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        126,
                                                    ),
                                                    ropd: 9,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                131,
                                                            ),
                                                            ident: `n`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Literal(
                                                    TokenIdx(
                                                        133,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 11,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        132,
                                                    ),
                                                    ropd: 12,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 14,
                                                    dot_token_idx: TokenIdx(
                                                        136,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `matches`,
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
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 15,
                                                    lbox_token_idx: TokenIdx(
                                                        138,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        16..17,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        140,
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 17,
                                                    be_token_idx: TokenIdx(
                                                        141,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 1,
                                                        },
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 19,
                                                    dot_token_idx: TokenIdx(
                                                        145,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `ilen`,
                                                        token_idx: TokenIdx(
                                                            146,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        147,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        20..20,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        148,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        150,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 20,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        149,
                                                    ),
                                                    ropd: 21,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 23,
                                                    dot_token_idx: TokenIdx(
                                                        154,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            155,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        157,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 24,
                                                    lbox_token_idx: TokenIdx(
                                                        156,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        25..26,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        158,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 26,
                                                    dot_token_idx: TokenIdx(
                                                        159,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            160,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        161,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        27..27,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        162,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                151,
                                                            ),
                                                            ident: `c`,
                                                        },
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 27,
                                                    dot_token_idx: TokenIdx(
                                                        163,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            164,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        165,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        28..28,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        166,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 28,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        152,
                                                    ),
                                                    ropd: 29,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                168,
                                                            ),
                                                            ident: `c`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Literal(
                                                    TokenIdx(
                                                        170,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 31,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        169,
                                                    ),
                                                    ropd: 32,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                171,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                173,
                                                            ),
                                                            ident: `Zero`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 34,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        172,
                                                    ),
                                                    ropd: 35,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 5,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 6,
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
                                                        180,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        38..38,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        181,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                174,
                                                            ),
                                                            ident: `simp_zero_match`,
                                                        },
                                                    ),
                                                ),
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 37,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        177,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        38..40,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            179,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        182,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 40,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        175,
                                                    ),
                                                    ropd: 41,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                183,
                                                            ),
                                                            ident: `narrow_down`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                185,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                187,
                                                            ),
                                                            ident: `Zero`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                191,
                                                            ),
                                                            ident: `simp_zero_match`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                195,
                                                            ),
                                                            ident: `simp_zero_match`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                199,
                                                            ),
                                                            ident: `simp_zero_match`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 44,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        186,
                                                    ),
                                                    ropd: 45,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        189,
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 46,
                                                    dot_token_idx: TokenIdx(
                                                        192,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            193,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 47,
                                                    dot_token_idx: TokenIdx(
                                                        196,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `rel_norm`,
                                                        token_idx: TokenIdx(
                                                            197,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 48,
                                                    dot_token_idx: TokenIdx(
                                                        200,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `angle_change_norm`,
                                                        token_idx: TokenIdx(
                                                            201,
                                                        ),
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 43,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        184,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        49..54,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            188,
                                                        ),
                                                        TokenIdx(
                                                            190,
                                                        ),
                                                        TokenIdx(
                                                            194,
                                                        ),
                                                        TokenIdx(
                                                            198,
                                                        ),
                                                        TokenIdx(
                                                            202,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        203,
                                                    ),
                                                },
                                                Expr::Suffix {
                                                    opd: 54,
                                                    opr: UnveilOrComposeWithOption,
                                                    opr_token_idx: TokenIdx(
                                                        204,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                206,
                                                            ),
                                                            ident: `simp_zero_match`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 56,
                                                    dot_token_idx: TokenIdx(
                                                        207,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            208,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        210,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 57,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        209,
                                                    ),
                                                    ropd: 58,
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
                                                    owner: 60,
                                                    dot_token_idx: TokenIdx(
                                                        213,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `eff_holes`,
                                                        token_idx: TokenIdx(
                                                            214,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 61,
                                                    dot_token_idx: TokenIdx(
                                                        215,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            216,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        218,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 62,
                                                    lbox_token_idx: TokenIdx(
                                                        217,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        63..64,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        219,
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 64,
                                                    be_token_idx: TokenIdx(
                                                        220,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 2,
                                                        },
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 8,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 66,
                                                    dot_token_idx: TokenIdx(
                                                        224,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `eff_holes`,
                                                        token_idx: TokenIdx(
                                                            225,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 67,
                                                    dot_token_idx: TokenIdx(
                                                        226,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            227,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        229,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 68,
                                                    lbox_token_idx: TokenIdx(
                                                        228,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        69..70,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        230,
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 70,
                                                    be_token_idx: TokenIdx(
                                                        231,
                                                    ),
                                                    target: Ok(
                                                        BeVariableDeclPattern {
                                                            pattern_expr_idx: 3,
                                                        },
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 9,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 72,
                                                    dot_token_idx: TokenIdx(
                                                        236,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `eff_holes`,
                                                        token_idx: TokenIdx(
                                                            237,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 73,
                                                    dot_token_idx: TokenIdx(
                                                        238,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            239,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        241,
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                233,
                                                            ),
                                                            ident: `major_hole`,
                                                        },
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 74,
                                                    lbox_token_idx: TokenIdx(
                                                        240,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        75..76,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        242,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 76,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        234,
                                                    ),
                                                    ropd: 77,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                245,
                                                            ),
                                                            ident: `major_hole`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 79,
                                                    dot_token_idx: TokenIdx(
                                                        246,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `bounding_box`,
                                                        token_idx: TokenIdx(
                                                            247,
                                                        ),
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                253,
                                                            ),
                                                            ident: `major_hole`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 81,
                                                    dot_token_idx: TokenIdx(
                                                        254,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `bounding_box`,
                                                        token_idx: TokenIdx(
                                                            255,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 80,
                                                    dot_token_idx: TokenIdx(
                                                        248,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            249,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        250,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        81..81,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        251,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 82,
                                                    dot_token_idx: TokenIdx(
                                                        256,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `ymin`,
                                                        token_idx: TokenIdx(
                                                            257,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        258,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        83..83,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        259,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                243,
                                                            ),
                                                            ident: `a`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 83,
                                                    opr: PureClosed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        252,
                                                    ),
                                                    ropd: 84,
                                                },
                                                Expr::Binary {
                                                    lopd: 85,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        244,
                                                    ),
                                                    ropd: 86,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 10,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 88,
                                                    dot_token_idx: TokenIdx(
                                                        263,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `bounding_box`,
                                                        token_idx: TokenIdx(
                                                            264,
                                                        ),
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 11,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 90,
                                                    dot_token_idx: TokenIdx(
                                                        271,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `bounding_box`,
                                                        token_idx: TokenIdx(
                                                            272,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 89,
                                                    dot_token_idx: TokenIdx(
                                                        265,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            266,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        267,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        90..90,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        268,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 91,
                                                    dot_token_idx: TokenIdx(
                                                        273,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `ymin`,
                                                        token_idx: TokenIdx(
                                                            274,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        275,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        92..92,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        276,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                260,
                                                            ),
                                                            ident: `b`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 92,
                                                    opr: PureClosed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        269,
                                                    ),
                                                    ropd: 93,
                                                },
                                                Expr::Binary {
                                                    lopd: 94,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        261,
                                                    ),
                                                    ropd: 95,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                279,
                                                            ),
                                                            ident: `a`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                281,
                                                            ),
                                                            ident: `b`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                277,
                                                            ),
                                                            ident: `ratio`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 97,
                                                    opr: PureClosed(
                                                        Div,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        280,
                                                    ),
                                                    ropd: 98,
                                                },
                                                Expr::Binary {
                                                    lopd: 99,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        278,
                                                    ),
                                                    ropd: 100,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                283,
                                                            ),
                                                            ident: `ratio`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Literal(
                                                    TokenIdx(
                                                        285,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 102,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        284,
                                                    ),
                                                    ropd: 103,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                288,
                                                            ),
                                                            ident: `simp_zero_match`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                286,
                                                            ),
                                                            ident: `a`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 105,
                                                    dot_token_idx: TokenIdx(
                                                        289,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            290,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 106,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        287,
                                                    ),
                                                    ropd: 107,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnrecognizedIdentifier {
                                                            token_idx: TokenIdx(
                                                                291,
                                                            ),
                                                            ident: `MnistLabel`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::UnresolvedSubentity {
                                                            token_idx: TokenIdx(
                                                                293,
                                                            ),
                                                            ident: `Zero`,
                                                        },
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 109,
                                                    opr: ScopeResolution,
                                                    opr_token_idx: TokenIdx(
                                                        292,
                                                    ),
                                                    ropd: 110,
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
                                                        115,
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
                                                        127,
                                                    ),
                                                    ident: `open_one_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        135,
                                                    ),
                                                    ident: `open_one_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        144,
                                                    ),
                                                    ident: `connected_components`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        153,
                                                    ),
                                                    ident: `open_one_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        176,
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
                                                        178,
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
                                                        212,
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
                                                        223,
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
                                                        235,
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
                                                        262,
                                                    ),
                                                    ident: `major_line_segment_sketch`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        270,
                                                    ),
                                                    ident: `major_line_segment_sketch`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                Stmt::Eval {
                                                    expr_idx: 10,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            130,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        13,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            134,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        18,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            143,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        22,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 30,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            167,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        33,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 36,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            110,
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
                                                                114,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            6,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonToken(
                                                                TokenIdx(
                                                                    124,
                                                                ),
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
                                                Stmt::Eval {
                                                    expr_idx: 42,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 55,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            205,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        59,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            211,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        65,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            222,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        71,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 78,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 87,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 96,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 101,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            282,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        104,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 108,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 111,
                                                },
                                            ],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                113,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                142,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                221,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                232,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                ],
                                            },
                                            pattern_infos: [
                                                Be,
                                                Be,
                                                Be,
                                                Be,
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 407,
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
                                                                    value: 410,
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
                                                                    value: 407,
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
                                                                    value: 410,
                                                                },
                                                            ),
                                                        ),
                                                        3,
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
                                                ],
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
                                                expr: 112,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    112,
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)