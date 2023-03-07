Ok(
    DefnSheet {
        defns: [
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Type(
                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Type(
                        TypeDefn::RegularStruct(
                            RegularStructTypeDefn {
                                path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                decl: RegularStructTypeDecl {
                                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    ast_idx: 74,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                    Expr::Prefix {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            38,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    44,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoRightOperandForBinaryOperator {
                                                                lopd: 2,
                                                                punctuation: PureClosed(
                                                                    RemEuclid,
                                                                ),
                                                                punctuation_token_idx: TokenIdx(
                                                                    44,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            43,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            3..4,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            45,
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 4,
                                                        argument: 5,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            39,
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
                                                            46,
                                                        ),
                                                        ident: `LineSegmentStroke`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                                                allow_self_type: True,
                                                allow_self_value: False,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: FieldType,
                                                    expr: 1,
                                                },
                                                ExprRoot {
                                                    kind: FieldType,
                                                    expr: 6,
                                                },
                                            ],
                                        },
                                    },
                                    implicit_parameter_decl_list: Ok(
                                        None,
                                    ),
                                    lcurl: LeftCurlyBraceToken(
                                        TokenIdx(
                                            35,
                                        ),
                                    ),
                                    field_comma_list: (
                                        [
                                            RegularStructFieldPattern {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 143,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        36,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        37,
                                                    ),
                                                ),
                                                ty: 1,
                                            },
                                            RegularStructFieldPattern {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 331,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        41,
                                                    ),
                                                },
                                                colon: ColonToken(
                                                    TokenIdx(
                                                        42,
                                                    ),
                                                ),
                                                ty: 6,
                                            },
                                        ],
                                        [
                                            CommaToken(
                                                TokenIdx(
                                                    40,
                                                ),
                                            ),
                                            CommaToken(
                                                TokenIdx(
                                                    47,
                                                ),
                                            ),
                                        ],
                                        Ok(
                                            (),
                                        ),
                                    ),
                                    rcurl: Ok(
                                        RightCurlyBraceToken(
                                            TokenIdx(
                                                48,
                                            ),
                                        ),
                                    ),
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Entity(
                    EntityPath::ModuleItem(
                        ModuleItemPath::Form(
                            FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
                        ),
                    ),
                ),
                Ok(
                    Defn::Form(
                        FormDefn::Function(
                            FunctionDefn {
                                path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
                                decl: FunctionDecl {
                                    path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
                                    ast_idx: 76,
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: None,
                                            path: RegionPath::Decl(
                                                DeclRegionPath::Entity(
                                                    EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
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
                                                    Expr::Prefix {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            520,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::List {
                                                        lbox_token_idx: TokenIdx(
                                                            524,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            525,
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::ExplicitApplication {
                                                        function: 2,
                                                        argument: 3,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            521,
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
                                                            526,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                    518,
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
                                                                        value: 143,
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
                                                                519,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
                                                                ident: `line_segment_sketch`,
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
                                                    expr: 4,
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
                                                    517,
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
                                                            519,
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
                                                        522,
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    curry_token: Ok(
                                        CurryToken(
                                            TokenIdx(
                                                523,
                                            ),
                                        ),
                                    ),
                                    return_ty: Ok(
                                        OutputTypeExpr {
                                            expr: 4,
                                        },
                                    ),
                                    eol_colon: Ok(
                                        EolColonToken(
                                            TokenIdx(
                                                527,
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
                                                                    FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
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
                                                            Expr::Prefix {
                                                                opr: Ref,
                                                                opr_token_idx: TokenIdx(
                                                                    520,
                                                                ),
                                                                opd: 0,
                                                            },
                                                            Expr::List {
                                                                lbox_token_idx: TokenIdx(
                                                                    524,
                                                                ),
                                                                items: ArenaIdxRange(
                                                                    2..2,
                                                                ),
                                                                rbox_token_idx: TokenIdx(
                                                                    525,
                                                                ),
                                                            },
                                                            Expr::EntityPath {
                                                                entity_path_expr: 1,
                                                                path: Some(
                                                                    EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        ),
                                                                    ),
                                                                ),
                                                            },
                                                            Expr::ExplicitApplication {
                                                                function: 2,
                                                                argument: 3,
                                                            },
                                                        ],
                                                    },
                                                    entity_path_expr_arena: Arena {
                                                        data: [
                                                            EntityPathExpr::Root {
                                                                token_idx: TokenIdx(
                                                                    521,
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
                                                                    526,
                                                                ),
                                                                ident: `ConcaveComponent`,
                                                                entity_path: EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                            518,
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
                                                                                value: 143,
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
                                                                        519,
                                                                    ),
                                                                    access_end: None,
                                                                    variant: CurrentSymbolVariant::RegularParameter {
                                                                        ident: `line_segment_sketch`,
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
                                                            expr: 4,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                        path: RegionPath::Defn(
                                            DefnRegionPath::Entity(
                                                EntityPath::ModuleItem(
                                                    ModuleItemPath::Form(
                                                        FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
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
                                                Expr::List {
                                                    lbox_token_idx: TokenIdx(
                                                        532,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        0..0,
                                                    ),
                                                    rbox_token_idx: TokenIdx(
                                                        533,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 0,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        535,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        1..1,
                                                    ),
                                                    commas: [],
                                                    rpar_token_idx: TokenIdx(
                                                        536,
                                                    ),
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
                                                Expr::ExplicitApplication {
                                                    function: 1,
                                                    argument: 2,
                                                },
                                                Expr::Binary {
                                                    lopd: 3,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        531,
                                                    ),
                                                    ropd: 4,
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        540,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 6,
                                                    dot_token_idx: TokenIdx(
                                                        541,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            542,
                                                        ),
                                                    },
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                539,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::MethodCall {
                                                    self_argument: 7,
                                                    dot_token_idx: TokenIdx(
                                                        543,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `ilen`,
                                                        token_idx: TokenIdx(
                                                            544,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        545,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        8..8,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        546,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 8,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        539,
                                                    ),
                                                    ropd: 9,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                550,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::Literal(
                                                    TokenIdx(
                                                        551,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 11,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        550,
                                                    ),
                                                    ropd: 12,
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                555,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::Literal(
                                                    TokenIdx(
                                                        556,
                                                    ),
                                                ),
                                                Expr::Binary {
                                                    lopd: 14,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        555,
                                                    ),
                                                    ropd: 15,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        561,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        558,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Prefix {
                                                    opr: Minus,
                                                    opr_token_idx: TokenIdx(
                                                        560,
                                                    ),
                                                    opd: 17,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 1,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        566,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        568,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 20,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        565,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        21..23,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            567,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        569,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 18,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        559,
                                                    ),
                                                    ropd: 19,
                                                },
                                                Expr::Prefix {
                                                    opr: Not,
                                                    opr_token_idx: TokenIdx(
                                                        563,
                                                    ),
                                                    opd: 23,
                                                },
                                                Expr::Binary {
                                                    lopd: 24,
                                                    opr: ShortCircuitLogic(
                                                        And,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        562,
                                                    ),
                                                    ropd: 25,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        571,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Suffix {
                                                    opd: 27,
                                                    opr: Decr,
                                                    opr_token_idx: TokenIdx(
                                                        572,
                                                    ),
                                                },
                                                Expr::Err(
                                                    ExprError::Original(
                                                        OriginalExprError::NoLeftOperandForBinaryOperator {
                                                            binary_token_idx: TokenIdx(
                                                                575,
                                                            ),
                                                        },
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        576,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 29,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        575,
                                                    ),
                                                    ropd: 30,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `ccv_start`,
                                                    token_idx: TokenIdx(
                                                        580,
                                                    ),
                                                    current_symbol_idx: 4,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 4,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        582,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        578,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 32,
                                                    opr: PureClosed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        581,
                                                    ),
                                                    ropd: 33,
                                                },
                                                Expr::Binary {
                                                    lopd: 34,
                                                    opr: Comparison(
                                                        Less,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        579,
                                                    ),
                                                    ropd: 35,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        587,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `L`,
                                                    token_idx: TokenIdx(
                                                        589,
                                                    ),
                                                    current_symbol_idx: 1,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 1,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        585,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 37,
                                                    opr: PureClosed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        588,
                                                    ),
                                                    ropd: 38,
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 2,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Form(
                                                                FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        594,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        596,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 41,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        593,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        42..44,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            595,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        597,
                                                    ),
                                                },
                                                Expr::Binary {
                                                    lopd: 39,
                                                    opr: Comparison(
                                                        Leq,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        586,
                                                    ),
                                                    ropd: 40,
                                                },
                                                Expr::Prefix {
                                                    opr: Not,
                                                    opr_token_idx: TokenIdx(
                                                        591,
                                                    ),
                                                    opd: 44,
                                                },
                                                Expr::Binary {
                                                    lopd: 45,
                                                    opr: ShortCircuitLogic(
                                                        And,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        590,
                                                    ),
                                                    ropd: 46,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        599,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Suffix {
                                                    opd: 48,
                                                    opr: Incr,
                                                    opr_token_idx: TokenIdx(
                                                        600,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        604,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        606,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        602,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 50,
                                                    opr: PureClosed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        605,
                                                    ),
                                                    ropd: 51,
                                                },
                                                Expr::Binary {
                                                    lopd: 52,
                                                    opr: Comparison(
                                                        Greater,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        603,
                                                    ),
                                                    ropd: 53,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `concave_components`,
                                                    token_idx: TokenIdx(
                                                        608,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::EntityPath {
                                                    entity_path_expr: 3,
                                                    path: Some(
                                                        EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            ),
                                                        ),
                                                    ),
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        616,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::Field {
                                                    owner: 57,
                                                    dot_token_idx: TokenIdx(
                                                        617,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `strokes`,
                                                        token_idx: TokenIdx(
                                                            618,
                                                        ),
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        622,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        624,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::InheritedSymbol {
                                                    ident: `line_segment_sketch`,
                                                    token_idx: TokenIdx(
                                                        614,
                                                    ),
                                                    inherited_symbol_idx: 0,
                                                    inherited_symbol_kind: InheritedSymbolKind::RegularParameter {
                                                        ident: `line_segment_sketch`,
                                                    },
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 58,
                                                    dot_token_idx: TokenIdx(
                                                        619,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `cyclic_slice`,
                                                        token_idx: TokenIdx(
                                                            620,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        621,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        59..61,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        625,
                                                    ),
                                                },
                                                Expr::ExplicitApplicationOrRitchieCall {
                                                    function: 56,
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        613,
                                                    ),
                                                    items: ArenaIdxRange(
                                                        61..63,
                                                    ),
                                                    commas: [
                                                        TokenIdx(
                                                            615,
                                                        ),
                                                    ],
                                                    rpar_token_idx: TokenIdx(
                                                        626,
                                                    ),
                                                },
                                                Expr::MethodCall {
                                                    self_argument: 55,
                                                    dot_token_idx: TokenIdx(
                                                        609,
                                                    ),
                                                    ident_token: IdentifierToken {
                                                        ident: `push`,
                                                        token_idx: TokenIdx(
                                                            610,
                                                        ),
                                                    },
                                                    implicit_arguments: None,
                                                    lpar_token_idx: TokenIdx(
                                                        611,
                                                    ),
                                                    nonself_arguments: ArenaIdxRange(
                                                        63..64,
                                                    ),
                                                    rpar_token_idx: TokenIdx(
                                                        627,
                                                    ),
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        628,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        630,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 65,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        629,
                                                    ),
                                                    ropd: 66,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `start`,
                                                    token_idx: TokenIdx(
                                                        633,
                                                    ),
                                                    current_symbol_idx: 2,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 2,
                                                    },
                                                },
                                                Expr::Literal(
                                                    TokenIdx(
                                                        635,
                                                    ),
                                                ),
                                                Expr::CurrentSymbol {
                                                    ident: `end`,
                                                    token_idx: TokenIdx(
                                                        631,
                                                    ),
                                                    current_symbol_idx: 3,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 3,
                                                    },
                                                },
                                                Expr::Binary {
                                                    lopd: 68,
                                                    opr: PureClosed(
                                                        Add,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        634,
                                                    ),
                                                    ropd: 69,
                                                },
                                                Expr::Binary {
                                                    lopd: 70,
                                                    opr: Assign(
                                                        None,
                                                    ),
                                                    opr_token_idx: TokenIdx(
                                                        632,
                                                    ),
                                                    ropd: 71,
                                                },
                                                Expr::CurrentSymbol {
                                                    ident: `concave_components`,
                                                    token_idx: TokenIdx(
                                                        637,
                                                    ),
                                                    current_symbol_idx: 0,
                                                    current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                        pattern_symbol_idx: 0,
                                                    },
                                                },
                                                Expr::Block {
                                                    stmts: ArenaIdxRange(
                                                        7..15,
                                                    ),
                                                },
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        534,
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
                                                        564,
                                                    ),
                                                    ident: `is_convex`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        592,
                                                    ),
                                                    ident: `is_convex`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Form(
                                                            FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                                                        ),
                                                    ),
                                                },
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        612,
                                                    ),
                                                    ident: `ConcaveComponent`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                        stmt_arena: Arena {
                                            data: [
                                                Stmt::Eval {
                                                    expr_idx: 28,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 49,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 64,
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            584,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        47,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolColonToken(
                                                            TokenIdx(
                                                                598,
                                                            ),
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            1..2,
                                                        ),
                                                    ),
                                                },
                                                Stmt::IfElse {
                                                    if_branch: IfBranch {
                                                        if_token: IfToken {
                                                            token_idx: TokenIdx(
                                                                601,
                                                            ),
                                                        },
                                                        condition: Ok(
                                                            54,
                                                        ),
                                                        eol_colon: Ok(
                                                            EolColonToken(
                                                                TokenIdx(
                                                                    607,
                                                                ),
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                2..3,
                                                            ),
                                                        ),
                                                    },
                                                    elif_branches: [],
                                                    else_branch: None,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 67,
                                                },
                                                Stmt::Eval {
                                                    expr_idx: 72,
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            528,
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
                                                                    531,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        5,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            537,
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
                                                                    539,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        10,
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            547,
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
                                                                    550,
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
                                                            552,
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
                                                                    555,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        16,
                                                    ),
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            557,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        26,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolColonToken(
                                                            TokenIdx(
                                                                570,
                                                            ),
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Let {
                                                    let_token: LetToken {
                                                        token_idx: TokenIdx(
                                                            573,
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
                                                                    575,
                                                                ),
                                                            ),
                                                        ),
                                                    ),
                                                    initial_value: Ok(
                                                        31,
                                                    ),
                                                },
                                                Stmt::While {
                                                    while_token: WhileToken {
                                                        token_idx: TokenIdx(
                                                            577,
                                                        ),
                                                    },
                                                    condition: Ok(
                                                        36,
                                                    ),
                                                    eol_colon: Ok(
                                                        EolColonToken(
                                                            TokenIdx(
                                                                583,
                                                            ),
                                                        ),
                                                    ),
                                                    block: Ok(
                                                        ArenaIdxRange(
                                                            3..7,
                                                        ),
                                                    ),
                                                },
                                                Stmt::Return {
                                                    return_token: ReturnToken {
                                                        token_idx: TokenIdx(
                                                            636,
                                                        ),
                                                    },
                                                    result: Ok(
                                                        73,
                                                    ),
                                                },
                                            ],
                                        },
                                        pattern_expr_region: PatternExprRegion {
                                            pattern_expr_arena: Arena {
                                                data: [
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `concave_components`,
                                                            token_idx: TokenIdx(
                                                                530,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `L`,
                                                            token_idx: TokenIdx(
                                                                538,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                549,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                554,
                                                            ),
                                                        },
                                                        liason: None,
                                                    },
                                                    PatternExpr::Identifier {
                                                        ident_token: IdentifierToken {
                                                            ident: `ccv_start`,
                                                            token_idx: TokenIdx(
                                                                574,
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
                                            ],
                                            pattern_symbol_maps: [
                                                [
                                                    (
                                                        Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 332,
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
                                                                    value: 339,
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
                                                                    value: 243,
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
                                                                    value: 244,
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
                                                                    value: 378,
                                                                },
                                                            ),
                                                        ),
                                                        4,
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
                                                ],
                                            },
                                            current_symbol_arena: Arena {
                                                data: [
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            531,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    638,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `concave_components`,
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            539,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    638,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `L`,
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            550,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    638,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `start`,
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            555,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    638,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `end`,
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    CurrentSymbol {
                                                        access_start: TokenIdx(
                                                            575,
                                                        ),
                                                        access_end: Some(
                                                            TokenIdxRangeEnd(
                                                                TokenIdx(
                                                                    638,
                                                                ),
                                                            ),
                                                        ),
                                                        variant: CurrentSymbolVariant::LetVariable {
                                                            ident: `ccv_start`,
                                                            pattern_symbol_idx: 4,
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
                                                expr: 74,
                                            },
                                        ],
                                    },
                                },
                                body: Ok(
                                    74,
                                ),
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::Impl(
                    ImplId {
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        impl_kind: ImplKind::Type {
                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        },
                        disambiguator: 0,
                    },
                ),
                Ok(
                    Defn::Impl(
                        ImplDecl::Type(
                            TypeImplDecl {
                                ast_idx: 75,
                                im: Impl {
                                    id: ImplId {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        impl_kind: ImplKind::Type {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        },
                                        disambiguator: 0,
                                    },
                                    ast_idx: 75,
                                    body: ArenaIdxRange(
                                        39..53,
                                    ),
                                    variant: ImplVariant::Type {
                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    },
                                },
                                impl_token: ImplToken {
                                    token_idx: TokenIdx(
                                        49,
                                    ),
                                },
                                implicit_parameter_decl_list: Ok(
                                    None,
                                ),
                                ty: TypeExpr {
                                    expr: 0,
                                },
                                eol_colon: Ok(
                                    EolColonToken(
                                        TokenIdx(
                                            51,
                                        ),
                                    ),
                                ),
                                expr_region: ExprRegion {
                                    data: ExprRegionData {
                                        parent: None,
                                        path: RegionPath::Decl(
                                            DeclRegionPath::Impl(
                                                ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
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
                                            ],
                                        },
                                        entity_path_expr_arena: Arena {
                                            data: [
                                                EntityPathExpr::Root {
                                                    token_idx: TokenIdx(
                                                        50,
                                                    ),
                                                    ident: `ConcaveComponent`,
                                                    entity_path: EntityPath::ModuleItem(
                                                        ModuleItemPath::Type(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                            allow_self_type: True,
                                            allow_self_value: False,
                                            pattern_ty_constraints: [],
                                        },
                                        roots: [
                                            ExprRoot {
                                                kind: SelfType,
                                                expr: 0,
                                            },
                                        ],
                                    },
                                },
                            },
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_id: ImplId {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `norm`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Memo(
                                TypeMemoDefn {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `norm`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    decl: TypeMemoDecl {
                                        path: Some(
                                            TypeItemPath {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `norm`,
                                                ty_item_kind: Memo,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ident: `norm`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `norm`,
                                                        ty_item_kind: Memo,
                                                    },
                                                ),
                                            ),
                                            im: Impl {
                                                id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 75,
                                                body: ArenaIdxRange(
                                                    39..53,
                                                ),
                                                variant: ImplVariant::Type {
                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                },
                                            },
                                            ast_idx: 39,
                                            ident: `norm`,
                                            associated_item_kind: TypeItem(
                                                Memo,
                                            ),
                                            accessibility: Accessibility::PublicUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 39,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::Impl(
                                                                    ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
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
                                                                ],
                                                            },
                                                            entity_path_expr_arena: Arena {
                                                                data: [
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                allow_self_type: True,
                                                                allow_self_value: False,
                                                                pattern_ty_constraints: [],
                                                            },
                                                            roots: [
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_id: ImplId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                impl_kind: ImplKind::Type {
                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                            ident: `norm`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
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
                                                    ],
                                                },
                                                entity_path_expr_arena: Arena {
                                                    data: [
                                                        EntityPathExpr::Root {
                                                            token_idx: TokenIdx(
                                                                55,
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
                                                    allow_self_type: True,
                                                    allow_self_value: True,
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
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    54,
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
                                                    56,
                                                ),
                                            ),
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::Impl(
                                                                            ImplId {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                impl_kind: ImplKind::Type {
                                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                },
                                                                                disambiguator: 0,
                                                                            },
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
                                                                        ],
                                                                    },
                                                                    entity_path_expr_arena: Arena {
                                                                        data: [
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        allow_self_type: True,
                                                                        allow_self_value: False,
                                                                        pattern_ty_constraints: [],
                                                                    },
                                                                    roots: [
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_id: ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `norm`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
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
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        55,
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
                                                            allow_self_type: True,
                                                            allow_self_value: True,
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
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `norm`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            57,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            58,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `hausdorff_norm`,
                                                            token_idx: TokenIdx(
                                                                59,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Eval {
                                                        expr_idx: 1,
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
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr: 2,
                                                },
                                            ],
                                        },
                                    },
                                    body: Ok(
                                        2,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_id: ImplId {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `rel_norm`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Memo(
                                TypeMemoDefn {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `rel_norm`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    decl: TypeMemoDecl {
                                        path: Some(
                                            TypeItemPath {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `rel_norm`,
                                                ty_item_kind: Memo,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ident: `rel_norm`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `rel_norm`,
                                                        ty_item_kind: Memo,
                                                    },
                                                ),
                                            ),
                                            im: Impl {
                                                id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 75,
                                                body: ArenaIdxRange(
                                                    39..53,
                                                ),
                                                variant: ImplVariant::Type {
                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                },
                                            },
                                            ast_idx: 40,
                                            ident: `rel_norm`,
                                            associated_item_kind: TypeItem(
                                                Memo,
                                            ),
                                            accessibility: Accessibility::PublicUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 40,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::Impl(
                                                                    ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
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
                                                                ],
                                                            },
                                                            entity_path_expr_arena: Arena {
                                                                data: [
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                allow_self_type: True,
                                                                allow_self_value: False,
                                                                pattern_ty_constraints: [],
                                                            },
                                                            roots: [
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_id: ImplId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                impl_kind: ImplKind::Type {
                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                            ident: `rel_norm`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
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
                                                    ],
                                                },
                                                entity_path_expr_arena: Arena {
                                                    data: [
                                                        EntityPathExpr::Root {
                                                            token_idx: TokenIdx(
                                                                63,
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
                                                    allow_self_type: True,
                                                    allow_self_value: True,
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
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    62,
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
                                                    64,
                                                ),
                                            ),
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::Impl(
                                                                            ImplId {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                impl_kind: ImplKind::Type {
                                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                },
                                                                                disambiguator: 0,
                                                                            },
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
                                                                        ],
                                                                    },
                                                                    entity_path_expr_arena: Arena {
                                                                        data: [
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        allow_self_type: True,
                                                                        allow_self_value: False,
                                                                        pattern_ty_constraints: [],
                                                                    },
                                                                    roots: [
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_id: ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `rel_norm`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
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
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        63,
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
                                                            allow_self_type: True,
                                                            allow_self_value: True,
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
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `rel_norm`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            65,
                                                        ),
                                                    ),
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            69,
                                                        ),
                                                    ),
                                                    Expr::MethodCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            70,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                71,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            72,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            73,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            66,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `norm`,
                                                            token_idx: TokenIdx(
                                                                67,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 2,
                                                        dot_token_idx: TokenIdx(
                                                            74,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `norm`,
                                                            token_idx: TokenIdx(
                                                                75,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            76,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            3..3,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            77,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 3,
                                                        opr: PureClosed(
                                                            Div,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            68,
                                                        ),
                                                        ropd: 4,
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            0..1,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
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
                                                allow_self_type: True,
                                                allow_self_value: True,
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
                                    body: Ok(
                                        6,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_id: ImplId {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `hausdorff_norm`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Memo(
                                TypeMemoDefn {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `hausdorff_norm`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    decl: TypeMemoDecl {
                                        path: Some(
                                            TypeItemPath {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `hausdorff_norm`,
                                                ty_item_kind: Memo,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ident: `hausdorff_norm`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `hausdorff_norm`,
                                                        ty_item_kind: Memo,
                                                    },
                                                ),
                                            ),
                                            im: Impl {
                                                id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 75,
                                                body: ArenaIdxRange(
                                                    39..53,
                                                ),
                                                variant: ImplVariant::Type {
                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                },
                                            },
                                            ast_idx: 41,
                                            ident: `hausdorff_norm`,
                                            associated_item_kind: TypeItem(
                                                Memo,
                                            ),
                                            accessibility: Accessibility::PublicUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 41,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::Impl(
                                                                    ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
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
                                                                ],
                                                            },
                                                            entity_path_expr_arena: Arena {
                                                                data: [
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                allow_self_type: True,
                                                                allow_self_value: False,
                                                                pattern_ty_constraints: [],
                                                            },
                                                            roots: [
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_id: ImplId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                impl_kind: ImplKind::Type {
                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                            ident: `hausdorff_norm`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
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
                                                    ],
                                                },
                                                entity_path_expr_arena: Arena {
                                                    data: [
                                                        EntityPathExpr::Root {
                                                            token_idx: TokenIdx(
                                                                81,
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
                                                    allow_self_type: True,
                                                    allow_self_value: True,
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
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    80,
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
                                                    82,
                                                ),
                                            ),
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::Impl(
                                                                            ImplId {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                impl_kind: ImplKind::Type {
                                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                },
                                                                                disambiguator: 0,
                                                                            },
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
                                                                        ],
                                                                    },
                                                                    entity_path_expr_arena: Arena {
                                                                        data: [
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        allow_self_type: True,
                                                                        allow_self_value: False,
                                                                        pattern_ty_constraints: [],
                                                                    },
                                                                    roots: [
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_id: ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `hausdorff_norm`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
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
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        81,
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
                                                            allow_self_type: True,
                                                            allow_self_value: True,
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
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `hausdorff_norm`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    86,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            87,
                                                        ),
                                                    ),
                                                    Expr::Binary {
                                                        lopd: 0,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            86,
                                                        ),
                                                        ropd: 1,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            91,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 3,
                                                        dot_token_idx: TokenIdx(
                                                            92,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                93,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 4,
                                                        dot_token_idx: TokenIdx(
                                                            94,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `firstx`,
                                                            token_idx: TokenIdx(
                                                                95,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            96,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            5..5,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            97,
                                                        ),
                                                    },
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    90,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 5,
                                                        dot_token_idx: TokenIdx(
                                                            98,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                99,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 6,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            90,
                                                        ),
                                                        ropd: 7,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            103,
                                                        ),
                                                    ),
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    102,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::MethodCall {
                                                        self_argument: 9,
                                                        dot_token_idx: TokenIdx(
                                                            104,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `line_segment`,
                                                            token_idx: TokenIdx(
                                                                105,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            106,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            10..10,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            107,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 10,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            102,
                                                        ),
                                                        ropd: 11,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `curve_ls`,
                                                        token_idx: TokenIdx(
                                                            111,
                                                        ),
                                                        current_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 13,
                                                        dot_token_idx: TokenIdx(
                                                            112,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                113,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            114,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            14..14,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            115,
                                                        ),
                                                    },
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    110,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::MethodCall {
                                                        self_argument: 14,
                                                        dot_token_idx: TokenIdx(
                                                            116,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `norm`,
                                                            token_idx: TokenIdx(
                                                                117,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            118,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            15..15,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            119,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 15,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            110,
                                                        ),
                                                        ropd: 16,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            121,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 18,
                                                        dot_token_idx: TokenIdx(
                                                            122,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                123,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 19,
                                                        dot_token_idx: TokenIdx(
                                                            124,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                125,
                                                            ),
                                                        },
                                                    },
                                                    Expr::FrameVarDecl {
                                                        token_idx: TokenIdx(
                                                            127,
                                                        ),
                                                        ident: `i`,
                                                        frame_var_symbol_idx: 4,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            21,
                                                        ),
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            129,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 22,
                                                        dot_token_idx: TokenIdx(
                                                            130,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                131,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 20,
                                                        opr: Comparison(
                                                            Leq,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            126,
                                                        ),
                                                        ropd: 21,
                                                    },
                                                    Expr::Field {
                                                        owner: 23,
                                                        dot_token_idx: TokenIdx(
                                                            132,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                133,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 24,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            128,
                                                        ),
                                                        ropd: 25,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            138,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 27,
                                                        dot_token_idx: TokenIdx(
                                                            139,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                140,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            142,
                                                        ),
                                                        current_symbol_idx: 4,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            21,
                                                        ),
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 28,
                                                        lbox_token_idx: TokenIdx(
                                                            141,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            29..30,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            143,
                                                        ),
                                                    },
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    137,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 30,
                                                        dot_token_idx: TokenIdx(
                                                            144,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                145,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 31,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            137,
                                                        ),
                                                        ropd: 32,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `curve_ls`,
                                                        token_idx: TokenIdx(
                                                            149,
                                                        ),
                                                        current_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `point`,
                                                        token_idx: TokenIdx(
                                                            153,
                                                        ),
                                                        current_symbol_idx: 5,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    148,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::MethodCall {
                                                        self_argument: 34,
                                                        dot_token_idx: TokenIdx(
                                                            150,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `dist_to_point`,
                                                            token_idx: TokenIdx(
                                                                151,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            152,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            35..36,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            154,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 36,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            148,
                                                        ),
                                                        ropd: 37,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `point_dist`,
                                                        token_idx: TokenIdx(
                                                            156,
                                                        ),
                                                        current_symbol_idx: 6,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `hausdorff_norm`,
                                                        token_idx: TokenIdx(
                                                            158,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 39,
                                                        opr: Comparison(
                                                            Greater,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            157,
                                                        ),
                                                        ropd: 40,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `hausdorff_norm`,
                                                        token_idx: TokenIdx(
                                                            160,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `point_dist`,
                                                        token_idx: TokenIdx(
                                                            162,
                                                        ),
                                                        current_symbol_idx: 6,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 42,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            161,
                                                        ),
                                                        ropd: 43,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `hausdorff_norm`,
                                                        token_idx: TokenIdx(
                                                            164,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            4..10,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Eval {
                                                        expr_idx: 44,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                135,
                                                            ),
                                                        },
                                                        let_variable_pattern: Ok(
                                                            LetVariablesPattern {
                                                                pattern: 4,
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
                                                                        137,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            33,
                                                        ),
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                146,
                                                            ),
                                                        },
                                                        let_variable_pattern: Ok(
                                                            LetVariablesPattern {
                                                                pattern: 5,
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
                                                                        148,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            38,
                                                        ),
                                                    },
                                                    Stmt::IfElse {
                                                        if_branch: IfBranch {
                                                            if_token: IfToken {
                                                                token_idx: TokenIdx(
                                                                    155,
                                                                ),
                                                            },
                                                            condition: Ok(
                                                                41,
                                                            ),
                                                            eol_colon: Ok(
                                                                EolColonToken(
                                                                    TokenIdx(
                                                                        159,
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
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                83,
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
                                                                        86,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            2,
                                                        ),
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                88,
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
                                                                        90,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            8,
                                                        ),
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                100,
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
                                                                        102,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            12,
                                                        ),
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                108,
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
                                                                        110,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            17,
                                                        ),
                                                    },
                                                    Stmt::ForBetween {
                                                        for_token: ForToken {
                                                            token_idx: TokenIdx(
                                                                120,
                                                            ),
                                                        },
                                                        particulars: ForBetweenParticulars {
                                                            frame_var_token_idx: TokenIdx(
                                                                127,
                                                            ),
                                                            frame_var_expr_idx: 21,
                                                            frame_var_ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 191,
                                                                    },
                                                                ),
                                                            ),
                                                            range: ForBetweenRange {
                                                                initial_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        20,
                                                                    ),
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        25,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        },
                                                        frame_var_symbol_idx: 4,
                                                        eol_colon: Ok(
                                                            EolColonToken(
                                                                TokenIdx(
                                                                    134,
                                                                ),
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                1..4,
                                                            ),
                                                        ),
                                                    },
                                                    Stmt::Return {
                                                        return_token: ReturnToken {
                                                            token_idx: TokenIdx(
                                                                163,
                                                            ),
                                                        },
                                                        result: Ok(
                                                            45,
                                                        ),
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `hausdorff_norm`,
                                                                token_idx: TokenIdx(
                                                                    85,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `curve_start`,
                                                                token_idx: TokenIdx(
                                                                    89,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `curve_ls`,
                                                                token_idx: TokenIdx(
                                                                    101,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `dp_norm`,
                                                                token_idx: TokenIdx(
                                                                    109,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `point`,
                                                                token_idx: TokenIdx(
                                                                    136,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `point_dist`,
                                                                token_idx: TokenIdx(
                                                                    147,
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
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 369,
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
                                                                        value: 371,
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
                                                                        value: 372,
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
                                                                        value: 350,
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
                                                                        value: 236,
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
                                                                        value: 373,
                                                                    },
                                                                ),
                                                            ),
                                                            5,
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
                                                                86,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        165,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `hausdorff_norm`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                90,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        165,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `curve_start`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                102,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        165,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `curve_ls`,
                                                                pattern_symbol_idx: 2,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                110,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        165,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `dp_norm`,
                                                                pattern_symbol_idx: 3,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                135,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        163,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::FrameVariable {
                                                                ident: `i`,
                                                                expr_idx: 21,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                137,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        163,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `point`,
                                                                pattern_symbol_idx: 4,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                148,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        163,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `point_dist`,
                                                                pattern_symbol_idx: 5,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    FrameVariable,
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr: 46,
                                                },
                                            ],
                                        },
                                    },
                                    body: Ok(
                                        46,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_id: ImplId {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `angle_change`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Memo(
                                TypeMemoDefn {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `angle_change`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    decl: TypeMemoDecl {
                                        path: Some(
                                            TypeItemPath {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `angle_change`,
                                                ty_item_kind: Memo,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ident: `angle_change`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `angle_change`,
                                                        ty_item_kind: Memo,
                                                    },
                                                ),
                                            ),
                                            im: Impl {
                                                id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 75,
                                                body: ArenaIdxRange(
                                                    39..53,
                                                ),
                                                variant: ImplVariant::Type {
                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                },
                                            },
                                            ast_idx: 42,
                                            ident: `angle_change`,
                                            associated_item_kind: TypeItem(
                                                Memo,
                                            ),
                                            accessibility: Accessibility::PublicUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 42,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::Impl(
                                                                    ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
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
                                                                ],
                                                            },
                                                            entity_path_expr_arena: Arena {
                                                                data: [
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                allow_self_type: True,
                                                                allow_self_value: False,
                                                                pattern_ty_constraints: [],
                                                            },
                                                            roots: [
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_id: ImplId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                impl_kind: ImplKind::Type {
                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                            ident: `angle_change`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
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
                                                    ],
                                                },
                                                entity_path_expr_arena: Arena {
                                                    data: [
                                                        EntityPathExpr::Root {
                                                            token_idx: TokenIdx(
                                                                168,
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
                                                    allow_self_type: True,
                                                    allow_self_value: True,
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
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    167,
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
                                                    169,
                                                ),
                                            ),
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::Impl(
                                                                            ImplId {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                impl_kind: ImplKind::Type {
                                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                },
                                                                                disambiguator: 0,
                                                                            },
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
                                                                        ],
                                                                    },
                                                                    entity_path_expr_arena: Arena {
                                                                        data: [
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        allow_self_type: True,
                                                                        allow_self_value: False,
                                                                        pattern_ty_constraints: [],
                                                                    },
                                                                    roots: [
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_id: ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `angle_change`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
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
                                                            ],
                                                        },
                                                        entity_path_expr_arena: Arena {
                                                            data: [
                                                                EntityPathExpr::Root {
                                                                    token_idx: TokenIdx(
                                                                        168,
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
                                                            allow_self_type: True,
                                                            allow_self_value: True,
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
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `angle_change`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    173,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            174,
                                                        ),
                                                    ),
                                                    Expr::Binary {
                                                        lopd: 0,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            173,
                                                        ),
                                                        ropd: 1,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            179,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 3,
                                                        dot_token_idx: TokenIdx(
                                                            180,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                181,
                                                            ),
                                                        },
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            183,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 5,
                                                        dot_token_idx: TokenIdx(
                                                            184,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                185,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 6,
                                                        dot_token_idx: TokenIdx(
                                                            186,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                187,
                                                            ),
                                                        },
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 4,
                                                        lbox_token_idx: TokenIdx(
                                                            182,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            7..8,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            188,
                                                        ),
                                                    },
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    178,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::MethodCall {
                                                        self_argument: 8,
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
                                                            9..9,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            192,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 9,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            178,
                                                        ),
                                                        ropd: 10,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            194,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 12,
                                                        dot_token_idx: TokenIdx(
                                                            195,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                196,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 13,
                                                        dot_token_idx: TokenIdx(
                                                            197,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                198,
                                                            ),
                                                        },
                                                    },
                                                    Expr::FrameVarDecl {
                                                        token_idx: TokenIdx(
                                                            200,
                                                        ),
                                                        ident: `i`,
                                                        frame_var_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            15,
                                                        ),
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            202,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 16,
                                                        dot_token_idx: TokenIdx(
                                                            203,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                204,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 14,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            199,
                                                        ),
                                                        ropd: 15,
                                                    },
                                                    Expr::Field {
                                                        owner: 17,
                                                        dot_token_idx: TokenIdx(
                                                            205,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                206,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 18,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            201,
                                                        ),
                                                        ropd: 19,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            211,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 21,
                                                        dot_token_idx: TokenIdx(
                                                            212,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                213,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            215,
                                                        ),
                                                        current_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            15,
                                                        ),
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 22,
                                                        lbox_token_idx: TokenIdx(
                                                            214,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            23..24,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            216,
                                                        ),
                                                    },
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    210,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::MethodCall {
                                                        self_argument: 24,
                                                        dot_token_idx: TokenIdx(
                                                            217,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                218,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            219,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            25..25,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            220,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 25,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            210,
                                                        ),
                                                        ropd: 26,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `dp0`,
                                                        token_idx: TokenIdx(
                                                            223,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `dp`,
                                                        token_idx: TokenIdx(
                                                            227,
                                                        ),
                                                        current_symbol_idx: 3,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    Expr::Literal(
                                                        TokenIdx(
                                                            229,
                                                        ),
                                                    ),
                                                    Expr::CurrentSymbol {
                                                        ident: `angle_change`,
                                                        token_idx: TokenIdx(
                                                            221,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 28,
                                                        dot_token_idx: TokenIdx(
                                                            224,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `angle_to`,
                                                            token_idx: TokenIdx(
                                                                225,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            226,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            29..31,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            230,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 31,
                                                        opr: Assign(
                                                            Some(
                                                                Add,
                                                            ),
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            222,
                                                        ),
                                                        ropd: 32,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `dp0`,
                                                        token_idx: TokenIdx(
                                                            231,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `dp`,
                                                        token_idx: TokenIdx(
                                                            233,
                                                        ),
                                                        current_symbol_idx: 3,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 34,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            232,
                                                        ),
                                                        ropd: 35,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `angle_change`,
                                                        token_idx: TokenIdx(
                                                            235,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            3..7,
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
                                                                208,
                                                            ),
                                                        },
                                                        let_variable_pattern: Ok(
                                                            LetVariablesPattern {
                                                                pattern: 2,
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
                                                                        210,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            27,
                                                        ),
                                                    },
                                                    Stmt::Eval {
                                                        expr_idx: 33,
                                                    },
                                                    Stmt::Eval {
                                                        expr_idx: 36,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                170,
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
                                                                        173,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            2,
                                                        ),
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                175,
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
                                                                        178,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            11,
                                                        ),
                                                    },
                                                    Stmt::ForBetween {
                                                        for_token: ForToken {
                                                            token_idx: TokenIdx(
                                                                193,
                                                            ),
                                                        },
                                                        particulars: ForBetweenParticulars {
                                                            frame_var_token_idx: TokenIdx(
                                                                200,
                                                            ),
                                                            frame_var_expr_idx: 15,
                                                            frame_var_ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 191,
                                                                    },
                                                                ),
                                                            ),
                                                            range: ForBetweenRange {
                                                                initial_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        14,
                                                                    ),
                                                                    kind: LowerOpen,
                                                                },
                                                                final_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        19,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        },
                                                        frame_var_symbol_idx: 2,
                                                        eol_colon: Ok(
                                                            EolColonToken(
                                                                TokenIdx(
                                                                    207,
                                                                ),
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                0..3,
                                                            ),
                                                        ),
                                                    },
                                                    Stmt::Return {
                                                        return_token: ReturnToken {
                                                            token_idx: TokenIdx(
                                                                234,
                                                            ),
                                                        },
                                                        result: Ok(
                                                            37,
                                                        ),
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `angle_change`,
                                                                token_idx: TokenIdx(
                                                                    172,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `dp0`,
                                                                token_idx: TokenIdx(
                                                                    177,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `dp`,
                                                                token_idx: TokenIdx(
                                                                    209,
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
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 297,
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
                                                                        value: 355,
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
                                                                        value: 345,
                                                                    },
                                                                ),
                                                            ),
                                                            2,
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
                                                                173,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        236,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `angle_change`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                178,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        236,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `dp0`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                208,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        234,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::FrameVariable {
                                                                ident: `i`,
                                                                expr_idx: 15,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                210,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        234,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `dp`,
                                                                pattern_symbol_idx: 2,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    FrameVariable,
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr: 38,
                                                },
                                            ],
                                        },
                                    },
                                    body: Ok(
                                        38,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_id: ImplId {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `bounding_box`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Memo(
                                TypeMemoDefn {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `bounding_box`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    decl: TypeMemoDecl {
                                        path: Some(
                                            TypeItemPath {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `bounding_box`,
                                                ty_item_kind: Memo,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ident: `bounding_box`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `bounding_box`,
                                                        ty_item_kind: Memo,
                                                    },
                                                ),
                                            ),
                                            im: Impl {
                                                id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 75,
                                                body: ArenaIdxRange(
                                                    39..53,
                                                ),
                                                variant: ImplVariant::Type {
                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                },
                                            },
                                            ast_idx: 43,
                                            ident: `bounding_box`,
                                            associated_item_kind: TypeItem(
                                                Memo,
                                            ),
                                            accessibility: Accessibility::PublicUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 43,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::Impl(
                                                                    ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
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
                                                                ],
                                                            },
                                                            entity_path_expr_arena: Arena {
                                                                data: [
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                allow_self_type: True,
                                                                allow_self_value: False,
                                                                pattern_ty_constraints: [],
                                                            },
                                                            roots: [
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_id: ImplId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                impl_kind: ImplKind::Type {
                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                            ident: `bounding_box`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                239,
                                                            ),
                                                            ident: `BoundingBox`,
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                    allow_self_type: True,
                                                    allow_self_value: True,
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
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    238,
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
                                                    240,
                                                ),
                                            ),
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::Impl(
                                                                            ImplId {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                impl_kind: ImplKind::Type {
                                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                },
                                                                                disambiguator: 0,
                                                                            },
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
                                                                        ],
                                                                    },
                                                                    entity_path_expr_arena: Arena {
                                                                        data: [
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        allow_self_type: True,
                                                                        allow_self_value: False,
                                                                        pattern_ty_constraints: [],
                                                                    },
                                                                    roots: [
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_id: ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `bounding_box`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                                        239,
                                                                    ),
                                                                    ident: `BoundingBox`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                                            allow_self_type: True,
                                                            allow_self_value: True,
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
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `bounding_box`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            244,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            245,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                246,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            247,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `firstx`,
                                                            token_idx: TokenIdx(
                                                                248,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            249,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            250,
                                                        ),
                                                    },
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    243,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 2,
                                                        dot_token_idx: TokenIdx(
                                                            251,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                252,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 3,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            243,
                                                        ),
                                                        ropd: 4,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `start_point`,
                                                        token_idx: TokenIdx(
                                                            257,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    256,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 6,
                                                        dot_token_idx: TokenIdx(
                                                            258,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `x`,
                                                            token_idx: TokenIdx(
                                                                259,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 7,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            256,
                                                        ),
                                                        ropd: 8,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `start_point`,
                                                        token_idx: TokenIdx(
                                                            264,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    263,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 10,
                                                        dot_token_idx: TokenIdx(
                                                            265,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `x`,
                                                            token_idx: TokenIdx(
                                                                266,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 11,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            263,
                                                        ),
                                                        ropd: 12,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `start_point`,
                                                        token_idx: TokenIdx(
                                                            271,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    270,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 14,
                                                        dot_token_idx: TokenIdx(
                                                            272,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `y`,
                                                            token_idx: TokenIdx(
                                                                273,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 15,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            270,
                                                        ),
                                                        ropd: 16,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `start_point`,
                                                        token_idx: TokenIdx(
                                                            278,
                                                        ),
                                                        current_symbol_idx: 0,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 0,
                                                        },
                                                    },
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    277,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 18,
                                                        dot_token_idx: TokenIdx(
                                                            279,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `y`,
                                                            token_idx: TokenIdx(
                                                                280,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 19,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            277,
                                                        ),
                                                        ropd: 20,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            282,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 22,
                                                        dot_token_idx: TokenIdx(
                                                            283,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                284,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 23,
                                                        dot_token_idx: TokenIdx(
                                                            285,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                286,
                                                            ),
                                                        },
                                                    },
                                                    Expr::FrameVarDecl {
                                                        token_idx: TokenIdx(
                                                            288,
                                                        ),
                                                        ident: `i`,
                                                        frame_var_symbol_idx: 5,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            25,
                                                        ),
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            290,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 26,
                                                        dot_token_idx: TokenIdx(
                                                            291,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                292,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 24,
                                                        opr: Comparison(
                                                            Leq,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            287,
                                                        ),
                                                        ropd: 25,
                                                    },
                                                    Expr::Field {
                                                        owner: 27,
                                                        dot_token_idx: TokenIdx(
                                                            293,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                294,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 28,
                                                        opr: Comparison(
                                                            Less,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            289,
                                                        ),
                                                        ropd: 29,
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            299,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 31,
                                                        dot_token_idx: TokenIdx(
                                                            300,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                301,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `i`,
                                                        token_idx: TokenIdx(
                                                            303,
                                                        ),
                                                        current_symbol_idx: 5,
                                                        current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                            25,
                                                        ),
                                                    },
                                                    Expr::IndexOrCompositionWithList {
                                                        owner: 32,
                                                        lbox_token_idx: TokenIdx(
                                                            302,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            33..34,
                                                        ),
                                                        rbox_token_idx: TokenIdx(
                                                            304,
                                                        ),
                                                    },
                                                    Expr::Err(
                                                        ExprError::Original(
                                                            OriginalExprError::NoLeftOperandForBinaryOperator {
                                                                binary_token_idx: TokenIdx(
                                                                    298,
                                                                ),
                                                            },
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 34,
                                                        dot_token_idx: TokenIdx(
                                                            305,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                306,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Binary {
                                                        lopd: 35,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            298,
                                                        ),
                                                        ropd: 36,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `xmin`,
                                                        token_idx: TokenIdx(
                                                            309,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `point`,
                                                        token_idx: TokenIdx(
                                                            313,
                                                        ),
                                                        current_symbol_idx: 6,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 39,
                                                        dot_token_idx: TokenIdx(
                                                            314,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `x`,
                                                            token_idx: TokenIdx(
                                                                315,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `xmin`,
                                                        token_idx: TokenIdx(
                                                            307,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 38,
                                                        dot_token_idx: TokenIdx(
                                                            310,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `min`,
                                                            token_idx: TokenIdx(
                                                                311,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            312,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            40..41,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            316,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 41,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            308,
                                                        ),
                                                        ropd: 42,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `xmax`,
                                                        token_idx: TokenIdx(
                                                            319,
                                                        ),
                                                        current_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `point`,
                                                        token_idx: TokenIdx(
                                                            323,
                                                        ),
                                                        current_symbol_idx: 6,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 45,
                                                        dot_token_idx: TokenIdx(
                                                            324,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `x`,
                                                            token_idx: TokenIdx(
                                                                325,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `xmax`,
                                                        token_idx: TokenIdx(
                                                            317,
                                                        ),
                                                        current_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 44,
                                                        dot_token_idx: TokenIdx(
                                                            320,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `max`,
                                                            token_idx: TokenIdx(
                                                                321,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            322,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            46..47,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            326,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 47,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            318,
                                                        ),
                                                        ropd: 48,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `ymin`,
                                                        token_idx: TokenIdx(
                                                            329,
                                                        ),
                                                        current_symbol_idx: 3,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `point`,
                                                        token_idx: TokenIdx(
                                                            333,
                                                        ),
                                                        current_symbol_idx: 6,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 51,
                                                        dot_token_idx: TokenIdx(
                                                            334,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `y`,
                                                            token_idx: TokenIdx(
                                                                335,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `ymin`,
                                                        token_idx: TokenIdx(
                                                            327,
                                                        ),
                                                        current_symbol_idx: 3,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 50,
                                                        dot_token_idx: TokenIdx(
                                                            330,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `min`,
                                                            token_idx: TokenIdx(
                                                                331,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            332,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            52..53,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            336,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 53,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            328,
                                                        ),
                                                        ropd: 54,
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            339,
                                                        ),
                                                        current_symbol_idx: 4,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `point`,
                                                        token_idx: TokenIdx(
                                                            343,
                                                        ),
                                                        current_symbol_idx: 6,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 5,
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 57,
                                                        dot_token_idx: TokenIdx(
                                                            344,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `y`,
                                                            token_idx: TokenIdx(
                                                                345,
                                                            ),
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            337,
                                                        ),
                                                        current_symbol_idx: 4,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 56,
                                                        dot_token_idx: TokenIdx(
                                                            340,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `max`,
                                                            token_idx: TokenIdx(
                                                                341,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            342,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            58..59,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            346,
                                                        ),
                                                    },
                                                    Expr::Binary {
                                                        lopd: 59,
                                                        opr: Assign(
                                                            None,
                                                        ),
                                                        opr_token_idx: TokenIdx(
                                                            338,
                                                        ),
                                                        ropd: 60,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `xmin`,
                                                        token_idx: TokenIdx(
                                                            352,
                                                        ),
                                                        current_symbol_idx: 1,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 1,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `xmax`,
                                                        token_idx: TokenIdx(
                                                            354,
                                                        ),
                                                        current_symbol_idx: 2,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 2,
                                                        },
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 2,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `ymin`,
                                                        token_idx: TokenIdx(
                                                            359,
                                                        ),
                                                        current_symbol_idx: 3,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 3,
                                                        },
                                                    },
                                                    Expr::CurrentSymbol {
                                                        ident: `ymax`,
                                                        token_idx: TokenIdx(
                                                            361,
                                                        ),
                                                        current_symbol_idx: 4,
                                                        current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                            pattern_symbol_idx: 4,
                                                        },
                                                    },
                                                    Expr::ExplicitApplicationOrRitchieCall {
                                                        function: 63,
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            351,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            64..66,
                                                        ),
                                                        commas: [
                                                            TokenIdx(
                                                                353,
                                                            ),
                                                        ],
                                                        rpar_token_idx: TokenIdx(
                                                            355,
                                                        ),
                                                    },
                                                    Expr::ExplicitApplicationOrRitchieCall {
                                                        function: 66,
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            358,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            67..69,
                                                        ),
                                                        commas: [
                                                            TokenIdx(
                                                                360,
                                                            ),
                                                        ],
                                                        rpar_token_idx: TokenIdx(
                                                            362,
                                                        ),
                                                    },
                                                    Expr::ExplicitApplicationOrRitchieCall {
                                                        function: 62,
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            349,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            69..71,
                                                        ),
                                                        commas: [
                                                            TokenIdx(
                                                                356,
                                                            ),
                                                            TokenIdx(
                                                                363,
                                                            ),
                                                        ],
                                                        rpar_token_idx: TokenIdx(
                                                            364,
                                                        ),
                                                    },
                                                    Expr::Block {
                                                        stmts: ArenaIdxRange(
                                                            5..12,
                                                        ),
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            348,
                                                        ),
                                                        ident: `BoundingBox`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            350,
                                                        ),
                                                        ident: `ClosedRange`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            357,
                                                        ),
                                                        ident: `ClosedRange`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                                                296,
                                                            ),
                                                        },
                                                        let_variable_pattern: Ok(
                                                            LetVariablesPattern {
                                                                pattern: 5,
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
                                                                        298,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            37,
                                                        ),
                                                    },
                                                    Stmt::Eval {
                                                        expr_idx: 43,
                                                    },
                                                    Stmt::Eval {
                                                        expr_idx: 49,
                                                    },
                                                    Stmt::Eval {
                                                        expr_idx: 55,
                                                    },
                                                    Stmt::Eval {
                                                        expr_idx: 61,
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                241,
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
                                                                        243,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            5,
                                                        ),
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                253,
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
                                                                        256,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            9,
                                                        ),
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                260,
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
                                                                        263,
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
                                                                267,
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
                                                                        270,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            17,
                                                        ),
                                                    },
                                                    Stmt::Let {
                                                        let_token: LetToken {
                                                            token_idx: TokenIdx(
                                                                274,
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
                                                                        277,
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        initial_value: Ok(
                                                            21,
                                                        ),
                                                    },
                                                    Stmt::ForBetween {
                                                        for_token: ForToken {
                                                            token_idx: TokenIdx(
                                                                281,
                                                            ),
                                                        },
                                                        particulars: ForBetweenParticulars {
                                                            frame_var_token_idx: TokenIdx(
                                                                288,
                                                            ),
                                                            frame_var_expr_idx: 25,
                                                            frame_var_ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 191,
                                                                    },
                                                                ),
                                                            ),
                                                            range: ForBetweenRange {
                                                                initial_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        24,
                                                                    ),
                                                                    kind: LowerClosed,
                                                                },
                                                                final_boundary: LoopBoundary {
                                                                    bound_expr: Some(
                                                                        29,
                                                                    ),
                                                                    kind: UpperOpen,
                                                                },
                                                                step: Constant(
                                                                    1,
                                                                ),
                                                            },
                                                        },
                                                        frame_var_symbol_idx: 5,
                                                        eol_colon: Ok(
                                                            EolColonToken(
                                                                TokenIdx(
                                                                    295,
                                                                ),
                                                            ),
                                                        ),
                                                        block: Ok(
                                                            ArenaIdxRange(
                                                                0..5,
                                                            ),
                                                        ),
                                                    },
                                                    Stmt::Return {
                                                        return_token: ReturnToken {
                                                            token_idx: TokenIdx(
                                                                347,
                                                            ),
                                                        },
                                                        result: Ok(
                                                            71,
                                                        ),
                                                    },
                                                ],
                                            },
                                            pattern_expr_region: PatternExprRegion {
                                                pattern_expr_arena: Arena {
                                                    data: [
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `start_point`,
                                                                token_idx: TokenIdx(
                                                                    242,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `xmin`,
                                                                token_idx: TokenIdx(
                                                                    255,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `xmax`,
                                                                token_idx: TokenIdx(
                                                                    262,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `ymin`,
                                                                token_idx: TokenIdx(
                                                                    269,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `ymax`,
                                                                token_idx: TokenIdx(
                                                                    276,
                                                                ),
                                                            },
                                                            liason: None,
                                                        },
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `point`,
                                                                token_idx: TokenIdx(
                                                                    297,
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
                                                ],
                                                pattern_symbol_maps: [
                                                    [
                                                        (
                                                            Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 231,
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
                                                                        value: 232,
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
                                                                        value: 233,
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
                                                                        value: 234,
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
                                                                        value: 235,
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
                                                                        value: 236,
                                                                    },
                                                                ),
                                                            ),
                                                            5,
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
                                                                243,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        365,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `start_point`,
                                                                pattern_symbol_idx: 0,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                256,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        365,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `xmin`,
                                                                pattern_symbol_idx: 1,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                263,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        365,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `xmax`,
                                                                pattern_symbol_idx: 2,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                270,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        365,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `ymin`,
                                                                pattern_symbol_idx: 3,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                277,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        365,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `ymax`,
                                                                pattern_symbol_idx: 4,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                296,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        347,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::FrameVariable {
                                                                ident: `i`,
                                                                expr_idx: 25,
                                                            },
                                                        },
                                                        CurrentSymbol {
                                                            access_start: TokenIdx(
                                                                298,
                                                            ),
                                                            access_end: Some(
                                                                TokenIdxRangeEnd(
                                                                    TokenIdx(
                                                                        347,
                                                                    ),
                                                                ),
                                                            ),
                                                            variant: CurrentSymbolVariant::LetVariable {
                                                                ident: `point`,
                                                                pattern_symbol_idx: 5,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [
                                                    FrameVariable,
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr: 72,
                                                },
                                            ],
                                        },
                                    },
                                    body: Ok(
                                        72,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_id: ImplId {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `relative_bounding_box`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Memo(
                                TypeMemoDefn {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `relative_bounding_box`,
                                            ty_item_kind: Memo,
                                        },
                                    ),
                                    decl: TypeMemoDecl {
                                        path: Some(
                                            TypeItemPath {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `relative_bounding_box`,
                                                ty_item_kind: Memo,
                                            },
                                        ),
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ident: `relative_bounding_box`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `relative_bounding_box`,
                                                        ty_item_kind: Memo,
                                                    },
                                                ),
                                            ),
                                            im: Impl {
                                                id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 75,
                                                body: ArenaIdxRange(
                                                    39..53,
                                                ),
                                                variant: ImplVariant::Type {
                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                },
                                            },
                                            ast_idx: 44,
                                            ident: `relative_bounding_box`,
                                            associated_item_kind: TypeItem(
                                                Memo,
                                            ),
                                            accessibility: Accessibility::PublicUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        ast_idx: 44,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::Impl(
                                                                    ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
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
                                                                ],
                                                            },
                                                            entity_path_expr_arena: Arena {
                                                                data: [
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                allow_self_type: True,
                                                                allow_self_value: False,
                                                                pattern_ty_constraints: [],
                                                            },
                                                            roots: [
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_id: ImplId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                impl_kind: ImplKind::Type {
                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                            ident: `relative_bounding_box`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                368,
                                                            ),
                                                            ident: `RelativeBoundingBox`,
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                    allow_self_type: True,
                                                    allow_self_value: True,
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
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    367,
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
                                                    369,
                                                ),
                                            ),
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::Impl(
                                                                            ImplId {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                impl_kind: ImplKind::Type {
                                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                },
                                                                                disambiguator: 0,
                                                                            },
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
                                                                        ],
                                                                    },
                                                                    entity_path_expr_arena: Arena {
                                                                        data: [
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        allow_self_type: True,
                                                                        allow_self_value: False,
                                                                        pattern_ty_constraints: [],
                                                                    },
                                                                    roots: [
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_id: ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `relative_bounding_box`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                                        368,
                                                                    ),
                                                                    ident: `RelativeBoundingBox`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                                            allow_self_type: True,
                                                            allow_self_value: True,
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
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `relative_bounding_box`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            370,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            371,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `line_segment_sketch`,
                                                            token_idx: TokenIdx(
                                                                372,
                                                            ),
                                                        },
                                                    },
                                                    Expr::Field {
                                                        owner: 1,
                                                        dot_token_idx: TokenIdx(
                                                            373,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `bounding_box`,
                                                            token_idx: TokenIdx(
                                                                374,
                                                            ),
                                                        },
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            378,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 3,
                                                        dot_token_idx: TokenIdx(
                                                            379,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `bounding_box`,
                                                            token_idx: TokenIdx(
                                                                380,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 2,
                                                        dot_token_idx: TokenIdx(
                                                            375,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `relative_bounding_box`,
                                                            token_idx: TokenIdx(
                                                                376,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            377,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            4..5,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            381,
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
                                                data: [],
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
                                                allow_self_type: True,
                                                allow_self_value: True,
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
                                    body: Ok(
                                        6,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_id: ImplId {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `line_segment`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Method(
                                TypeMethodDefn {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `line_segment`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    decl: TypeMethodDecl {
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ident: `line_segment`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `line_segment`,
                                                        ty_item_kind: Method,
                                                    },
                                                ),
                                            ),
                                            im: Impl {
                                                id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 75,
                                                body: ArenaIdxRange(
                                                    39..53,
                                                ),
                                                variant: ImplVariant::Type {
                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                },
                                            },
                                            ast_idx: 45,
                                            ident: `line_segment`,
                                            associated_item_kind: TypeItem(
                                                Method,
                                            ),
                                            accessibility: Accessibility::PublicUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `line_segment`,
                                                ty_item_kind: Method,
                                            },
                                        ),
                                        ast_idx: 45,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::Impl(
                                                                    ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
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
                                                                ],
                                                            },
                                                            entity_path_expr_arena: Arena {
                                                                data: [
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                allow_self_type: True,
                                                                allow_self_value: False,
                                                                pattern_ty_constraints: [],
                                                            },
                                                            roots: [
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_id: ImplId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                impl_kind: ImplKind::Type {
                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                            ident: `line_segment`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                                387,
                                                            ),
                                                            ident: `LineSegment`,
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                    allow_self_type: True,
                                                    allow_self_value: True,
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
                                        implicit_parameter_decl_list: Ok(
                                            None,
                                        ),
                                        parameter_decl_list: Ok(
                                            ExplicitParameterDeclList {
                                                lpar: LeftParenthesisToken(
                                                    TokenIdx(
                                                        384,
                                                    ),
                                                ),
                                                parameters: [],
                                                commas: [],
                                                decl_list_result: Ok(
                                                    (),
                                                ),
                                                rpar: Ok(
                                                    RightParenthesisToken(
                                                        TokenIdx(
                                                            385,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    386,
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
                                                    388,
                                                ),
                                            ),
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::Impl(
                                                                            ImplId {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                impl_kind: ImplKind::Type {
                                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                },
                                                                                disambiguator: 0,
                                                                            },
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
                                                                        ],
                                                                    },
                                                                    entity_path_expr_arena: Arena {
                                                                        data: [
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        allow_self_type: True,
                                                                        allow_self_value: False,
                                                                        pattern_ty_constraints: [],
                                                                    },
                                                                    roots: [
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_id: ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `line_segment`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                                        387,
                                                                    ),
                                                                    ident: `LineSegment`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                            allow_self_type: True,
                                                            allow_self_value: True,
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
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `line_segment`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        path: Some(
                                                            EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            391,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 1,
                                                        dot_token_idx: TokenIdx(
                                                            392,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                393,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 2,
                                                        dot_token_idx: TokenIdx(
                                                            394,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `firstx`,
                                                            token_idx: TokenIdx(
                                                                395,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            396,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            3..3,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            397,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 3,
                                                        dot_token_idx: TokenIdx(
                                                            398,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                399,
                                                            ),
                                                        },
                                                    },
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            405,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 5,
                                                        dot_token_idx: TokenIdx(
                                                            406,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                407,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 6,
                                                        dot_token_idx: TokenIdx(
                                                            408,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `lastx`,
                                                            token_idx: TokenIdx(
                                                                409,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            410,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            7..7,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            411,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 7,
                                                        dot_token_idx: TokenIdx(
                                                            412,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                413,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 4,
                                                        dot_token_idx: TokenIdx(
                                                            400,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `clone`,
                                                            token_idx: TokenIdx(
                                                                401,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            402,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            5..5,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            403,
                                                        ),
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 8,
                                                        dot_token_idx: TokenIdx(
                                                            414,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `clone`,
                                                            token_idx: TokenIdx(
                                                                415,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            416,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            9..9,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            417,
                                                        ),
                                                    },
                                                    Expr::ExplicitApplicationOrRitchieCall {
                                                        function: 0,
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            390,
                                                        ),
                                                        items: ArenaIdxRange(
                                                            9..11,
                                                        ),
                                                        commas: [
                                                            TokenIdx(
                                                                404,
                                                            ),
                                                        ],
                                                        rpar_token_idx: TokenIdx(
                                                            418,
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
                                                            389,
                                                        ),
                                                        ident: `LineSegment`,
                                                        entity_path: EntityPath::ModuleItem(
                                                            ModuleItemPath::Type(
                                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                            ),
                                                        ),
                                                    },
                                                ],
                                            },
                                            stmt_arena: Arena {
                                                data: [
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
                                                    data: [],
                                                },
                                                current_symbol_arena: Arena {
                                                    data: [],
                                                },
                                                allow_self_type: True,
                                                allow_self_value: True,
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
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_id: ImplId {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `start`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Method(
                                TypeMethodDefn {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `start`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    decl: TypeMethodDecl {
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ident: `start`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `start`,
                                                        ty_item_kind: Method,
                                                    },
                                                ),
                                            ),
                                            im: Impl {
                                                id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 75,
                                                body: ArenaIdxRange(
                                                    39..53,
                                                ),
                                                variant: ImplVariant::Type {
                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                },
                                            },
                                            ast_idx: 46,
                                            ident: `start`,
                                            associated_item_kind: TypeItem(
                                                Method,
                                            ),
                                            accessibility: Accessibility::PublicUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `start`,
                                                ty_item_kind: Method,
                                            },
                                        ),
                                        ast_idx: 46,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::Impl(
                                                                    ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
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
                                                                ],
                                                            },
                                                            entity_path_expr_arena: Arena {
                                                                data: [
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                allow_self_type: True,
                                                                allow_self_value: False,
                                                                pattern_ty_constraints: [],
                                                            },
                                                            roots: [
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_id: ImplId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                impl_kind: ImplKind::Type {
                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                            ident: `start`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                                424,
                                                            ),
                                                            ident: `Point2d`,
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                    allow_self_type: True,
                                                    allow_self_value: True,
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
                                        implicit_parameter_decl_list: Ok(
                                            None,
                                        ),
                                        parameter_decl_list: Ok(
                                            ExplicitParameterDeclList {
                                                lpar: LeftParenthesisToken(
                                                    TokenIdx(
                                                        421,
                                                    ),
                                                ),
                                                parameters: [],
                                                commas: [],
                                                decl_list_result: Ok(
                                                    (),
                                                ),
                                                rpar: Ok(
                                                    RightParenthesisToken(
                                                        TokenIdx(
                                                            422,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    423,
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
                                                    425,
                                                ),
                                            ),
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::Impl(
                                                                            ImplId {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                impl_kind: ImplKind::Type {
                                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                },
                                                                                disambiguator: 0,
                                                                            },
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
                                                                        ],
                                                                    },
                                                                    entity_path_expr_arena: Arena {
                                                                        data: [
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        allow_self_type: True,
                                                                        allow_self_value: False,
                                                                        pattern_ty_constraints: [],
                                                                    },
                                                                    roots: [
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_id: ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `start`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                                        424,
                                                                    ),
                                                                    ident: `Point2d`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                            allow_self_type: True,
                                                            allow_self_value: True,
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
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `start`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            426,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            427,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                428,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            429,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `firstx`,
                                                            token_idx: TokenIdx(
                                                                430,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            431,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            432,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 2,
                                                        dot_token_idx: TokenIdx(
                                                            433,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `start`,
                                                            token_idx: TokenIdx(
                                                                434,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 3,
                                                        dot_token_idx: TokenIdx(
                                                            435,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `clone`,
                                                            token_idx: TokenIdx(
                                                                436,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            437,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            4..4,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            438,
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
                                                data: [],
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
                                                allow_self_type: True,
                                                allow_self_value: True,
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
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_id: ImplId {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `end`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Method(
                                TypeMethodDefn {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `end`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    decl: TypeMethodDecl {
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ident: `end`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `end`,
                                                        ty_item_kind: Method,
                                                    },
                                                ),
                                            ),
                                            im: Impl {
                                                id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 75,
                                                body: ArenaIdxRange(
                                                    39..53,
                                                ),
                                                variant: ImplVariant::Type {
                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                },
                                            },
                                            ast_idx: 47,
                                            ident: `end`,
                                            associated_item_kind: TypeItem(
                                                Method,
                                            ),
                                            accessibility: Accessibility::PublicUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `end`,
                                                ty_item_kind: Method,
                                            },
                                        ),
                                        ast_idx: 47,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::Impl(
                                                                    ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
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
                                                                ],
                                                            },
                                                            entity_path_expr_arena: Arena {
                                                                data: [
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                allow_self_type: True,
                                                                allow_self_value: False,
                                                                pattern_ty_constraints: [],
                                                            },
                                                            roots: [
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_id: ImplId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                impl_kind: ImplKind::Type {
                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                            ident: `end`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                                444,
                                                            ),
                                                            ident: `Point2d`,
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                    allow_self_type: True,
                                                    allow_self_value: True,
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
                                        implicit_parameter_decl_list: Ok(
                                            None,
                                        ),
                                        parameter_decl_list: Ok(
                                            ExplicitParameterDeclList {
                                                lpar: LeftParenthesisToken(
                                                    TokenIdx(
                                                        441,
                                                    ),
                                                ),
                                                parameters: [],
                                                commas: [],
                                                decl_list_result: Ok(
                                                    (),
                                                ),
                                                rpar: Ok(
                                                    RightParenthesisToken(
                                                        TokenIdx(
                                                            442,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    443,
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
                                                    445,
                                                ),
                                            ),
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::Impl(
                                                                            ImplId {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                impl_kind: ImplKind::Type {
                                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                },
                                                                                disambiguator: 0,
                                                                            },
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
                                                                        ],
                                                                    },
                                                                    entity_path_expr_arena: Arena {
                                                                        data: [
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        allow_self_type: True,
                                                                        allow_self_value: False,
                                                                        pattern_ty_constraints: [],
                                                                    },
                                                                    roots: [
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_id: ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `end`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                                        444,
                                                                    ),
                                                                    ident: `Point2d`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                                            allow_self_type: True,
                                                            allow_self_value: True,
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
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `end`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            446,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            447,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                448,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            449,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `lastx`,
                                                            token_idx: TokenIdx(
                                                                450,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            451,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            452,
                                                        ),
                                                    },
                                                    Expr::Field {
                                                        owner: 2,
                                                        dot_token_idx: TokenIdx(
                                                            453,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `end`,
                                                            token_idx: TokenIdx(
                                                                454,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 3,
                                                        dot_token_idx: TokenIdx(
                                                            455,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `clone`,
                                                            token_idx: TokenIdx(
                                                                456,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            457,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            4..4,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            458,
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
                                                data: [],
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
                                                allow_self_type: True,
                                                allow_self_value: True,
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
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_id: ImplId {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `displacement`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Method(
                                TypeMethodDefn {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `displacement`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    decl: TypeMethodDecl {
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ident: `displacement`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `displacement`,
                                                        ty_item_kind: Method,
                                                    },
                                                ),
                                            ),
                                            im: Impl {
                                                id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 75,
                                                body: ArenaIdxRange(
                                                    39..53,
                                                ),
                                                variant: ImplVariant::Type {
                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                },
                                            },
                                            ast_idx: 48,
                                            ident: `displacement`,
                                            associated_item_kind: TypeItem(
                                                Method,
                                            ),
                                            accessibility: Accessibility::PublicUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `displacement`,
                                                ty_item_kind: Method,
                                            },
                                        ),
                                        ast_idx: 48,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::Impl(
                                                                    ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
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
                                                                ],
                                                            },
                                                            entity_path_expr_arena: Arena {
                                                                data: [
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                allow_self_type: True,
                                                                allow_self_value: False,
                                                                pattern_ty_constraints: [],
                                                            },
                                                            roots: [
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_id: ImplId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                impl_kind: ImplKind::Type {
                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                            ident: `displacement`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                464,
                                                            ),
                                                            ident: `Vector2d`,
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                    allow_self_type: True,
                                                    allow_self_value: True,
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
                                        implicit_parameter_decl_list: Ok(
                                            None,
                                        ),
                                        parameter_decl_list: Ok(
                                            ExplicitParameterDeclList {
                                                lpar: LeftParenthesisToken(
                                                    TokenIdx(
                                                        461,
                                                    ),
                                                ),
                                                parameters: [],
                                                commas: [],
                                                decl_list_result: Ok(
                                                    (),
                                                ),
                                                rpar: Ok(
                                                    RightParenthesisToken(
                                                        TokenIdx(
                                                            462,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    463,
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
                                                    465,
                                                ),
                                            ),
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::Impl(
                                                                            ImplId {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                impl_kind: ImplKind::Type {
                                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                },
                                                                                disambiguator: 0,
                                                                            },
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
                                                                        ],
                                                                    },
                                                                    entity_path_expr_arena: Arena {
                                                                        data: [
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        allow_self_type: True,
                                                                        allow_self_value: False,
                                                                        pattern_ty_constraints: [],
                                                                    },
                                                                    roots: [
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_id: ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `displacement`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                        464,
                                                                    ),
                                                                    ident: `Vector2d`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                            allow_self_type: True,
                                                            allow_self_value: True,
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
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `displacement`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            466,
                                                        ),
                                                    ),
                                                    Expr::MethodCall {
                                                        self_argument: 0,
                                                        dot_token_idx: TokenIdx(
                                                            467,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `line_segment`,
                                                            token_idx: TokenIdx(
                                                                468,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            469,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            1..1,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            470,
                                                        ),
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            471,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                472,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            473,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            474,
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
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Eval {
                                                        expr_idx: 2,
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
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr: 3,
                                                },
                                            ],
                                        },
                                    },
                                    body: Ok(
                                        3,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_id: ImplId {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `start_tangent`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Method(
                                TypeMethodDefn {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `start_tangent`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    decl: TypeMethodDecl {
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ident: `start_tangent`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `start_tangent`,
                                                        ty_item_kind: Method,
                                                    },
                                                ),
                                            ),
                                            im: Impl {
                                                id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 75,
                                                body: ArenaIdxRange(
                                                    39..53,
                                                ),
                                                variant: ImplVariant::Type {
                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                },
                                            },
                                            ast_idx: 49,
                                            ident: `start_tangent`,
                                            associated_item_kind: TypeItem(
                                                Method,
                                            ),
                                            accessibility: Accessibility::PublicUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `start_tangent`,
                                                ty_item_kind: Method,
                                            },
                                        ),
                                        ast_idx: 49,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::Impl(
                                                                    ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
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
                                                                ],
                                                            },
                                                            entity_path_expr_arena: Arena {
                                                                data: [
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                allow_self_type: True,
                                                                allow_self_value: False,
                                                                pattern_ty_constraints: [],
                                                            },
                                                            roots: [
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_id: ImplId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                impl_kind: ImplKind::Type {
                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                            ident: `start_tangent`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                480,
                                                            ),
                                                            ident: `Vector2d`,
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                    allow_self_type: True,
                                                    allow_self_value: True,
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
                                        implicit_parameter_decl_list: Ok(
                                            None,
                                        ),
                                        parameter_decl_list: Ok(
                                            ExplicitParameterDeclList {
                                                lpar: LeftParenthesisToken(
                                                    TokenIdx(
                                                        477,
                                                    ),
                                                ),
                                                parameters: [],
                                                commas: [],
                                                decl_list_result: Ok(
                                                    (),
                                                ),
                                                rpar: Ok(
                                                    RightParenthesisToken(
                                                        TokenIdx(
                                                            478,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    479,
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
                                                    481,
                                                ),
                                            ),
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::Impl(
                                                                            ImplId {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                impl_kind: ImplKind::Type {
                                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                },
                                                                                disambiguator: 0,
                                                                            },
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
                                                                        ],
                                                                    },
                                                                    entity_path_expr_arena: Arena {
                                                                        data: [
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        allow_self_type: True,
                                                                        allow_self_value: False,
                                                                        pattern_ty_constraints: [],
                                                                    },
                                                                    roots: [
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_id: ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `start_tangent`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                        480,
                                                                    ),
                                                                    ident: `Vector2d`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                            allow_self_type: True,
                                                            allow_self_value: True,
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
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `start_tangent`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            482,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            483,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                484,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            485,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `firstx`,
                                                            token_idx: TokenIdx(
                                                                486,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            487,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            488,
                                                        ),
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 2,
                                                        dot_token_idx: TokenIdx(
                                                            489,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                490,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            491,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            3..3,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            492,
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
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Eval {
                                                        expr_idx: 3,
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
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr: 4,
                                                },
                                            ],
                                        },
                                    },
                                    body: Ok(
                                        4,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
            (
                DefnRegionPath::AssociatedItem(
                    AssociatedItemId {
                        impl_id: ImplId {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            impl_kind: ImplKind::Type {
                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            },
                            disambiguator: 0,
                        },
                        ident: `end_tangent`,
                    },
                ),
                Ok(
                    Defn::AssociatedItem(
                        AssociatedItemDefn::TypeItem(
                            TypeItemDefn::Method(
                                TypeMethodDefn {
                                    path: Some(
                                        TypeItemPath {
                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ident: `end_tangent`,
                                            ty_item_kind: Method,
                                        },
                                    ),
                                    decl: TypeMethodDecl {
                                        associated_item: AssociatedItem {
                                            id: AssociatedItemId {
                                                impl_id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ident: `end_tangent`,
                                            },
                                            path: Some(
                                                AssociatedItemPath::TypeItem(
                                                    TypeItemPath {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ident: `end_tangent`,
                                                        ty_item_kind: Method,
                                                    },
                                                ),
                                            ),
                                            im: Impl {
                                                id: ImplId {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    impl_kind: ImplKind::Type {
                                                        ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    disambiguator: 0,
                                                },
                                                ast_idx: 75,
                                                body: ArenaIdxRange(
                                                    39..53,
                                                ),
                                                variant: ImplVariant::Type {
                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                },
                                            },
                                            ast_idx: 50,
                                            ident: `end_tangent`,
                                            associated_item_kind: TypeItem(
                                                Method,
                                            ),
                                            accessibility: Accessibility::PublicUnder(
                                                `mnist_classifier::line_segment_sketch::concave_component`,
                                            ),
                                            is_generic: false,
                                        },
                                        path: Some(
                                            TypeItemPath {
                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ident: `end_tangent`,
                                                ty_item_kind: Method,
                                            },
                                        ),
                                        ast_idx: 50,
                                        expr_region: ExprRegion {
                                            data: ExprRegionData {
                                                parent: Some(
                                                    ExprRegion {
                                                        data: ExprRegionData {
                                                            parent: None,
                                                            path: RegionPath::Decl(
                                                                DeclRegionPath::Impl(
                                                                    ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
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
                                                                ],
                                                            },
                                                            entity_path_expr_arena: Arena {
                                                                data: [
                                                                    EntityPathExpr::Root {
                                                                        token_idx: TokenIdx(
                                                                            50,
                                                                        ),
                                                                        ident: `ConcaveComponent`,
                                                                        entity_path: EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                allow_self_type: True,
                                                                allow_self_value: False,
                                                                pattern_ty_constraints: [],
                                                            },
                                                            roots: [
                                                                ExprRoot {
                                                                    kind: SelfType,
                                                                    expr: 0,
                                                                },
                                                            ],
                                                        },
                                                    },
                                                ),
                                                path: RegionPath::Decl(
                                                    DeclRegionPath::AssociatedItem(
                                                        AssociatedItemId {
                                                            impl_id: ImplId {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                impl_kind: ImplKind::Type {
                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                },
                                                                disambiguator: 0,
                                                            },
                                                            ident: `end_tangent`,
                                                        },
                                                    ),
                                                ),
                                                expr_arena: Arena {
                                                    data: [
                                                        Expr::EntityPath {
                                                            entity_path_expr: 0,
                                                            path: Some(
                                                                EntityPath::ModuleItem(
                                                                    ModuleItemPath::Type(
                                                                        TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                498,
                                                            ),
                                                            ident: `Vector2d`,
                                                            entity_path: EntityPath::ModuleItem(
                                                                ModuleItemPath::Type(
                                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                    allow_self_type: True,
                                                    allow_self_value: True,
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
                                        implicit_parameter_decl_list: Ok(
                                            None,
                                        ),
                                        parameter_decl_list: Ok(
                                            ExplicitParameterDeclList {
                                                lpar: LeftParenthesisToken(
                                                    TokenIdx(
                                                        495,
                                                    ),
                                                ),
                                                parameters: [],
                                                commas: [],
                                                decl_list_result: Ok(
                                                    (),
                                                ),
                                                rpar: Ok(
                                                    RightParenthesisToken(
                                                        TokenIdx(
                                                            496,
                                                        ),
                                                    ),
                                                ),
                                            },
                                        ),
                                        curry_token: Ok(
                                            CurryToken(
                                                TokenIdx(
                                                    497,
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
                                                    499,
                                                ),
                                            ),
                                        ),
                                    },
                                    expr_region: ExprRegion {
                                        data: ExprRegionData {
                                            parent: Some(
                                                ExprRegion {
                                                    data: ExprRegionData {
                                                        parent: Some(
                                                            ExprRegion {
                                                                data: ExprRegionData {
                                                                    parent: None,
                                                                    path: RegionPath::Decl(
                                                                        DeclRegionPath::Impl(
                                                                            ImplId {
                                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                                impl_kind: ImplKind::Type {
                                                                                    ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                                },
                                                                                disambiguator: 0,
                                                                            },
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
                                                                        ],
                                                                    },
                                                                    entity_path_expr_arena: Arena {
                                                                        data: [
                                                                            EntityPathExpr::Root {
                                                                                token_idx: TokenIdx(
                                                                                    50,
                                                                                ),
                                                                                ident: `ConcaveComponent`,
                                                                                entity_path: EntityPath::ModuleItem(
                                                                                    ModuleItemPath::Type(
                                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                                        allow_self_type: True,
                                                                        allow_self_value: False,
                                                                        pattern_ty_constraints: [],
                                                                    },
                                                                    roots: [
                                                                        ExprRoot {
                                                                            kind: SelfType,
                                                                            expr: 0,
                                                                        },
                                                                    ],
                                                                },
                                                            },
                                                        ),
                                                        path: RegionPath::Decl(
                                                            DeclRegionPath::AssociatedItem(
                                                                AssociatedItemId {
                                                                    impl_id: ImplId {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        impl_kind: ImplKind::Type {
                                                                            ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        },
                                                                        disambiguator: 0,
                                                                    },
                                                                    ident: `end_tangent`,
                                                                },
                                                            ),
                                                        ),
                                                        expr_arena: Arena {
                                                            data: [
                                                                Expr::EntityPath {
                                                                    entity_path_expr: 0,
                                                                    path: Some(
                                                                        EntityPath::ModuleItem(
                                                                            ModuleItemPath::Type(
                                                                                TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                                        498,
                                                                    ),
                                                                    ident: `Vector2d`,
                                                                    entity_path: EntityPath::ModuleItem(
                                                                        ModuleItemPath::Type(
                                                                            TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                            allow_self_type: True,
                                                            allow_self_value: True,
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
                                            ),
                                            path: RegionPath::Defn(
                                                DefnRegionPath::AssociatedItem(
                                                    AssociatedItemId {
                                                        impl_id: ImplId {
                                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                            impl_kind: ImplKind::Type {
                                                                ty: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            },
                                                            disambiguator: 0,
                                                        },
                                                        ident: `end_tangent`,
                                                    },
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::SelfValue(
                                                        TokenIdx(
                                                            500,
                                                        ),
                                                    ),
                                                    Expr::Field {
                                                        owner: 0,
                                                        dot_token_idx: TokenIdx(
                                                            501,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `strokes`,
                                                            token_idx: TokenIdx(
                                                                502,
                                                            ),
                                                        },
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 1,
                                                        dot_token_idx: TokenIdx(
                                                            503,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `lastx`,
                                                            token_idx: TokenIdx(
                                                                504,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            505,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            2..2,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            506,
                                                        ),
                                                    },
                                                    Expr::MethodCall {
                                                        self_argument: 2,
                                                        dot_token_idx: TokenIdx(
                                                            507,
                                                        ),
                                                        ident_token: IdentifierToken {
                                                            ident: `displacement`,
                                                            token_idx: TokenIdx(
                                                                508,
                                                            ),
                                                        },
                                                        implicit_arguments: None,
                                                        lpar_token_idx: TokenIdx(
                                                            509,
                                                        ),
                                                        nonself_arguments: ArenaIdxRange(
                                                            3..3,
                                                        ),
                                                        rpar_token_idx: TokenIdx(
                                                            510,
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
                                                data: [],
                                            },
                                            stmt_arena: Arena {
                                                data: [
                                                    Stmt::Eval {
                                                        expr_idx: 3,
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
                                                allow_self_type: True,
                                                allow_self_value: True,
                                                pattern_ty_constraints: [],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: BlockExpr,
                                                    expr: 4,
                                                },
                                            ],
                                        },
                                    },
                                    body: Ok(
                                        4,
                                    ),
                                },
                            ),
                        ),
                    ),
                ),
            ),
        ],
    },
)