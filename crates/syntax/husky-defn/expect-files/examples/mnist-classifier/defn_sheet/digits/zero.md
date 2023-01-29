Ok(
    DefnSheet {
        defns: [
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                            ast_idx: 33,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        66,
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
                                        68,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Err(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        67,
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
                                        FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
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
                                                FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
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
                                                73,
                                            ),
                                            items: ArenaIdxRange(
                                                1..2,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                75,
                                            ),
                                        },
                                        FunctionCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                70,
                                            ),
                                            arguments: ArenaIdxRange(
                                                2..4,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                76,
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
                                                69,
                                            ),
                                            ident: `fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                71,
                                            ),
                                            ident: `major_concave_components`,
                                            entity_path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                74,
                                            ),
                                            ident: `almost_closed`,
                                            entity_path: FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
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
                        path: FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
                            ast_idx: 34,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
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
                                                    82,
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
                                                    86,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    83,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    87,
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
                                                        81,
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
                                        79,
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
                                                81,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        84,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        85,
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
                                        88,
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
                                                    FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
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
                                                            82,
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
                                                            86,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    Root {
                                                        token_idx: TokenIdx(
                                                            83,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    Root {
                                                        token_idx: TokenIdx(
                                                            87,
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
                                                                81,
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
                                        FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                90,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: Parameter,
                                        },
                                        Field {
                                            this_expr: 0,
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
                                        Literal(
                                            TokenIdx(
                                                94,
                                            ),
                                        ),
                                        Literal(
                                            TokenIdx(
                                                97,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 1,
                                            opr: PureClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                93,
                                            ),
                                            ropd: 2,
                                        },
                                        PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                96,
                                            ),
                                            opd: 3,
                                        },
                                        BinaryOpn {
                                            lopd: 4,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                95,
                                            ),
                                            ropd: 5,
                                        },
                                        InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                99,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: Parameter,
                                        },
                                        Field {
                                            this_expr: 7,
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
                                        PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                98,
                                            ),
                                            opd: 8,
                                        },
                                        Literal(
                                            TokenIdx(
                                                103,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 9,
                                            opr: PureClosed(
                                                Add,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                102,
                                            ),
                                            ropd: 10,
                                        },
                                        Block {
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
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    89,
                                                ),
                                            },
                                            condition: Ok(
                                                6,
                                            ),
                                        },
                                        Eval {
                                            expr: 11,
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
                                                ident: `cc`,
                                                kind: Parameter,
                                            },
                                        ],
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
                            12,
                        ),
                    },
                ),
            ),
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                            ast_idx: 35,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        106,
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
                                        109,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Err(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        108,
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
                                        FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    111,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 113,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Be {
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
                                        EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 2,
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
                                        MethodCall {
                                            this_expr: 3,
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
                                            arguments: ArenaIdxRange(
                                                4..4,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                121,
                                            ),
                                        },
                                        Literal(
                                            TokenIdx(
                                                123,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 4,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                122,
                                            ),
                                            ropd: 5,
                                        },
                                        EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                                            ),
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    125,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 363,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Field {
                                            this_expr: 7,
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
                                        BinaryOpn {
                                            lopd: 8,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                126,
                                            ),
                                            ropd: 9,
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    131,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 363,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Literal(
                                            TokenIdx(
                                                133,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 11,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                132,
                                            ),
                                            ropd: 12,
                                        },
                                        EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 14,
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
                                        Literal(
                                            TokenIdx(
                                                139,
                                            ),
                                        ),
                                        NewBoxList {
                                            caller: Some(
                                                15,
                                            ),
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
                                        Be {
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
                                        EntityPath {
                                            entity_path_expr: 3,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                            ),
                                        },
                                        MethodCall {
                                            this_expr: 19,
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
                                            arguments: ArenaIdxRange(
                                                20..20,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                148,
                                            ),
                                        },
                                        Literal(
                                            TokenIdx(
                                                150,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 20,
                                            opr: Comparison(
                                                Eq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                149,
                                            ),
                                            ropd: 21,
                                        },
                                        EntityPath {
                                            entity_path_expr: 4,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 23,
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
                                        Literal(
                                            TokenIdx(
                                                157,
                                            ),
                                        ),
                                        NewBoxList {
                                            caller: Some(
                                                24,
                                            ),
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
                                        MethodCall {
                                            this_expr: 26,
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
                                            arguments: ArenaIdxRange(
                                                27..27,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                162,
                                            ),
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    151,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 366,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        MethodCall {
                                            this_expr: 27,
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
                                            arguments: ArenaIdxRange(
                                                28..28,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                166,
                                            ),
                                        },
                                        BinaryOpn {
                                            lopd: 28,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                152,
                                            ),
                                            ropd: 29,
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    168,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 366,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Literal(
                                            TokenIdx(
                                                170,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 31,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                169,
                                            ),
                                            ropd: 32,
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    171,
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
                                                    173,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        BinaryOpn {
                                            lopd: 34,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                172,
                                            ),
                                            ropd: 35,
                                        },
                                        EntityPath {
                                            entity_path_expr: 5,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 6,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                            ),
                                        },
                                        NewBoxList {
                                            caller: None,
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
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    174,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 368,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        FunctionCall {
                                            function: 37,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                177,
                                            ),
                                            arguments: ArenaIdxRange(
                                                38..40,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                182,
                                            ),
                                        },
                                        BinaryOpn {
                                            lopd: 40,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                175,
                                            ),
                                            ropd: 41,
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    183,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 357,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    185,
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
                                                    187,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    191,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 368,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    195,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 368,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    199,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 368,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        BinaryOpn {
                                            lopd: 44,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                186,
                                            ),
                                            ropd: 45,
                                        },
                                        Literal(
                                            TokenIdx(
                                                189,
                                            ),
                                        ),
                                        Field {
                                            this_expr: 46,
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
                                        Field {
                                            this_expr: 47,
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
                                        Field {
                                            this_expr: 48,
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
                                        FunctionCall {
                                            function: 43,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                184,
                                            ),
                                            arguments: ArenaIdxRange(
                                                49..54,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                203,
                                            ),
                                        },
                                        SuffixOpn {
                                            opd: 54,
                                            punctuation: Unveil,
                                            punctuation_token_idx: TokenIdx(
                                                204,
                                            ),
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    206,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 368,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Field {
                                            this_expr: 56,
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
                                        Literal(
                                            TokenIdx(
                                                210,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 57,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                209,
                                            ),
                                            ropd: 58,
                                        },
                                        EntityPath {
                                            entity_path_expr: 7,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 60,
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
                                        Field {
                                            this_expr: 61,
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
                                        Literal(
                                            TokenIdx(
                                                218,
                                            ),
                                        ),
                                        NewBoxList {
                                            caller: Some(
                                                62,
                                            ),
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
                                        Be {
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
                                        EntityPath {
                                            entity_path_expr: 8,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 66,
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
                                        Field {
                                            this_expr: 67,
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
                                        Literal(
                                            TokenIdx(
                                                229,
                                            ),
                                        ),
                                        NewBoxList {
                                            caller: Some(
                                                68,
                                            ),
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
                                        Be {
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
                                        EntityPath {
                                            entity_path_expr: 9,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 72,
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
                                        Field {
                                            this_expr: 73,
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
                                        Literal(
                                            TokenIdx(
                                                241,
                                            ),
                                        ),
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    233,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 369,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        NewBoxList {
                                            caller: Some(
                                                74,
                                            ),
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
                                        BinaryOpn {
                                            lopd: 76,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                234,
                                            ),
                                            ropd: 77,
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    245,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 369,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Field {
                                            this_expr: 79,
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
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    253,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 369,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Field {
                                            this_expr: 81,
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
                                        MethodCall {
                                            this_expr: 80,
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
                                            arguments: ArenaIdxRange(
                                                81..81,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                251,
                                            ),
                                        },
                                        MethodCall {
                                            this_expr: 82,
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
                                            arguments: ArenaIdxRange(
                                                83..83,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                259,
                                            ),
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    243,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 165,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        BinaryOpn {
                                            lopd: 83,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                252,
                                            ),
                                            ropd: 84,
                                        },
                                        BinaryOpn {
                                            lopd: 85,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                244,
                                            ),
                                            ropd: 86,
                                        },
                                        EntityPath {
                                            entity_path_expr: 10,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 88,
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
                                        EntityPath {
                                            entity_path_expr: 11,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 90,
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
                                        MethodCall {
                                            this_expr: 89,
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
                                            arguments: ArenaIdxRange(
                                                90..90,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                268,
                                            ),
                                        },
                                        MethodCall {
                                            this_expr: 91,
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
                                            arguments: ArenaIdxRange(
                                                92..92,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                276,
                                            ),
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    260,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 195,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        BinaryOpn {
                                            lopd: 92,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                269,
                                            ),
                                            ropd: 93,
                                        },
                                        BinaryOpn {
                                            lopd: 94,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                261,
                                            ),
                                            ropd: 95,
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    279,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 165,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    281,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 195,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    277,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 371,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        BinaryOpn {
                                            lopd: 97,
                                            opr: PureClosed(
                                                Div,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                280,
                                            ),
                                            ropd: 98,
                                        },
                                        BinaryOpn {
                                            lopd: 99,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                278,
                                            ),
                                            ropd: 100,
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    283,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 371,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Literal(
                                            TokenIdx(
                                                285,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 102,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                284,
                                            ),
                                            ropd: 103,
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    288,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 368,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    286,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 165,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        Field {
                                            this_expr: 105,
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
                                        BinaryOpn {
                                            lopd: 106,
                                            opr: Assign(
                                                None,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                287,
                                            ),
                                            ropd: 107,
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    291,
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
                                                    293,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        BinaryOpn {
                                            lopd: 109,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                292,
                                            ),
                                            ropd: 110,
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                7..21,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        Root {
                                            token_idx: TokenIdx(
                                                115,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                127,
                                            ),
                                            ident: `open_one_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                135,
                                            ),
                                            ident: `open_one_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                144,
                                            ),
                                            ident: `connected_components`,
                                            entity_path: FormPath(`mnist_classifier::major::connected_components`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                153,
                                            ),
                                            ident: `open_one_match`,
                                            entity_path: FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                176,
                                            ),
                                            ident: `fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                178,
                                            ),
                                            ident: `major_concave_components`,
                                            entity_path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                212,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                223,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                235,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                262,
                                            ),
                                            ident: `major_line_segment_sketch`,
                                            entity_path: FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                270,
                                            ),
                                            ident: `major_line_segment_sketch`,
                                            entity_path: FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Feature`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 10,
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    130,
                                                ),
                                            },
                                            condition: Ok(
                                                13,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    134,
                                                ),
                                            },
                                            condition: Ok(
                                                18,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    143,
                                                ),
                                            },
                                            condition: Ok(
                                                22,
                                            ),
                                        },
                                        Eval {
                                            expr: 30,
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    167,
                                                ),
                                            },
                                            condition: Ok(
                                                33,
                                            ),
                                        },
                                        Eval {
                                            expr: 36,
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    110,
                                                ),
                                            },
                                            condition: Ok(
                                                1,
                                            ),
                                        },
                                        IfElse {
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
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            124,
                                                        ),
                                                    },
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
                                        Eval {
                                            expr: 42,
                                        },
                                        Eval {
                                            expr: 55,
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    205,
                                                ),
                                            },
                                            condition: Ok(
                                                59,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    211,
                                                ),
                                            },
                                            condition: Ok(
                                                65,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    222,
                                                ),
                                            },
                                            condition: Ok(
                                                71,
                                            ),
                                        },
                                        Eval {
                                            expr: 78,
                                        },
                                        Eval {
                                            expr: 87,
                                        },
                                        Eval {
                                            expr: 96,
                                        },
                                        Eval {
                                            expr: 101,
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    282,
                                                ),
                                            },
                                            condition: Ok(
                                                104,
                                            ),
                                        },
                                        Eval {
                                            expr: 108,
                                        },
                                        Eval {
                                            expr: 111,
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
                                                        113,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        142,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        221,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
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
                                                            value: 364,
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
                                                            value: 364,
                                                        },
                                                    ),
                                                ),
                                                3,
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
                                    ty_constraints: [],
                                },
                                roots: [],
                            },
                        },
                        body: Ok(
                            112,
                        ),
                    },
                ),
            ),
        ],
    },
)