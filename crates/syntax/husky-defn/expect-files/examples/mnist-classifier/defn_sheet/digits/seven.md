Ok(
    DefnSheet {
        defns: [
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Feature`),
                            ast_idx: 47,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        62,
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
                                        64,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Err(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        63,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 346,
                                                            },
                                                        ),
                                                    ),
                                                },
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
                                        ty_constraints: [],
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
                                path: Defn(
                                    Entity(
                                        FormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Function`),
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                            ),
                                        },
                                        NewBoxList {
                                            caller: None,
                                            lbox_token_idx: TokenIdx(
                                                69,
                                            ),
                                            items: ArenaIdxRange(
                                                1..2,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                71,
                                            ),
                                        },
                                        FunctionCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                66,
                                            ),
                                            arguments: ArenaIdxRange(
                                                2..4,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                72,
                                            ),
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        Root {
                                            token_idx: TokenIdx(
                                                65,
                                            ),
                                            ident: `fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                67,
                                            ),
                                            ident: `major_concave_components`,
                                            entity_path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                70,
                                            ),
                                            ident: `simple_leftdown_pattern`,
                                            entity_path: FormPath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Function`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 4,
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
                                    ty_constraints: [],
                                },
                                roots: [],
                            },
                        },
                        body: Ok(
                            5,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Function`),
                            ast_idx: 48,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            },
                                            PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    78,
                                                ),
                                                opd: 0,
                                            },
                                            EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                            PrefixOpn {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    82,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    79,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    83,
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
                                            data: [
                                                Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            76,
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
                                                                value: 179,
                                                            },
                                                        ),
                                                    ),
                                                    0,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                Atom(
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
                                                    ident: `cc`,
                                                    access_start: TokenIdx(
                                                        77,
                                                    ),
                                                    access_end: None,
                                                    variant: Parameter {
                                                        pattern_symbol: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        ty_constraints: [
                                            RegularParameter {
                                                pattern: 0,
                                                ty: 1,
                                            },
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: OutputType,
                                            expr: 3,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        75,
                                    ),
                                },
                                parameters: [
                                    RegularParameterDeclPattern {
                                        pattern: 0,
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                77,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        80,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        81,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 3,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        84,
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
                                            path: Decl(
                                                Entity(
                                                    FormPath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    },
                                                    PrefixOpn {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            78,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`core::num::f32`, `Alien`),
                                                        ),
                                                    },
                                                    PrefixOpn {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            82,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    Root {
                                                        token_idx: TokenIdx(
                                                            79,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    Root {
                                                        token_idx: TokenIdx(
                                                            83,
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
                                                    data: [
                                                        Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    76,
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
                                                                        value: 179,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        Atom(
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
                                                            ident: `cc`,
                                                            access_start: TokenIdx(
                                                                77,
                                                            ),
                                                            access_end: None,
                                                            variant: Parameter {
                                                                pattern_symbol: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 1,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: OutputType,
                                                    expr: 3,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: Defn(
                                    Entity(
                                        FormPath(`mnist_classifier::digits::seven::simple_leftdown_pattern`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                88,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: Parameter,
                                        },
                                        MethodCall {
                                            this_expr: 0,
                                            dot_token_idx: TokenIdx(
                                                89,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    90,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                91,
                                            ),
                                            arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                92,
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                94,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Field {
                                            this_expr: 2,
                                            dot_token_idx: TokenIdx(
                                                95,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    96,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                98,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 3,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                97,
                                            ),
                                            ropd: 4,
                                        },
                                        CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                100,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Field {
                                            this_expr: 6,
                                            dot_token_idx: TokenIdx(
                                                101,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    102,
                                                ),
                                            },
                                        },
                                        PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                99,
                                            ),
                                            opd: 7,
                                        },
                                        Block {
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
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    85,
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
                                                        87,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                1,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    93,
                                                ),
                                            },
                                            condition: Ok(
                                                5,
                                            ),
                                        },
                                        Eval {
                                            expr: 8,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        86,
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
                                                            value: 299,
                                                        },
                                                    ),
                                                ),
                                                0,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            Atom(
                                                0,
                                            ),
                                        ],
                                    },
                                },
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [
                                            InheritedSymbol {
                                                ident: `cc`,
                                                kind: Parameter,
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: `dp`,
                                                access_start: TokenIdx(
                                                    87,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            103,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 0,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    ty_constraints: [],
                                },
                                roots: [],
                            },
                        },
                        body: Ok(
                            9,
                        ),
                    },
                ),
            ),
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::seven::special_seven_match`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::seven::special_seven_match`, `Feature`),
                            ast_idx: 49,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        105,
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
                                        107,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::digits::seven::special_seven_match`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Err(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        106,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 346,
                                                            },
                                                        ),
                                                    ),
                                                },
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
                                        ty_constraints: [],
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
                                path: Defn(
                                    Entity(
                                        FormPath(`mnist_classifier::digits::seven::special_seven_match`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Function`),
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 3,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Function`),
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                            ),
                                        },
                                        NewBoxList {
                                            caller: None,
                                            lbox_token_idx: TokenIdx(
                                                112,
                                            ),
                                            items: ArenaIdxRange(
                                                1..3,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                116,
                                            ),
                                        },
                                        FunctionCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                109,
                                            ),
                                            arguments: ArenaIdxRange(
                                                3..5,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                117,
                                            ),
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        Root {
                                            token_idx: TokenIdx(
                                                108,
                                            ),
                                            ident: `fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                110,
                                            ),
                                            ident: `major_concave_components`,
                                            entity_path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                113,
                                            ),
                                            ident: `leftupcc_pattern`,
                                            entity_path: FormPath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Function`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                115,
                                            ),
                                            ident: `leftdowncc_pattern`,
                                            entity_path: FormPath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Function`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 5,
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
                                    ty_constraints: [],
                                },
                                roots: [],
                            },
                        },
                        body: Ok(
                            6,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Function`),
                            ast_idx: 50,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            },
                                            PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    123,
                                                ),
                                                opd: 0,
                                            },
                                            EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                            PrefixOpn {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    127,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    124,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    128,
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
                                            data: [
                                                Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            121,
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
                                                                value: 179,
                                                            },
                                                        ),
                                                    ),
                                                    0,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                Atom(
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
                                                    ident: `cc`,
                                                    access_start: TokenIdx(
                                                        122,
                                                    ),
                                                    access_end: None,
                                                    variant: Parameter {
                                                        pattern_symbol: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        ty_constraints: [
                                            RegularParameter {
                                                pattern: 0,
                                                ty: 1,
                                            },
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: OutputType,
                                            expr: 3,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        120,
                                    ),
                                },
                                parameters: [
                                    RegularParameterDeclPattern {
                                        pattern: 0,
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                122,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        125,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        126,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 3,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        129,
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
                                            path: Decl(
                                                Entity(
                                                    FormPath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    },
                                                    PrefixOpn {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            123,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`core::num::f32`, `Alien`),
                                                        ),
                                                    },
                                                    PrefixOpn {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            127,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    Root {
                                                        token_idx: TokenIdx(
                                                            124,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    Root {
                                                        token_idx: TokenIdx(
                                                            128,
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
                                                    data: [
                                                        Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    121,
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
                                                                        value: 179,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        Atom(
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
                                                            ident: `cc`,
                                                            access_start: TokenIdx(
                                                                122,
                                                            ),
                                                            access_end: None,
                                                            variant: Parameter {
                                                                pattern_symbol: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 1,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: OutputType,
                                                    expr: 3,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: Defn(
                                    Entity(
                                        FormPath(`mnist_classifier::digits::seven::leftupcc_pattern`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                133,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: Parameter,
                                        },
                                        MethodCall {
                                            this_expr: 0,
                                            dot_token_idx: TokenIdx(
                                                134,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    135,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                136,
                                            ),
                                            arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                137,
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                139,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Field {
                                            this_expr: 2,
                                            dot_token_idx: TokenIdx(
                                                140,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    141,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                143,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 3,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                142,
                                            ),
                                            ropd: 4,
                                        },
                                        InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                145,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: Parameter,
                                        },
                                        Field {
                                            this_expr: 6,
                                            dot_token_idx: TokenIdx(
                                                146,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    147,
                                                ),
                                            },
                                        },
                                        MethodCall {
                                            this_expr: 7,
                                            dot_token_idx: TokenIdx(
                                                148,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `ymax`,
                                                token_idx: TokenIdx(
                                                    149,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                150,
                                            ),
                                            arguments: ArenaIdxRange(
                                                8..8,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                151,
                                            ),
                                        },
                                        Literal(
                                            TokenIdx(
                                                153,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 8,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                152,
                                            ),
                                            ropd: 9,
                                        },
                                        InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                154,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: Parameter,
                                        },
                                        MethodCall {
                                            this_expr: 11,
                                            dot_token_idx: TokenIdx(
                                                155,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `end`,
                                                token_idx: TokenIdx(
                                                    156,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                157,
                                            ),
                                            arguments: ArenaIdxRange(
                                                12..12,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                158,
                                            ),
                                        },
                                        Field {
                                            this_expr: 12,
                                            dot_token_idx: TokenIdx(
                                                159,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    160,
                                                ),
                                            },
                                        },
                                        Block {
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
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    130,
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
                                                        132,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                1,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    138,
                                                ),
                                            },
                                            condition: Ok(
                                                5,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    144,
                                                ),
                                            },
                                            condition: Ok(
                                                10,
                                            ),
                                        },
                                        Eval {
                                            expr: 13,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        131,
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
                                                            value: 299,
                                                        },
                                                    ),
                                                ),
                                                0,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            Atom(
                                                0,
                                            ),
                                        ],
                                    },
                                },
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [
                                            InheritedSymbol {
                                                ident: `cc`,
                                                kind: Parameter,
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: `dp`,
                                                access_start: TokenIdx(
                                                    132,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            161,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 0,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    ty_constraints: [],
                                },
                                roots: [],
                            },
                        },
                        body: Ok(
                            14,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Function`),
                            ast_idx: 51,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            },
                                            PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    166,
                                                ),
                                                opd: 0,
                                            },
                                            EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                            PrefixOpn {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    170,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    167,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    171,
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
                                            data: [
                                                Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `cc`,
                                                        token_idx: TokenIdx(
                                                            164,
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
                                                                value: 179,
                                                            },
                                                        ),
                                                    ),
                                                    0,
                                                ),
                                            ],
                                        ],
                                        pattern_symbol_arena: Arena {
                                            data: [
                                                Atom(
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
                                                    ident: `cc`,
                                                    access_start: TokenIdx(
                                                        165,
                                                    ),
                                                    access_end: None,
                                                    variant: Parameter {
                                                        pattern_symbol: 0,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        ty_constraints: [
                                            RegularParameter {
                                                pattern: 0,
                                                ty: 1,
                                            },
                                        ],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: OutputType,
                                            expr: 3,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        163,
                                    ),
                                },
                                parameters: [
                                    RegularParameterDeclPattern {
                                        pattern: 0,
                                        variables: ArenaIdxRange(
                                            0..1,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                165,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        168,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        169,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 3,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        172,
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
                                            path: Decl(
                                                Entity(
                                                    FormPath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    },
                                                    PrefixOpn {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            166,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`core::num::f32`, `Alien`),
                                                        ),
                                                    },
                                                    PrefixOpn {
                                                        opr: Option,
                                                        opr_token_idx: TokenIdx(
                                                            170,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    Root {
                                                        token_idx: TokenIdx(
                                                            167,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    Root {
                                                        token_idx: TokenIdx(
                                                            171,
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
                                                    data: [
                                                        Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `cc`,
                                                                token_idx: TokenIdx(
                                                                    164,
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
                                                                        value: 179,
                                                                    },
                                                                ),
                                                            ),
                                                            0,
                                                        ),
                                                    ],
                                                ],
                                                pattern_symbol_arena: Arena {
                                                    data: [
                                                        Atom(
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
                                                            ident: `cc`,
                                                            access_start: TokenIdx(
                                                                165,
                                                            ),
                                                            access_end: None,
                                                            variant: Parameter {
                                                                pattern_symbol: 0,
                                                            },
                                                        },
                                                    ],
                                                },
                                                allow_self_type: False,
                                                allow_self_value: False,
                                                ty_constraints: [
                                                    RegularParameter {
                                                        pattern: 0,
                                                        ty: 1,
                                                    },
                                                ],
                                            },
                                            roots: [
                                                ExprRoot {
                                                    kind: OutputType,
                                                    expr: 3,
                                                },
                                            ],
                                        },
                                    },
                                ),
                                path: Defn(
                                    Entity(
                                        FormPath(`mnist_classifier::digits::seven::leftdowncc_pattern`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                176,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: Parameter,
                                        },
                                        MethodCall {
                                            this_expr: 0,
                                            dot_token_idx: TokenIdx(
                                                177,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    178,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                179,
                                            ),
                                            arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                180,
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                182,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Field {
                                            this_expr: 2,
                                            dot_token_idx: TokenIdx(
                                                183,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    184,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                186,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 3,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                185,
                                            ),
                                            ropd: 4,
                                        },
                                        InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                188,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: Parameter,
                                        },
                                        Field {
                                            this_expr: 6,
                                            dot_token_idx: TokenIdx(
                                                189,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    190,
                                                ),
                                            },
                                        },
                                        MethodCall {
                                            this_expr: 7,
                                            dot_token_idx: TokenIdx(
                                                191,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    192,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                193,
                                            ),
                                            arguments: ArenaIdxRange(
                                                8..8,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                194,
                                            ),
                                        },
                                        Literal(
                                            TokenIdx(
                                                196,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 8,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                195,
                                            ),
                                            ropd: 9,
                                        },
                                        InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                200,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: Parameter,
                                        },
                                        MethodCall {
                                            this_expr: 11,
                                            dot_token_idx: TokenIdx(
                                                201,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `start_tangent`,
                                                token_idx: TokenIdx(
                                                    202,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                203,
                                            ),
                                            arguments: ArenaIdxRange(
                                                12..12,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                204,
                                            ),
                                        },
                                        Literal(
                                            TokenIdx(
                                                208,
                                            ),
                                        ),
                                        MethodCall {
                                            this_expr: 12,
                                            dot_token_idx: TokenIdx(
                                                205,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `angle`,
                                                token_idx: TokenIdx(
                                                    206,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                207,
                                            ),
                                            arguments: ArenaIdxRange(
                                                13..14,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                209,
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: `ang`,
                                            token_idx: TokenIdx(
                                                211,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                213,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 15,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                212,
                                            ),
                                            ropd: 16,
                                        },
                                        CurrentSymbol {
                                            ident: `ang`,
                                            token_idx: TokenIdx(
                                                214,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Block {
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
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    173,
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
                                                        175,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                1,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    181,
                                                ),
                                            },
                                            condition: Ok(
                                                5,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    187,
                                                ),
                                            },
                                            condition: Ok(
                                                10,
                                            ),
                                        },
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    197,
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
                                                        199,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                14,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    210,
                                                ),
                                            },
                                            condition: Ok(
                                                17,
                                            ),
                                        },
                                        Eval {
                                            expr: 18,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `dp`,
                                                    token_idx: TokenIdx(
                                                        174,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `ang`,
                                                    token_idx: TokenIdx(
                                                        198,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                        ],
                                    },
                                    pattern_infos: [
                                        Let,
                                        Let,
                                    ],
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 299,
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
                                                            value: 425,
                                                        },
                                                    ),
                                                ),
                                                1,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            Atom(
                                                0,
                                            ),
                                            Atom(
                                                1,
                                            ),
                                        ],
                                    },
                                },
                                symbol_region: SymbolRegion {
                                    inherited_symbol_arena: Arena {
                                        data: [
                                            InheritedSymbol {
                                                ident: `cc`,
                                                kind: Parameter,
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: `dp`,
                                                access_start: TokenIdx(
                                                    175,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            215,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 0,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `ang`,
                                                access_start: TokenIdx(
                                                    199,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            215,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 1,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    ty_constraints: [],
                                },
                                roots: [],
                            },
                        },
                        body: Ok(
                            19,
                        ),
                    },
                ),
            ),
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::seven::is_seven`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::seven::is_seven`, `Feature`),
                            ast_idx: 52,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        217,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                OutputTypeExpr {
                                    expr: 1,
                                },
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        220,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::digits::seven::is_seven`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Err(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        219,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 106,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                            PrefixOpn {
                                                opr: Option,
                                                opr_token_idx: TokenIdx(
                                                    218,
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
                                        ty_constraints: [],
                                    },
                                    roots: [
                                        ExprRoot {
                                            kind: OutputType,
                                            expr: 1,
                                        },
                                    ],
                                },
                            },
                        },
                        expr_region: ExprRegion {
                            data: ExprRegionData {
                                parent: None,
                                path: Defn(
                                    Entity(
                                        FormPath(`mnist_classifier::digits::seven::is_seven`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                            ),
                                        },
                                        Be {
                                            src: 0,
                                            be_token_idx: TokenIdx(
                                                223,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 0,
                                                },
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::six::is_six`, `Feature`),
                                            ),
                                        },
                                        Be {
                                            src: 2,
                                            be_token_idx: TokenIdx(
                                                227,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 1,
                                                },
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                                            ),
                                        },
                                        Be {
                                            src: 4,
                                            be_token_idx: TokenIdx(
                                                231,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 2,
                                                },
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 3,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 6,
                                            dot_token_idx: TokenIdx(
                                                235,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `max_hole_ilen`,
                                                token_idx: TokenIdx(
                                                    236,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                238,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 7,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                237,
                                            ),
                                            ropd: 8,
                                        },
                                        EntityPath {
                                            entity_path_expr: 4,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 10,
                                            dot_token_idx: TokenIdx(
                                                243,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    244,
                                                ),
                                            },
                                        },
                                        CurrentSymbol {
                                            ident: `simple_match_norm`,
                                            token_idx: TokenIdx(
                                                246,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 3,
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                248,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 12,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                247,
                                            ),
                                            ropd: 13,
                                        },
                                        EntityPath {
                                            entity_path_expr: 5,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 15,
                                            dot_token_idx: TokenIdx(
                                                252,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    253,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                255,
                                            ),
                                        ),
                                        NewBoxList {
                                            caller: Some(
                                                16,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                254,
                                            ),
                                            items: ArenaIdxRange(
                                                17..18,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                256,
                                            ),
                                        },
                                        Be {
                                            src: 18,
                                            be_token_idx: TokenIdx(
                                                257,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 4,
                                                },
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 6,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 7,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 20,
                                            dot_token_idx: TokenIdx(
                                                263,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    264,
                                                ),
                                            },
                                        },
                                        Field {
                                            this_expr: 21,
                                            dot_token_idx: TokenIdx(
                                                267,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    268,
                                                ),
                                            },
                                        },
                                        BinaryOpn {
                                            lopd: 22,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                265,
                                            ),
                                            ropd: 23,
                                        },
                                        CurrentSymbol {
                                            ident: `upper_excess`,
                                            token_idx: TokenIdx(
                                                270,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 3,
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                272,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 25,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                271,
                                            ),
                                            ropd: 26,
                                        },
                                        EntityPath {
                                            entity_path_expr: 8,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 28,
                                            dot_token_idx: TokenIdx(
                                                278,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    279,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                281,
                                            ),
                                        ),
                                        NewBoxList {
                                            caller: Some(
                                                29,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                280,
                                            ),
                                            items: ArenaIdxRange(
                                                30..31,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                282,
                                            ),
                                        },
                                        MethodCall {
                                            this_expr: 31,
                                            dot_token_idx: TokenIdx(
                                                283,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `end_tangent`,
                                                token_idx: TokenIdx(
                                                    284,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                285,
                                            ),
                                            arguments: ArenaIdxRange(
                                                32..32,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                286,
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: `end_tangent`,
                                            token_idx: TokenIdx(
                                                290,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 5,
                                            },
                                        },
                                        Field {
                                            this_expr: 33,
                                            dot_token_idx: TokenIdx(
                                                291,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    292,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                297,
                                            ),
                                        ),
                                        CurrentSymbol {
                                            ident: `a`,
                                            token_idx: TokenIdx(
                                                294,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 3,
                                            },
                                        },
                                        PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                296,
                                            ),
                                            opd: 35,
                                        },
                                        BinaryOpn {
                                            lopd: 36,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                295,
                                            ),
                                            ropd: 37,
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    298,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 106,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Err(
                                            UnresolvedSubentity {
                                                token_idx: TokenIdx(
                                                    300,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 428,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        BinaryOpn {
                                            lopd: 39,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                299,
                                            ),
                                            ropd: 40,
                                        },
                                        CurrentSymbol {
                                            ident: `simple_match_norm`,
                                            token_idx: TokenIdx(
                                                302,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 7,
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                304,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 42,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                303,
                                            ),
                                            ropd: 43,
                                        },
                                        EntityPath {
                                            entity_path_expr: 9,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 10,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 45,
                                            dot_token_idx: TokenIdx(
                                                310,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    311,
                                                ),
                                            },
                                        },
                                        Field {
                                            this_expr: 46,
                                            dot_token_idx: TokenIdx(
                                                314,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    315,
                                                ),
                                            },
                                        },
                                        BinaryOpn {
                                            lopd: 47,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                312,
                                            ),
                                            ropd: 48,
                                        },
                                        CurrentSymbol {
                                            ident: `upper_excess`,
                                            token_idx: TokenIdx(
                                                317,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 3,
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                319,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 50,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                318,
                                            ),
                                            ropd: 51,
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    320,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 106,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Err(
                                            UnresolvedSubentity {
                                                token_idx: TokenIdx(
                                                    322,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 428,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        BinaryOpn {
                                            lopd: 53,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                321,
                                            ),
                                            ropd: 54,
                                        },
                                        EntityPath {
                                            entity_path_expr: 11,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::seven::special_seven_match`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 56,
                                            dot_token_idx: TokenIdx(
                                                325,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    326,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                328,
                                            ),
                                        ),
                                        NewBoxList {
                                            caller: Some(
                                                57,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                327,
                                            ),
                                            items: ArenaIdxRange(
                                                58..59,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                329,
                                            ),
                                        },
                                        Be {
                                            src: 59,
                                            be_token_idx: TokenIdx(
                                                330,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 9,
                                                },
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 12,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::seven::special_seven_match`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 61,
                                            dot_token_idx: TokenIdx(
                                                336,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `others`,
                                                token_idx: TokenIdx(
                                                    337,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                339,
                                            ),
                                        ),
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    340,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 106,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Err(
                                            UnresolvedSubentity {
                                                token_idx: TokenIdx(
                                                    342,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 428,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        BinaryOpn {
                                            lopd: 64,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                341,
                                            ),
                                            ropd: 65,
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                10..21,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        Root {
                                            token_idx: TokenIdx(
                                                222,
                                            ),
                                            ident: `is_one`,
                                            entity_path: FormPath(`mnist_classifier::digits::one::is_one`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                226,
                                            ),
                                            ident: `is_six`,
                                            entity_path: FormPath(`mnist_classifier::digits::six::is_six`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                230,
                                            ),
                                            ident: `is_zero`,
                                            entity_path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                234,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                242,
                                            ),
                                            ident: `simple_seven_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                251,
                                            ),
                                            ident: `simple_seven_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                262,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                266,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                277,
                                            ),
                                            ident: `simple_seven_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::seven::simple_seven_match`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                309,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                313,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                324,
                                            ),
                                            ident: `special_seven_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::seven::special_seven_match`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                335,
                                            ),
                                            ident: `special_seven_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::seven::special_seven_match`, `Feature`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    274,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 6,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        276,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                32,
                                            ),
                                        },
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    287,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        289,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                34,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    293,
                                                ),
                                            },
                                            condition: Ok(
                                                38,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    250,
                                                ),
                                            },
                                            condition: Ok(
                                                19,
                                            ),
                                        },
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    259,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 5,
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
                                                        261,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                24,
                                            ),
                                        },
                                        IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        269,
                                                    ),
                                                },
                                                condition: Ok(
                                                    27,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            273,
                                                        ),
                                                    },
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
                                        Eval {
                                            expr: 41,
                                        },
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    306,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        308,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                49,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    316,
                                                ),
                                            },
                                            condition: Ok(
                                                52,
                                            ),
                                        },
                                        Eval {
                                            expr: 55,
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    221,
                                                ),
                                            },
                                            condition: Ok(
                                                1,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    225,
                                                ),
                                            },
                                            condition: Ok(
                                                3,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    229,
                                                ),
                                            },
                                            condition: Ok(
                                                5,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    233,
                                                ),
                                            },
                                            condition: Ok(
                                                9,
                                            ),
                                        },
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    239,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 3,
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
                                                        241,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                11,
                                            ),
                                        },
                                        IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        245,
                                                    ),
                                                },
                                                condition: Ok(
                                                    14,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            249,
                                                        ),
                                                    },
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
                                        IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        301,
                                                    ),
                                                },
                                                condition: Ok(
                                                    44,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            305,
                                                        ),
                                                    },
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
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    323,
                                                ),
                                            },
                                            condition: Ok(
                                                60,
                                            ),
                                        },
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    332,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 10,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        334,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                62,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    338,
                                                ),
                                            },
                                            condition: Ok(
                                                63,
                                            ),
                                        },
                                        Eval {
                                            expr: 66,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        224,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        228,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        232,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `simple_match_norm`,
                                                    token_idx: TokenIdx(
                                                        240,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        258,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `upper_excess`,
                                                    token_idx: TokenIdx(
                                                        260,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `end_tangent`,
                                                    token_idx: TokenIdx(
                                                        275,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `a`,
                                                    token_idx: TokenIdx(
                                                        288,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `upper_excess`,
                                                    token_idx: TokenIdx(
                                                        307,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        331,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `others`,
                                                    token_idx: TokenIdx(
                                                        333,
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
                                        Let,
                                        Be,
                                        Let,
                                        Let,
                                        Let,
                                        Let,
                                        Be,
                                        Let,
                                    ],
                                    pattern_symbol_maps: [
                                        [
                                            (
                                                Identifier(
                                                    Word(
                                                        Id {
                                                            value: 361,
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
                                                            value: 361,
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
                                                            value: 361,
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
                                                            value: 426,
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
                                                            value: 364,
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
                                                            value: 427,
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
                                                            value: 330,
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
                                                            value: 165,
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
                                                            value: 427,
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
                                                            value: 364,
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
                                                            value: 347,
                                                        },
                                                    ),
                                                ),
                                                10,
                                            ),
                                        ],
                                    ],
                                    pattern_symbol_arena: Arena {
                                        data: [
                                            Atom(
                                                0,
                                            ),
                                            Atom(
                                                1,
                                            ),
                                            Atom(
                                                2,
                                            ),
                                            Atom(
                                                3,
                                            ),
                                            Atom(
                                                4,
                                            ),
                                            Atom(
                                                5,
                                            ),
                                            Atom(
                                                6,
                                            ),
                                            Atom(
                                                7,
                                            ),
                                            Atom(
                                                8,
                                            ),
                                            Atom(
                                                9,
                                            ),
                                            Atom(
                                                10,
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
                                                ident: `simple_match_norm`,
                                                access_start: TokenIdx(
                                                    241,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            343,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 3,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `upper_excess`,
                                                access_start: TokenIdx(
                                                    261,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            301,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 5,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `end_tangent`,
                                                access_start: TokenIdx(
                                                    276,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            298,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 6,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `a`,
                                                access_start: TokenIdx(
                                                    289,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            298,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 7,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `upper_excess`,
                                                access_start: TokenIdx(
                                                    308,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            323,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 8,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `others`,
                                                access_start: TokenIdx(
                                                    334,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            343,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 10,
                                                },
                                            },
                                        ],
                                    },
                                    allow_self_type: False,
                                    allow_self_value: False,
                                    ty_constraints: [],
                                },
                                roots: [],
                            },
                        },
                        body: Ok(
                            67,
                        ),
                    },
                ),
            ),
        ],
    },
)