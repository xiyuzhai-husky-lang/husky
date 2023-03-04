Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                                decl: FunctionDecl {
                                    path: FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                                    ast_idx: 22,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
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
                                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::num::i32`, `Extern`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`core::basic::bool`, `Extern`),
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
                                                            24,
                                                        ),
                                                        ident: `LineSegmentSketch`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            28,
                                                        ),
                                                        ident: `i32`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::num::i32`, `Extern`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            31,
                                                        ),
                                                        ident: `bool`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`core::basic::bool`, `Extern`),
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
                                                                ident: `line_segment_sketch`,
                                                                token_idx: TokenIdx(
                                                                    22,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `index`,
                                                                token_idx: TokenIdx(
                                                                    26,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                    ],
                                                },
                                                pattern_infos: [
                                                    Parameter,
                                                    Parameter,
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 141,
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
                                                                        value: 378,
                                                                    },
                                                                ),
                                                            ),
                                                            1,
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
                                                                23,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                ident: `line_segment_sketch`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                27,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                ident: `index`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 0,
                                                    },
                                                    RegularParameter {
                                                        pattern: 1,
                                                        ty: 1,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: ReturnType,
                                                    expr: 2,
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
                                                    21,
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
                                                            23,
                                                        ),
                                                    ),
                                                    ty: 0,
                                                },
                                                ExplicitParameterDeclPattern {
                                                    pattern: 1,
                                                    variables: ArenaIdxRange(
                                                        1..2,
                                                    ),
                                                    colon: ColonToken(
                                                        TokenIdx(
                                                            27,
                                                        ),
                                                    ),
                                                    ty: 1,
                                                },
                                            ],
                                            commas: [
                                                CommaToken(
                                                    TokenIdx(
                                                        25,
                                                    ),
                                                ),
                                            ],
                                            decl_list_result: Ok(
                                                (),
                                            ),
                                            rpar: Ok(
                                                RightParenthesisToken(
                                                    TokenIdx(
                                                        29,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                30,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 2,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                32,
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
                                                                    FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
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
                                                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 1,
                                                                path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::num::i32`, `Extern`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 2,
                                                                path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`core::basic::bool`, `Extern`),
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
                                                                    24,
                                                                ),
                                                                ident: `LineSegmentSketch`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    28,
                                                                ),
                                                                ident: `i32`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::num::i32`, `Extern`),
                                                                    ),
                                                                ),
                                                            },
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    31,
                                                                ),
                                                                ident: `bool`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`core::basic::bool`, `Extern`),
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
                                                                        ident: `line_segment_sketch`,
                                                                        token_idx: TokenIdx(
                                                                            22,
                                                                        ),
                                                                    },
                                                                    liason: None,
                                                                },
                                                                PatternExpr::Identifier {
                                                                    ident_token: IdentifierToken {
                                                                        ident: `index`,
                                                                        token_idx: TokenIdx(
                                                                            26,
                                                                        ),
                                                                    },
                                                                    liason: None,
                                                                },
                                                            ],
                                                        },
                                                        pattern_infos: [
                                                            Parameter,
                                                            Parameter,
                                                        ],
                                                        pattern_symbol_maps: [
                                                            [
                                                                (
                                                                    Identifier(
                                                                        Word(
                                                                            Id {
                                                                                value: 141,
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
                                                                                value: 378,
                                                                            },
                                                                        ),
                                                                    ),
                                                                    1,
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
                                                                        23,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                                        ident: `line_segment_sketch`,
                                                                        pattern_symbol_idx: 0,
                                                                    },
                                                                },
                                                                CurrentSymbol {
                                                                    access_start: TokenIdx(
                                                                        27,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                                        ident: `index`,
                                                                        pattern_symbol_idx: 1,
                                                                    },
                                                                },
                                                            ],
                                                        },
                                                        allow_self_type: False,
                                                        allow_self_value: False,
                                                        pattern_ty_constraints: [
                                                            RegularParameter {
                                                                pattern: 0,
                                                                ty: 0,
                                                            },
                                                            RegularParameter {
                                                                pattern: 1,
                                                                ty: 1,
                                                            },
                                                        ],
                                                    },
                                                    roots: [
                                                        ExprRoot {
                                                            kind: ReturnType,
                                                            expr: 2,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                                                    ),
                                                ),
                                            ),
                                        ),
                                        expr_arena: Arena {
                                            data: [
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        36,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 0,
                                                    dot_token_idx: TokenIdx(
                                                        37,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            38,
                                                        ),
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                35,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 1,
                                                    dot_token_idx: TokenIdx(
                                                        39,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `ilen`,
                                                        token_idx: TokenIdx(
                                                            40,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        41,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        2..2,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        42,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 2,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        35,
                                                    ),
                                                    ropd: 3,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        46,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 5,
                                                    dot_token_idx: TokenIdx(
                                                        47,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            48,
                                                        ),
                                                    },
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `index`,
                                                    token_idx: TokenIdx(
                                                        50,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `index`,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        52,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 7,
                                                    opr: PureClosed(
                                                        RemEuclid,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        51,
                                                    ),
                                                    ropd: 8,
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 6,
                                                    lbox_token_idx: TokenIdx(
                                                        49,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        9..10,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        53,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                45,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 10,
                                                    dot_token_idx: TokenIdx(
                                                        54,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            55,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        56,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        11..11,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        57,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 11,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        45,
                                                    ),
                                                    ropd: 12,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        61,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 14,
                                                    dot_token_idx: TokenIdx(
                                                        62,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            63,
                                                        ),
                                                    },
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `index`,
                                                    token_idx: TokenIdx(
                                                        66,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `index`,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        68,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 16,
                                                    opr: PureClosed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        67,
                                                    ),
                                                    ropd: 17,
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        65,
                                                    ),
                                                    item: 18,
                                                    rpar_token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        71,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 19,
                                                    opr: PureClosed(
                                                        RemEuclid,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        70,
                                                    ),
                                                    ropd: 20,
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 15,
                                                    lbox_token_idx: TokenIdx(
                                                        64,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        21..22,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        72,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                60,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 22,
                                                    dot_token_idx: TokenIdx(
                                                        73,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            74,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        75,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        23..23,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        76,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 23,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        60,
                                                    ),
                                                    ropd: 24,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `previous_displacement`,
                                                    token_idx: TokenIdx(
                                                        80,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `current_displacement`,
                                                    token_idx: TokenIdx(
                                                        84,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                79,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 26,
                                                    dot_token_idx: TokenIdx(
                                                        81,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `rotation_direction_to`,
                                                        token_idx: TokenIdx(
                                                            82,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        83,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        27..28,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        85,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 28,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        79,
                                                    ),
                                                    ropd: 29,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `is_rotation_counterclockwise_result`,
                                                    token_idx: TokenIdx(
                                                        87,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        89,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 31,
                                                    opr: Comparison(
                                                        Eq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        88,
                                                    ),
                                                    ropd: 32,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        96,
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                94,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        95,
                                                    ),
                                                    opd: 34,
                                                },
                                                Expr::Binary {
                                                    lopd: 35,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        94,
                                                    ),
                                                    ropd: 36,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        100,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 38,
                                                    dot_token_idx: TokenIdx(
                                                        101,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            102,
                                                        ),
                                                    },
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `index`,
                                                    token_idx: TokenIdx(
                                                        105,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `index`,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        107,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 40,
                                                    opr: PureClosed(
                                                        Sub,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        106,
                                                    ),
                                                    ropd: 41,
                                                },
                                                Expr::Bracketed {
                                                    lpar_token_idx: TokenIdx(
                                                        104,
                                                    ),
                                                    item: 42,
                                                    rpar_token_idx: TokenIdx(
                                                        108,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        110,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 43,
                                                    opr: PureClosed(
                                                        RemEuclid,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        109,
                                                    ),
                                                    ropd: 44,
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 39,
                                                    lbox_token_idx: TokenIdx(
                                                        103,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        45..46,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        111,
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
                                                Expr::Field {
                                                    owner: 46,
                                                    dot_token_idx: TokenIdx(
                                                        112,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `points`,
                                                        token_idx: TokenIdx(
                                                            113,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 47,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        99,
                                                    ),
                                                    ropd: 48,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `previous_interval`,
                                                    token_idx: TokenIdx(
                                                        115,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 50,
                                                    dot_token_idx: TokenIdx(
                                                        116,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `start`,
                                                        token_idx: TokenIdx(
                                                            117,
                                                        ),
                                                    },
                                                },
                                                Expr::FrameVarDecl {
                                                    token_idx: TokenIdx(
                                                        119,
                                                    ),
                                                    ident: `i1`,
                                                    frame_var_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        52,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `previous_interval`,
                                                    token_idx: TokenIdx(
                                                        121,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 51,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        118,
                                                    ),
                                                    ropd: 52,
                                                },
                                                Expr::Field {
                                                    owner: 53,
                                                    dot_token_idx: TokenIdx(
                                                        122,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `end`,
                                                        token_idx: TokenIdx(
                                                            123,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 54,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        120,
                                                    ),
                                                    ropd: 55,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        128,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 57,
                                                    dot_token_idx: TokenIdx(
                                                        129,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `contour`,
                                                        token_idx: TokenIdx(
                                                            130,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `previous_interval`,
                                                    token_idx: TokenIdx(
                                                        134,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 59,
                                                    dot_token_idx: TokenIdx(
                                                        135,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `start`,
                                                        token_idx: TokenIdx(
                                                            136,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i1`,
                                                    token_idx: TokenIdx(
                                                        138,
                                                    ),
                                                    current_symbol_idx: 6,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        52,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                127,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 58,
                                                    dot_token_idx: TokenIdx(
                                                        131,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            132,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        133,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        60..62,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        139,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 62,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        127,
                                                    ),
                                                    ropd: 63,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `previous_raw_cross`,
                                                    token_idx: TokenIdx(
                                                        142,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `current_displacement`,
                                                    token_idx: TokenIdx(
                                                        146,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `displacement`,
                                                    token_idx: TokenIdx(
                                                        150,
                                                    ),
                                                    current_symbol_idx: 7,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 6,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 66,
                                                    dot_token_idx: TokenIdx(
                                                        147,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `cross`,
                                                        token_idx: TokenIdx(
                                                            148,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        149,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        67..68,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        151,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `previous_raw_cross`,
                                                    token_idx: TokenIdx(
                                                        140,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 65,
                                                    dot_token_idx: TokenIdx(
                                                        143,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `max`,
                                                        token_idx: TokenIdx(
                                                            144,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        145,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        68..69,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        152,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 69,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        141,
                                                    ),
                                                    ropd: 70,
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        158,
                                                    ),
                                                ),
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                156,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        157,
                                                    ),
                                                    opd: 72,
                                                },
                                                Expr::Binary {
                                                    lopd: 73,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        156,
                                                    ),
                                                    ropd: 74,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        162,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 76,
                                                    dot_token_idx: TokenIdx(
                                                        163,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            164,
                                                        ),
                                                    },
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `index`,
                                                    token_idx: TokenIdx(
                                                        166,
                                                    ),
                                                    inherited_symbol_idx: 1,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `index`,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        168,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 78,
                                                    opr: PureClosed(
                                                        RemEuclid,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        167,
                                                    ),
                                                    ropd: 79,
                                                },
                                                Expr::IndexOrCompositionWithList {
                                                    owner: 77,
                                                    lbox_token_idx: TokenIdx(
                                                        165,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        80..81,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        169,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                161,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::Field {
                                                    owner: 81,
                                                    dot_token_idx: TokenIdx(
                                                        170,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `points`,
                                                        token_idx: TokenIdx(
                                                            171,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 82,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        161,
                                                    ),
                                                    ropd: 83,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `current_interval`,
                                                    token_idx: TokenIdx(
                                                        173,
                                                    ),
                                                    current_symbol_idx: 9,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 85,
                                                    dot_token_idx: TokenIdx(
                                                        174,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `start`,
                                                        token_idx: TokenIdx(
                                                            175,
                                                        ),
                                                    },
                                                },
                                                Expr::FrameVarDecl {
                                                    token_idx: TokenIdx(
                                                        177,
                                                    ),
                                                    ident: `i2`,
                                                    frame_var_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        87,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `current_interval`,
                                                    token_idx: TokenIdx(
                                                        179,
                                                    ),
                                                    current_symbol_idx: 9,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 8,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 86,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        176,
                                                    ),
                                                    ropd: 87,
                                                },
                                                Expr::Field {
                                                    owner: 88,
                                                    dot_token_idx: TokenIdx(
                                                        180,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `end`,
                                                        token_idx: TokenIdx(
                                                            181,
                                                        ),
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 89,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        178,
                                                    ),
                                                    ropd: 90,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        186,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 92,
                                                    dot_token_idx: TokenIdx(
                                                        187,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `contour`,
                                                        token_idx: TokenIdx(
                                                            188,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `previous_interval`,
                                                    token_idx: TokenIdx(
                                                        192,
                                                    ),
                                                    current_symbol_idx: 5,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 5,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 94,
                                                    dot_token_idx: TokenIdx(
                                                        193,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `start`,
                                                        token_idx: TokenIdx(
                                                            194,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `i2`,
                                                    token_idx: TokenIdx(
                                                        196,
                                                    ),
                                                    current_symbol_idx: 10,
                                                    current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                        87,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                185,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 93,
                                                    dot_token_idx: TokenIdx(
                                                        189,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `displacement`,
                                                        token_idx: TokenIdx(
                                                            190,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        191,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        95..97,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        197,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 97,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        185,
                                                    ),
                                                    ropd: 98,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `current_raw_cross`,
                                                    token_idx: TokenIdx(
                                                        200,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `current_displacement`,
                                                    token_idx: TokenIdx(
                                                        204,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `displacement`,
                                                    token_idx: TokenIdx(
                                                        208,
                                                    ),
                                                    current_symbol_idx: 11,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 9,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 101,
                                                    dot_token_idx: TokenIdx(
                                                        205,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `cross`,
                                                        token_idx: TokenIdx(
                                                            206,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        207,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        102..103,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        209,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `current_raw_cross`,
                                                    token_idx: TokenIdx(
                                                        198,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 100,
                                                    dot_token_idx: TokenIdx(
                                                        201,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `max`,
                                                        token_idx: TokenIdx(
                                                            202,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        203,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        103..104,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        210,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 104,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        199,
                                                    ),
                                                    ropd: 105,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `current_raw_cross`,
                                                    token_idx: TokenIdx(
                                                        212,
                                                    ),
                                                    current_symbol_idx: 8,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 7,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `previous_raw_cross`,
                                                    token_idx: TokenIdx(
                                                        214,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 107,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        213,
                                                    ),
                                                    ropd: 108,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `is_rotation_counterclockwise_result`,
                                                    token_idx: TokenIdx(
                                                        218,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        220,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 110,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        219,
                                                    ),
                                                    ropd: 111,
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        12..17,
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
                                                            125,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 6,
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
                                                                    127,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        64,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 71,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            183,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 9,
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
                                                                    185,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        99,
                                                    ),
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 106,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            91,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 4,
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
                                                                    94,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        37,
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
                                                            pattern: 5,
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
                                                                    99,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        49,
                                                    ),
                                                },
                                                Stmt::ForBetween {
                                                    for_token: ForToken {
                                                        token_idx: TokenIdx(
                                                            114,
                                                        ),
                                                    },
                                                    particulars: ForBetweenParticulars {
                                                        frame_var_token_idx: TokenIdx(
                                                            119,
                                                        ),
                                                        frame_var_expr_idx: 52,
                                                        frame_var_ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 201,
                                                                },
                                                            ),
                                                        ),
                                                        range: ForBetweenRange {
                                                            initial_boundary: LoopBoundary {
                                                                bound_expr: Some(
                                                                    51,
                                                                ),
                                                                kind: LowerClosed,
                                                            },
                                                            final_boundary: LoopBoundary {
                                                                bound_expr: Some(
                                                                    55,
                                                                ),
                                                                kind: UpperOpen,
                                                            },
                                                            step: Constant(
                                                                1,
                                                            ),
                                                        },
                                                    },
                                                    frame_var_symbol_idx: 6,
                                                    eol_colon: Ok(
                                                        EolColonToken(
                                                            TokenIdx(
                                                                124,
                                                            ),
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            0..2,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            153,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 7,
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
                                                                    156,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        75,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            159,
                                                        ),
                                                    },
                                                    let_variable_pattern: Ok(
                                                        LetVariablesPattern {
                                                            pattern: 8,
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
                                                                    161,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        84,
                                                    ),
                                                },
                                                Stmt::ForBetween {
                                                    for_token: ForToken {
                                                        token_idx: TokenIdx(
                                                            172,
                                                        ),
                                                    },
                                                    particulars: ForBetweenParticulars {
                                                        frame_var_token_idx: TokenIdx(
                                                            177,
                                                        ),
                                                        frame_var_expr_idx: 87,
                                                        frame_var_ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 203,
                                                                },
                                                            ),
                                                        ),
                                                        range: ForBetweenRange {
                                                            initial_boundary: LoopBoundary {
                                                                bound_expr: Some(
                                                                    86,
                                                                ),
                                                                kind: LowerClosed,
                                                            },
                                                            final_boundary: LoopBoundary {
                                                                bound_expr: Some(
                                                                    90,
                                                                ),
                                                                kind: UpperOpen,
                                                            },
                                                            step: Constant(
                                                                1,
                                                            ),
                                                        },
                                                    },
                                                    frame_var_symbol_idx: 10,
                                                    eol_colon: Ok(
                                                        EolColonToken(
                                                            TokenIdx(
                                                                182,
                                                            ),
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            2..4,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Return {
                                                    return_token: ReturnToken {
                                                        token_idx: TokenIdx(
                                                            211,
                                                        ),
                                                    },
                                                    result: Ok(
                                                        109,
                                                    ),
                                                },
                                                Stmt::Return {
                                                    return_token: ReturnToken {
                                                        token_idx: TokenIdx(
                                                            217,
                                                        ),
                                                    },
                                                    result: Ok(
                                                        112,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            33,
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
                                                                    35,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        4,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            43,
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
                                                                    45,
                                                                ),
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
                                                            58,
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
                                                                    60,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        25,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            77,
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
                                                                    79,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        30,
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                86,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            33,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonToken(
                                                                TokenIdx(
                                                                    90,
                                                                ),
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                4..11,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: Some(
                                                        ElseBranch {
                                                            else_token: ElseToken {
                                                                token_idx: TokenIdx(
                                                                    215,
                                                                ),
                                                            },
                                                            eol_colon: Ok(
                                                                EolColonToken(
                                                                    TokenIdx(
                                                                        216,
                                                                    ),
                                                                ),
                                                            ),
                                                            block: Ok(
                                                                ArenaIdxRange(
                                                                    11..12,
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
                                                            ident: `L`,
                                                            token_idx: TokenIdx(
                                                                34,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `current_displacement`,
                                                            token_idx: TokenIdx(
                                                                44,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `previous_displacement`,
                                                            token_idx: TokenIdx(
                                                                59,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `is_rotation_counterclockwise_result`,
                                                            token_idx: TokenIdx(
                                                                78,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `previous_raw_cross`,
                                                            token_idx: TokenIdx(
                                                                93,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `previous_interval`,
                                                            token_idx: TokenIdx(
                                                                98,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                126,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `current_raw_cross`,
                                                            token_idx: TokenIdx(
                                                                155,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `current_interval`,
                                                            token_idx: TokenIdx(
                                                                160,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                184,
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
                                                                    value: 337,
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
                                                                    value: 379,
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
                                                                    value: 380,
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
                                                                    value: 381,
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
                                                                    value: 382,
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
                                                                    value: 383,
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
                                                                    value: 240,
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
                                                                    value: 384,
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
                                                                    value: 385,
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
                                                                    value: 240,
                                                                },
                                                            ),
                                                        ),
                                                        9,
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
                                                            ident: `line_segment_sketch`,
                                                        },
                                                    },
                                                    InheritedSymbol {
                                                        parent_symbol_idx: Current(
                                                            1,
                                                        ),
                                                        kind: InheritedSymbolKind::RegularParameter {
                                                            ident: `index`,
                                                        },
                                                    },
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            35,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    221,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `L`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            45,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    221,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `current_displacement`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            60,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    221,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `previous_displacement`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            79,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    221,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `is_rotation_counterclockwise_result`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            94,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    215,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `previous_raw_cross`,
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            99,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    215,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `previous_interval`,
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            125,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    153,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::FrameVariable {
                                                            ident: `i1`,
                                                            expr_idx: 52,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            127,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    153,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `displacement`,
                                                            pattern_symbol_idx: 6,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            156,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    215,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `current_raw_cross`,
                                                            pattern_symbol_idx: 7,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            161,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    215,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `current_interval`,
                                                            pattern_symbol_idx: 8,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            183,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    211,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::FrameVariable {
                                                            ident: `i2`,
                                                            expr_idx: 87,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            185,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    211,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `displacement`,
                                                            pattern_symbol_idx: 9,
                                                        },
                                                    },
                                                ],
                                            },
                                            allow_self_type: False,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [
                                                FrameVariable,
                                                FrameVariable,
                                            ],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: BlockExpr,
                                                expr: 113,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    113,
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)