Ok(
    DefnSheet {
        defns: [
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                            ast_idx: 23,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        33,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 2,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        37,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    34,
                                                ),
                                                items: ArenaIdxRange(
                                                    0..0,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    35,
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                ),
                                            },
                                            Expr::Application {
                                                function: 0,
                                                argument: 1,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    36,
                                                ),
                                                ident: `ConnectedComponent`,
                                                entity_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                            kind: OutputType,
                                            expr: 2,
                                        },
                                    ],
                                },
                            },
                        },
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
                                            ),
                                        },
                                        Expr::Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    40,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 437,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Expr::ApplicationOrFunctionCall {
                                            function: 0,
                                            lpar_token_idx: TokenIdx(
                                                39,
                                            ),
                                            argument: 1,
                                            rpar_token_idx: TokenIdx(
                                                41,
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
                                                38,
                                            ),
                                            ident: `find_connected_components`,
                                            entity_path: FormPath(`mnist_classifier::connected_component::find_connected_components`, `Function`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr: 2,
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
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                            ast_idx: 24,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        45,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 0,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        47,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    46,
                                                ),
                                                ident: `ConnectedComponent`,
                                                entity_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
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
                                            kind: OutputType,
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
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::Literal(
                                            TokenIdx(
                                                52,
                                            ),
                                        ),
                                        Expr::Literal(
                                            TokenIdx(
                                                57,
                                            ),
                                        ),
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                            ),
                                        },
                                        Expr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                59,
                                            ),
                                            ident: `i`,
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                3,
                                            ),
                                        },
                                        Expr::MethodCall {
                                            this_expr: 2,
                                            dot_token_idx: TokenIdx(
                                                62,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    63,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                64,
                                            ),
                                            arguments: ArenaIdxRange(
                                                3..3,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                65,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 3,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                60,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                72,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                6,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                71,
                                            ),
                                            items: ArenaIdxRange(
                                                7..8,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                73,
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 8,
                                            dot_token_idx: TokenIdx(
                                                74,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `row_span_sum`,
                                                token_idx: TokenIdx(
                                                    75,
                                                ),
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `row_span_sum`,
                                            token_idx: TokenIdx(
                                                77,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `max_row_span_sum`,
                                            token_idx: TokenIdx(
                                                79,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                3,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 10,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                78,
                                            ),
                                            ropd: 11,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `max_row_span_sum`,
                                            token_idx: TokenIdx(
                                                81,
                                            ),
                                            current_symbol_idx: 2,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                3,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `row_span_sum`,
                                            token_idx: TokenIdx(
                                                83,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 13,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                82,
                                            ),
                                            ropd: 14,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i0`,
                                            token_idx: TokenIdx(
                                                84,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                86,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 1,
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 16,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                85,
                                            ),
                                            ropd: 17,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i0`,
                                            token_idx: TokenIdx(
                                                90,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                19,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                89,
                                            ),
                                            items: ArenaIdxRange(
                                                20..21,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                91,
                                            ),
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                4..8,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                61,
                                            ),
                                            ident: `connected_components`,
                                            entity_path: FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                70,
                                            ),
                                            ident: `connected_components`,
                                            entity_path: FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                88,
                                            ),
                                            ident: `connected_components`,
                                            entity_path: FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr: 15,
                                        },
                                        Stmt::Eval {
                                            expr: 18,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    67,
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
                                            assign_token: Ok(
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        69,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                9,
                                            ),
                                        },
                                        Stmt::IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        76,
                                                    ),
                                                },
                                                condition: Ok(
                                                    12,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            80,
                                                        ),
                                                    },
                                                ),
                                                block: Ok(
                                                    ArenaIdxRange(
                                                        0..2,
                                                    ),
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    48,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        51,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                0,
                                            ),
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    53,
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
                                            assign_token: Ok(
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        56,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                1,
                                            ),
                                        },
                                        Stmt::ForBetween {
                                            for_token: ForToken {
                                                token_idx: TokenIdx(
                                                    58,
                                                ),
                                            },
                                            particulars: ForBetweenParticulars {
                                                frame_var_token_idx: TokenIdx(
                                                    59,
                                                ),
                                                frame_var_expr_idx: 3,
                                                frame_var_ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 144,
                                                        },
                                                    ),
                                                ),
                                                range: ForBetweenRange {
                                                    initial_boundary: LoopBoundary {
                                                        bound_expr: None,
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            4,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            eol_colon: Ok(
                                                EolColonToken {
                                                    token_idx: TokenIdx(
                                                        66,
                                                    ),
                                                },
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
                                                    87,
                                                ),
                                            },
                                            result: Ok(
                                                21,
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `i0`,
                                                    token_idx: TokenIdx(
                                                        50,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `max_row_span_sum`,
                                                    token_idx: TokenIdx(
                                                        55,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `row_span_sum`,
                                                    token_idx: TokenIdx(
                                                        68,
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
                                                            value: 241,
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
                                                            value: 438,
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
                                                            value: 152,
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
                                                ident: `i0`,
                                                access_start: TokenIdx(
                                                    51,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            92,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `max_row_span_sum`,
                                                access_start: TokenIdx(
                                                    56,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            92,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 1,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `i`,
                                                access_start: TokenIdx(
                                                    67,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            87,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::FrameVariable(
                                                    3,
                                                ),
                                            },
                                            CurrentSymbol {
                                                ident: `row_span_sum`,
                                                access_start: TokenIdx(
                                                    69,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            87,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        FrameVariable,
                                    ],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr: 22,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            22,
                        ),
                    },
                ),
            ),
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                            ast_idx: 25,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        95,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 0,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        97,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    96,
                                                ),
                                                ident: `f32`,
                                                entity_path: TypePath(`core::num::f32`, `Alien`),
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
                                            kind: OutputType,
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
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::Literal(
                                            TokenIdx(
                                                102,
                                            ),
                                        ),
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                            ),
                                        },
                                        Expr::FrameVarDecl {
                                            token_idx: TokenIdx(
                                                104,
                                            ),
                                            ident: `i`,
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                2,
                                            ),
                                        },
                                        Expr::MethodCall {
                                            this_expr: 1,
                                            dot_token_idx: TokenIdx(
                                                107,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    108,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                109,
                                            ),
                                            arguments: ArenaIdxRange(
                                                2..2,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                110,
                                            ),
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 2,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                105,
                                            ),
                                            ropd: 3,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `i`,
                                            token_idx: TokenIdx(
                                                116,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::NewBoxList {
                                            caller: Some(
                                                5,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                115,
                                            ),
                                            items: ArenaIdxRange(
                                                6..7,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                117,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `sum`,
                                            token_idx: TokenIdx(
                                                112,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                2,
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 7,
                                            dot_token_idx: TokenIdx(
                                                118,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `row_span_sum`,
                                                token_idx: TokenIdx(
                                                    119,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 8,
                                            opr: Assign(
                                                Some(
                                                    Add,
                                                ),
                                            ),
                                            opr_token_idx: TokenIdx(
                                                113,
                                            ),
                                            ropd: 9,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `sum`,
                                            token_idx: TokenIdx(
                                                121,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::FrameVariable(
                                                2,
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 11,
                                            dot_token_idx: TokenIdx(
                                                124,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `row_span_sum`,
                                                token_idx: TokenIdx(
                                                    125,
                                                ),
                                            },
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 12,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                122,
                                            ),
                                            ropd: 13,
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                1..4,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                106,
                                            ),
                                            ident: `connected_components`,
                                            entity_path: FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                114,
                                            ),
                                            ident: `connected_components`,
                                            entity_path: FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                123,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr: 10,
                                        },
                                        Stmt::Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    98,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        101,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                0,
                                            ),
                                        },
                                        Stmt::ForBetween {
                                            for_token: ForToken {
                                                token_idx: TokenIdx(
                                                    103,
                                                ),
                                            },
                                            particulars: ForBetweenParticulars {
                                                frame_var_token_idx: TokenIdx(
                                                    104,
                                                ),
                                                frame_var_expr_idx: 2,
                                                frame_var_ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 144,
                                                        },
                                                    ),
                                                ),
                                                range: ForBetweenRange {
                                                    initial_boundary: LoopBoundary {
                                                        bound_expr: None,
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: LoopBoundary {
                                                        bound_expr: Some(
                                                            3,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            eol_colon: Ok(
                                                EolColonToken {
                                                    token_idx: TokenIdx(
                                                        111,
                                                    ),
                                                },
                                            ),
                                            block: Ok(
                                                ArenaIdxRange(
                                                    0..1,
                                                ),
                                            ),
                                        },
                                        Stmt::Return {
                                            return_token: ReturnToken {
                                                token_idx: TokenIdx(
                                                    120,
                                                ),
                                            },
                                            result: Ok(
                                                14,
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `sum`,
                                                    token_idx: TokenIdx(
                                                        100,
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
                                                            value: 439,
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
                                                ident: `sum`,
                                                access_start: TokenIdx(
                                                    101,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            126,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `i`,
                                                access_start: TokenIdx(
                                                    112,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            120,
                                                        ),
                                                    ),
                                                ),
                                                variant: CurrentSymbolVariant::FrameVariable(
                                                    2,
                                                ),
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    pattern_ty_constraints: [
                                        FrameVariable,
                                    ],
                                },
                                roots: [
                                    ExprRoot {
                                        kind: BlockExpr,
                                        expr: 15,
                                    },
                                ],
                            },
                        },
                        body: Ok(
                            15,
                        ),
                    },
                ),
            ),
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::major::major_raw_contours`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::major::major_raw_contours`, `Feature`),
                            ast_idx: 26,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        129,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 2,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        133,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::major::major_raw_contours`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    130,
                                                ),
                                                items: ArenaIdxRange(
                                                    0..0,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    131,
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                ),
                                            },
                                            Expr::Application {
                                                function: 0,
                                                argument: 1,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    132,
                                                ),
                                                ident: `RawContour`,
                                                entity_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                            kind: OutputType,
                                            expr: 2,
                                        },
                                    ],
                                },
                            },
                        },
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::major::major_raw_contours`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 0,
                                            dot_token_idx: TokenIdx(
                                                135,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `raw_contours`,
                                                token_idx: TokenIdx(
                                                    136,
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
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                134,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr: 1,
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
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::major::major_raw_contour`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::major::major_raw_contour`, `Feature`),
                            ast_idx: 27,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        140,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 0,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        142,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::major::major_raw_contour`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    141,
                                                ),
                                                ident: `RawContour`,
                                                entity_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                            kind: OutputType,
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
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::major::major_raw_contour`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 0,
                                            dot_token_idx: TokenIdx(
                                                144,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `raw_contours`,
                                                token_idx: TokenIdx(
                                                    145,
                                                ),
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                147,
                                            ),
                                        ),
                                        Expr::NewBoxList {
                                            caller: Some(
                                                1,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                146,
                                            ),
                                            items: ArenaIdxRange(
                                                2..3,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                148,
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
                                                143,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr: 3,
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
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                            ast_idx: 28,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        152,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 0,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        154,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    153,
                                                ),
                                                ident: `LineSegmentSketch`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                            kind: OutputType,
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
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_raw_contour`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 0,
                                            dot_token_idx: TokenIdx(
                                                156,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `line_segment_sketch`,
                                                token_idx: TokenIdx(
                                                    157,
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
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                155,
                                            ),
                                            ident: `major_raw_contour`,
                                            entity_path: FormPath(`mnist_classifier::major::major_raw_contour`, `Feature`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr: 1,
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
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                            ast_idx: 29,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        161,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 2,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        165,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::NewBoxList {
                                                caller: None,
                                                lbox_token_idx: TokenIdx(
                                                    162,
                                                ),
                                                items: ArenaIdxRange(
                                                    0..0,
                                                ),
                                                rbox_token_idx: TokenIdx(
                                                    163,
                                                ),
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            },
                                            Expr::Application {
                                                function: 0,
                                                argument: 1,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    164,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                            kind: OutputType,
                                            expr: 2,
                                        },
                                    ],
                                },
                            },
                        },
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: RegionPath::Defn(
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            this_expr: 0,
                                            dot_token_idx: TokenIdx(
                                                167,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `concave_components`,
                                                token_idx: TokenIdx(
                                                    168,
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
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                166,
                                            ),
                                            ident: `major_line_segment_sketch`,
                                            entity_path: FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Eval {
                                            expr: 1,
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
        ],
    },
)