Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Fugitive(
                        FugitiveDefn::Val(
                            ValDefn {
                                path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                decl: ValDecl {
                                    path: FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                    ast_idx: 40,
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
                                                            FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
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
                                                        FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
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
                                                                FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        14,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        16,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 0,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        11,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        2..4,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            13,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        17,
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
                                                            FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
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
                                                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        15,
                                                    ),
                                                    ident: `simple_leftdown_pattern`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
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
                                                kind: BlockExpr,
                                                expr: 5,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
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
                            FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Fugitive(
                        FugitiveDefn::Fn(
                            FnDefn {
                                path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
                                decl: FnDecl {
                                    path: FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
                                    ast_idx: 41,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
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
                                                            23,
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
                                                            27,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            24,
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
                                                            28,
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
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    21,
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
                                                                22,
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
                                                20,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        regular_parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        22,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                25,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                26,
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
                                                29,
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
                                                                    FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
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
                                                                    23,
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
                                                                    27,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    24,
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
                                                                    28,
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
                                                                    modifier_keyword_group: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `cc`,
                                                                        token_idx: TokenIdx(
                                                                            21,
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
                                                                        22,
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
                                                        FugitivePath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        33,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        34,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            35,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        36,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        37,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        39,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        40,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            41,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        43,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 3,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        42,
                                                    ),
                                                    ropd: 4,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        45,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 6,
                                                    dot_token_idx: TokenIdx(
                                                        46,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            47,
                                                        ),
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        44,
                                                    ),
                                                    opd: 7,
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
                                                            30,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
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
                                                                32,
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
                                                            38,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        5,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 8,
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
                                                                31,
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
                                                            32,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    48,
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
                                                expr: 9,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    9,
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
                            FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Fugitive(
                        FugitiveDefn::Val(
                            ValDefn {
                                path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                decl: ValDecl {
                                    path: FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                    ast_idx: 42,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                50,
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
                                            52,
                                        ),
                                    ),
                                    expr_or_eol_token: Left(
                                        EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    53,
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
                                                            FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
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
                                                            51,
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
                                                        FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
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
                                                                FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        58,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        1..3,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        62,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 0,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        55,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        3..5,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            57,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        63,
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
                                                        54,
                                                    ),
                                                    ident: `fermi_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        56,
                                                    ),
                                                    ident: `major_concave_components`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::major::major_concave_components`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        59,
                                                    ),
                                                    ident: `leftupcc_pattern`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        61,
                                                    ),
                                                    ident: `leftdowncc_pattern`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
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
                                                kind: BlockExpr,
                                                expr: 6,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    6,
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
                            FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Fugitive(
                        FugitiveDefn::Fn(
                            FnDefn {
                                path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
                                decl: FnDecl {
                                    path: FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
                                    ast_idx: 43,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
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
                                                            69,
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
                                                            73,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            70,
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
                                                            74,
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
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    67,
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
                                                                68,
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
                                                66,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        regular_parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        68,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                71,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                72,
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
                                                75,
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
                                                                    FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
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
                                                                    69,
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
                                                                    73,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    70,
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
                                                                    74,
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
                                                                    modifier_keyword_group: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `cc`,
                                                                        token_idx: TokenIdx(
                                                                            67,
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
                                                                        68,
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
                                                        FugitivePath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        79,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        80,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            81,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        82,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        83,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        85,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        86,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            87,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        89,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 3,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        88,
                                                    ),
                                                    ropd: 4,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        91,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 6,
                                                    dot_token_idx: TokenIdx(
                                                        92,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_bounding_box`,
                                                        token_idx: TokenIdx(
                                                            93,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 7,
                                                    dot_token_idx: TokenIdx(
                                                        94,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            95,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        96,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        8..8,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        97,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        99,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 8,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        98,
                                                    ),
                                                    ropd: 9,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        100,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 11,
                                                    dot_token_idx: TokenIdx(
                                                        101,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `end`,
                                                        token_idx: TokenIdx(
                                                            102,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        103,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        12..12,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        104,
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 12,
                                                    dot_token_idx: TokenIdx(
                                                        105,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            106,
                                                        ),
                                                    },
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
                                                            76,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
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
                                                                78,
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
                                                            84,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        5,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            90,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        10,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 13,
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
                                                                77,
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
                                                            78,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    107,
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
                                                expr: 14,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    14,
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
                            FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Fugitive(
                        FugitiveDefn::Fn(
                            FnDefn {
                                path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
                                decl: FnDecl {
                                    path: FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
                                    ast_idx: 44,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
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
                                                            112,
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
                                                            116,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            113,
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
                                                            117,
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
                                                            modifier_keyword_group: None,
                                                            ident_token: IdentToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    110,
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
                                                                111,
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
                                                109,
                                            ),
                                        ),
                                        self_parameter: None,
                                        comma_after_self_parameter: None,
                                        regular_parameters: [
                                            RegularParameterDeclPattern {
                                                pattern: 0,
                                                variables: ArenaIdxRange(
                                                    0..1,
                                                ),
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        111,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                        ],
                                        commas: [],
                                        rpar: RightParenthesisToken(
                                            TokenIdx(
                                                114,
                                            ),
                                        ),
                                    },
                                    curry_token: Some(
                                        CurryToken(
                                            TokenIdx(
                                                115,
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
                                                118,
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
                                                                    FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
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
                                                                    112,
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
                                                                    116,
                                                                ),
                                                                opd: 2,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    113,
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
                                                                    117,
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
                                                                    modifier_keyword_group: None,
                                                                    ident_token: IdentToken {
                                                                        ident: `cc`,
                                                                        token_idx: TokenIdx(
                                                                            110,
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
                                                                        111,
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
                                                        FugitivePath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Fn`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        122,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 0,
                                                    dot_token_idx: TokenIdx(
                                                        123,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            124,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        125,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        126,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        128,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 2,
                                                    dot_token_idx: TokenIdx(
                                                        129,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
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
                                                    lopd: 3,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        131,
                                                    ),
                                                    ropd: 4,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        134,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 6,
                                                    dot_token_idx: TokenIdx(
                                                        135,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `relative_bounding_box`,
                                                        token_idx: TokenIdx(
                                                            136,
                                                        ),
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 7,
                                                    dot_token_idx: TokenIdx(
                                                        137,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `ymin`,
                                                        token_idx: TokenIdx(
                                                            138,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        139,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        8..8,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        140,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        142,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 8,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        141,
                                                    ),
                                                    ropd: 9,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `cc`,
                                                    token_idx: TokenIdx(
                                                        146,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::ExplicitParameter {
                                                        ident: `cc`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 11,
                                                    dot_token_idx: TokenIdx(
                                                        147,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `start_tangent`,
                                                        token_idx: TokenIdx(
                                                            148,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        149,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        12..12,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        150,
                                                    ),
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        154,
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 12,
                                                    dot_token_idx: TokenIdx(
                                                        151,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `angle`,
                                                        token_idx: TokenIdx(
                                                            152,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        153,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        13..14,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        155,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `ang`,
                                                    token_idx: TokenIdx(
                                                        157,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        159,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 15,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        158,
                                                    ),
                                                    ropd: 16,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `ang`,
                                                    token_idx: TokenIdx(
                                                        160,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        0..6,
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
                                                            119,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
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
                                                                121,
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
                                                            127,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        5,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            133,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        10,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            143,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
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
                                                                145,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        14,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            156,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        17,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 18,
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
                                                                120,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `ang`,
                                                            token_idx: TokenIdx(
                                                                144,
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
                                                        `ang`,
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
                                                            121,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    161,
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
                                                            145,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    161,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `ang`,
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
                                                kind: BlockExpr,
                                                expr: 19,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    19,
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
                            FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Fugitive(
                        FugitiveDefn::Val(
                            ValDefn {
                                path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                decl: ValDecl {
                                    path: FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
                                    ast_idx: 45,
                                    colon_token: Some(
                                        ColonToken(
                                            TokenIdx(
                                                167,
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
                                            170,
                                        ),
                                    ),
                                    expr_or_eol_token: Left(
                                        EolToken::Colon(
                                            EolColonToken {
                                                token_idx: TokenIdx(
                                                    171,
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
                                                            FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
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
                                                            168,
                                                        ),
                                                        opd: 0,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            169,
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
                                                        FugitivePath(`mnist_classifier::digits::seven::is_seven`, `Val`),
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
                                                                FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 0,
                                                    be_token_idx: TokenIdx(
                                                        174,
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
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 2,
                                                    be_token_idx: TokenIdx(
                                                        178,
                                                    ),
                                                    target: Ok(
                                                        BeVariablesPattern {
                                                            pattern_expr: 1,
                                                            variables: ArenaIdxRange(
                                                                1..2,
                                                            ),
                                                        },
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 4,
                                                    be_token_idx: TokenIdx(
                                                        182,
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
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 6,
                                                    dot_token_idx: TokenIdx(
                                                        186,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `max_hole_ilen`,
                                                        token_idx: TokenIdx(
                                                            187,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        189,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 7,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        188,
                                                    ),
                                                    ropd: 8,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 4,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 10,
                                                    dot_token_idx: TokenIdx(
                                                        194,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `norm`,
                                                        token_idx: TokenIdx(
                                                            195,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `simple_match_norm`,
                                                    token_idx: TokenIdx(
                                                        197,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        199,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 12,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        198,
                                                    ),
                                                    ropd: 13,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 5,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 15,
                                                    dot_token_idx: TokenIdx(
                                                        203,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            204,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        206,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 16,
                                                    lbox_token_idx: TokenIdx(
                                                        205,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        17..18,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        207,
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 18,
                                                    be_token_idx: TokenIdx(
                                                        208,
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
                                                Expr::EntityPath {
                                                    entity_path_expr: 6,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 7,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 20,
                                                    dot_token_idx: TokenIdx(
                                                        214,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `upper_mass`,
                                                        token_idx: TokenIdx(
                                                            215,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 21,
                                                    dot_token_idx: TokenIdx(
                                                        218,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `lower_mass`,
                                                        token_idx: TokenIdx(
                                                            219,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 22,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        216,
                                                    ),
                                                    ropd: 23,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upper_excess`,
                                                    token_idx: TokenIdx(
                                                        221,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        223,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 25,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        222,
                                                    ),
                                                    ropd: 26,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 8,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 28,
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
                                                    owner: 29,
                                                    lbox_token_idx: TokenIdx(
                                                        231,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        30..31,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        233,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 31,
                                                    dot_token_idx: TokenIdx(
                                                        234,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `end_tangent`,
                                                        token_idx: TokenIdx(
                                                            235,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        236,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        32..32,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        237,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `end_tangent`,
                                                    token_idx: TokenIdx(
                                                        241,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 33,
                                                    dot_token_idx: TokenIdx(
                                                        242,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `y`,
                                                        token_idx: TokenIdx(
                                                            243,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        248,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        245,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        247,
                                                    ),
                                                    opd: 35,
                                                },
                                                Expr::Binary {
                                                    lopd: 36,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        246,
                                                    ),
                                                    ropd: 37,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 10,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Seven`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `simple_match_norm`,
                                                    token_idx: TokenIdx(
                                                        254,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        256,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 40,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        255,
                                                    ),
                                                    ropd: 41,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 11,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 12,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 43,
                                                    dot_token_idx: TokenIdx(
                                                        262,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `upper_mass`,
                                                        token_idx: TokenIdx(
                                                            263,
                                                        ),
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 44,
                                                    dot_token_idx: TokenIdx(
                                                        266,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `lower_mass`,
                                                        token_idx: TokenIdx(
                                                            267,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 45,
                                                    opr: Closed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        264,
                                                    ),
                                                    ropd: 46,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `upper_excess`,
                                                    token_idx: TokenIdx(
                                                        269,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        271,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 48,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        270,
                                                    ),
                                                    ropd: 49,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 14,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Seven`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 15,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 52,
                                                    dot_token_idx: TokenIdx(
                                                        278,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `matches`,
                                                        token_idx: TokenIdx(
                                                            279,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        281,
                                                    ),
                                                ),
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 53,
                                                    lbox_token_idx: TokenIdx(
                                                        280,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        54..55,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        282,
                                                    ),
                                                },
                                                Expr::Be {
                                                    src: 55,
                                                    be_token_idx: TokenIdx(
                                                        283,
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
                                                Expr::EntityPath {
                                                    entity_path_expr: 16,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::Field {
                                                    owner: 57,
                                                    dot_token_idx: TokenIdx(
                                                        289,
                                                    ),
                                                    ident_token: IdentToken {
                                                        ident: `others`,
                                                        token_idx: TokenIdx(
                                                            290,
                                                        ),
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        292,
                                                    ),
                                                ),
                                                Expr::EntityPath {
                                                    entity_path_expr: 18,
                                                    path: Some(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Seven`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        10..21,
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
                                                            FugitivePath(`mnist_classifier::digits::one::is_one`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        177,
                                                    ),
                                                    ident: `is_six`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::six::is_six`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        181,
                                                    ),
                                                    ident: `is_zero`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        185,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        193,
                                                    ),
                                                    ident: `simple_seven_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        202,
                                                    ),
                                                    ident: `simple_seven_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
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
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
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
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        228,
                                                    ),
                                                    ident: `simple_seven_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::seven::simple_seven_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        250,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 9,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            251,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Seven`,
                                                            token_idx: TokenIdx(
                                                                252,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Seven`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        261,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        265,
                                                    ),
                                                    ident: `major_connected_component`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::major::major_connected_component`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        273,
                                                    ),
                                                    ident: `MnistLabel`,
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
                                                            274,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Seven`,
                                                            token_idx: TokenIdx(
                                                                275,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Seven`,
                                                            },
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        277,
                                                    ),
                                                    ident: `special_seven_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        288,
                                                    ),
                                                    ident: `special_seven_match`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FugitivePath(`mnist_classifier::digits::seven::special_seven_match`, `Val`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        293,
                                                    ),
                                                    ident: `MnistLabel`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist::MnistLabel`, `Enum`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Subentity {
                                                    parent: 17,
                                                    scope_resolution_token: ScopeResolutionToken(
                                                        TokenIdx(
                                                            294,
                                                        ),
                                                    ),
                                                    ident_token: Ok(
                                                        IdentToken {
                                                            ident: `Seven`,
                                                            token_idx: TokenIdx(
                                                                295,
                                                            ),
                                                        },
                                                    ),
                                                    path: Ok(
                                                        EntityPath::TypeVariant(
                                                            TypeVariantPath {
                                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                                ident: `Seven`,
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
                                                            225,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
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
                                                                227,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        32,
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
                                                                240,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        34,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            244,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        38,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            201,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        19,
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
                                                                212,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        24,
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                220,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            27,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        224,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                0..3,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                Stmt::Return {
                                                    return_token: ReturnToken {
                                                        token_idx: TokenIdx(
                                                            249,
                                                        ),
                                                    },
                                                    result: Ok(
                                                        39,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            258,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
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
                                                                260,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        47,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            268,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        50,
                                                    ),
                                                },
                                                Stmt::Return {
                                                    return_token: ReturnToken {
                                                        token_idx: TokenIdx(
                                                            272,
                                                        ),
                                                    },
                                                    result: Ok(
                                                        51,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            172,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        1,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            176,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        3,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            180,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        5,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            184,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        9,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            190,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
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
                                                                192,
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
                                                                196,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            14,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        200,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                3..7,
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
                                                                253,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            42,
                                                        ),
                                                        eol_colon: Ok(
                                                            Colon(
                                                                EolColonToken {
                                                                    token_idx: TokenIdx(
                                                                        257,
                                                                    ),
                                                                },
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                7..10,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            276,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        56,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            285,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
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
                                                                287,
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        58,
                                                    ),
                                                },
                                                Stmt::Require {
                                                    require_token: RequireToken {
                                                        token_idx: TokenIdx(
                                                            291,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        59,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 60,
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
                                                                175,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                179,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `none`,
                                                            token_idx: TokenIdx(
                                                                183,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `simple_match_norm`,
                                                            token_idx: TokenIdx(
                                                                191,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                209,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `upper_excess`,
                                                            token_idx: TokenIdx(
                                                                211,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `end_tangent`,
                                                            token_idx: TokenIdx(
                                                                226,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `a`,
                                                            token_idx: TokenIdx(
                                                                239,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `upper_excess`,
                                                            token_idx: TokenIdx(
                                                                259,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `some`,
                                                            token_idx: TokenIdx(
                                                                284,
                                                            ),
                                                        },
                                                    },
                                                    PatternExpr::Ident {
                                                        modifier_keyword_group: None,
                                                        ident_token: IdentToken {
                                                            ident: `others`,
                                                            token_idx: TokenIdx(
                                                                286,
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
                                                        `simple_match_norm`,
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
                                                        `upper_excess`,
                                                        5,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `end_tangent`,
                                                        6,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `a`,
                                                        7,
                                                    ),
                                                ],
                                                [
                                                    (
                                                        `upper_excess`,
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
                                                        `others`,
                                                        10,
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
                                                            176,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    296,
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
                                                            180,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    296,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `none`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            184,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    296,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `none`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            192,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    296,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `simple_match_norm`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            210,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    253,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            212,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    253,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `upper_excess`,
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            227,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    249,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `end_tangent`,
                                                            pattern_symbol_idx: 6,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            240,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    249,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `a`,
                                                            pattern_symbol_idx: 7,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            260,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    276,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `upper_excess`,
                                                            pattern_symbol_idx: 8,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            285,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    296,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `some`,
                                                            pattern_symbol_idx: 9,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        modifier: Pure,
                                                        access_start: TokenIdx(
                                                            287,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    296,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `others`,
                                                            pattern_symbol_idx: 10,
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
                                                expr: 61,
                                            },
                                        ],
                                    },
                                },
                                body: Some(
                                    61,
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)