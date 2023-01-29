Ok(
    DefnSheet {
        defns: [
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
                            ast_idx: 44,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        64,
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
                                        66,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Err(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        65,
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
                                        FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
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
                                                FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 3,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
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
                                                71,
                                            ),
                                            items: ArenaIdxRange(
                                                1..3,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                75,
                                            ),
                                        },
                                        FunctionCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                68,
                                            ),
                                            arguments: ArenaIdxRange(
                                                3..5,
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
                                                67,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 349,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 24,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                69,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 359,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 70,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                72,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 409,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 44,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                74,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 409,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 44,
                                                        },
                                                    ),
                                                ),
                                            ),
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
                        path: FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
                            ast_idx: 45,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
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
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 287,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 31,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    87,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 42,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 13,
                                                            },
                                                        ),
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
                                                Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 179,
                                                                },
                                                            ),
                                                        ),
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
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 179,
                                                            },
                                                        ),
                                                    ),
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
                                                    FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
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
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 287,
                                                                },
                                                            ),
                                                        ),
                                                        entity_path: ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Root {
                                                        token_idx: TokenIdx(
                                                            87,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 42,
                                                                },
                                                            ),
                                                        ),
                                                        entity_path: ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 13,
                                                                    },
                                                                ),
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
                                                        Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 179,
                                                                        },
                                                                    ),
                                                                ),
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
                                                            ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 179,
                                                                    },
                                                                ),
                                                            ),
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
                                        FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                89,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: Parameter,
                                        },
                                        Field {
                                            this_expr: 0,
                                            dot_token_idx: TokenIdx(
                                                90,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    91,
                                                ),
                                            },
                                        },
                                        MethodCall {
                                            this_expr: 1,
                                            dot_token_idx: TokenIdx(
                                                92,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `xmax`,
                                                token_idx: TokenIdx(
                                                    93,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                94,
                                            ),
                                            arguments: ArenaIdxRange(
                                                2..2,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                95,
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
                                    data: [],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
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
                                        data: [
                                            InheritedSymbol {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 179,
                                                        },
                                                    ),
                                                ),
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
                            3,
                        ),
                    },
                ),
            ),
            Form(
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Feature`),
                            ast_idx: 46,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        98,
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
                                        100,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Err(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        99,
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
                                        FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Feature`),
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
                                                FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Function`),
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
                                                105,
                                            ),
                                            items: ArenaIdxRange(
                                                1..2,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                107,
                                            ),
                                        },
                                        FunctionCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                102,
                                            ),
                                            arguments: ArenaIdxRange(
                                                2..4,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                108,
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
                                                101,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 349,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 24,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                103,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 359,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 70,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                106,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 411,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 48,
                                                        },
                                                    ),
                                                ),
                                            ),
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
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                            ast_idx: 47,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        111,
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
                                        113,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Err(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        112,
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
                                        FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
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
                                                FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Function`),
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
                                                118,
                                            ),
                                            items: ArenaIdxRange(
                                                1..2,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                120,
                                            ),
                                        },
                                        FunctionCall {
                                            function: 0,
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                115,
                                            ),
                                            arguments: ArenaIdxRange(
                                                2..4,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                121,
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
                                                114,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 349,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 24,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                116,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 359,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 70,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                119,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 413,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 49,
                                                        },
                                                    ),
                                                ),
                                            ),
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
                Feature(
                    FeatureDefn {
                        path: FormPath(`mnist_classifier::digits::four::is_four`, `Feature`),
                        decl: FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::four::is_four`, `Feature`),
                            ast_idx: 48,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        124,
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
                                        127,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::digits::four::is_four`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Err(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        126,
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
                                                    125,
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
                                        FormPath(`mnist_classifier::digits::four::is_four`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 0,
                                            dot_token_idx: TokenIdx(
                                                130,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    131,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                133,
                                            ),
                                        ),
                                        NewBoxList {
                                            caller: Some(
                                                1,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                132,
                                            ),
                                            items: ArenaIdxRange(
                                                2..3,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                134,
                                            ),
                                        },
                                        Be {
                                            src: 3,
                                            be_token_idx: TokenIdx(
                                                135,
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
                                                FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 5,
                                            dot_token_idx: TokenIdx(
                                                139,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    140,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                142,
                                            ),
                                        ),
                                        NewBoxList {
                                            caller: Some(
                                                6,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                141,
                                            ),
                                            items: ArenaIdxRange(
                                                7..8,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                143,
                                            ),
                                        },
                                        Be {
                                            src: 8,
                                            be_token_idx: TokenIdx(
                                                144,
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
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 10,
                                            dot_token_idx: TokenIdx(
                                                150,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `eff_holes`,
                                                token_idx: TokenIdx(
                                                    151,
                                                ),
                                            },
                                        },
                                        CurrentSymbol {
                                            ident: `eff_holes`,
                                            token_idx: TokenIdx(
                                                153,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 2,
                                            },
                                        },
                                        Field {
                                            this_expr: 12,
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
                                                13,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                156,
                                            ),
                                            items: ArenaIdxRange(
                                                14..15,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                158,
                                            ),
                                        },
                                        Be {
                                            src: 15,
                                            be_token_idx: TokenIdx(
                                                159,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 3,
                                                },
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 3,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 17,
                                            dot_token_idx: TokenIdx(
                                                165,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    166,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                168,
                                            ),
                                        ),
                                        NewBoxList {
                                            caller: Some(
                                                18,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                167,
                                            ),
                                            items: ArenaIdxRange(
                                                19..20,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                169,
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: `down_match`,
                                            token_idx: TokenIdx(
                                                171,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 2,
                                            },
                                        },
                                        Be {
                                            src: 21,
                                            be_token_idx: TokenIdx(
                                                172,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 5,
                                                },
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: `down_match`,
                                            token_idx: TokenIdx(
                                                177,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 4,
                                            },
                                        },
                                        MethodCall {
                                            this_expr: 23,
                                            dot_token_idx: TokenIdx(
                                                178,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    179,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                180,
                                            ),
                                            arguments: ArenaIdxRange(
                                                24..24,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                181,
                                            ),
                                        },
                                        Field {
                                            this_expr: 24,
                                            dot_token_idx: TokenIdx(
                                                182,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    183,
                                                ),
                                            },
                                        },
                                        EntityPath {
                                            entity_path_expr: 4,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 5,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 26,
                                            dot_token_idx: TokenIdx(
                                                188,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    189,
                                                ),
                                            },
                                        },
                                        Field {
                                            this_expr: 27,
                                            dot_token_idx: TokenIdx(
                                                192,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    193,
                                                ),
                                            },
                                        },
                                        BinaryOpn {
                                            lopd: 28,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                190,
                                            ),
                                            ropd: 29,
                                        },
                                        CurrentSymbol {
                                            ident: `higher_excess`,
                                            token_idx: TokenIdx(
                                                195,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 2,
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                197,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 31,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                196,
                                            ),
                                            ropd: 32,
                                        },
                                        CurrentSymbol {
                                            ident: `eff_holes`,
                                            token_idx: TokenIdx(
                                                199,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 7,
                                            },
                                        },
                                        Field {
                                            this_expr: 34,
                                            dot_token_idx: TokenIdx(
                                                200,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    201,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                203,
                                            ),
                                        ),
                                        NewBoxList {
                                            caller: Some(
                                                35,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                202,
                                            ),
                                            items: ArenaIdxRange(
                                                36..37,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                204,
                                            ),
                                        },
                                        Be {
                                            src: 37,
                                            be_token_idx: TokenIdx(
                                                205,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 8,
                                                },
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 6,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                            ),
                                        },
                                        MethodCall {
                                            this_expr: 39,
                                            dot_token_idx: TokenIdx(
                                                210,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `ilen`,
                                                token_idx: TokenIdx(
                                                    211,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                212,
                                            ),
                                            arguments: ArenaIdxRange(
                                                40..40,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                213,
                                            ),
                                        },
                                        Literal(
                                            TokenIdx(
                                                215,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 40,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                214,
                                            ),
                                            ropd: 41,
                                        },
                                        EntityPath {
                                            entity_path_expr: 7,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 43,
                                            dot_token_idx: TokenIdx(
                                                220,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    221,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                223,
                                            ),
                                        ),
                                        NewBoxList {
                                            caller: Some(
                                                44,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                222,
                                            ),
                                            items: ArenaIdxRange(
                                                45..46,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                224,
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: `four_match_refine_result`,
                                            token_idx: TokenIdx(
                                                226,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 2,
                                            },
                                        },
                                        Be {
                                            src: 47,
                                            be_token_idx: TokenIdx(
                                                227,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 10,
                                                },
                                            ),
                                        },
                                        EntityPath {
                                            entity_path_expr: 8,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 49,
                                            dot_token_idx: TokenIdx(
                                                231,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    232,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                234,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 50,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                233,
                                            ),
                                            ropd: 51,
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
                                            this_expr: 53,
                                            dot_token_idx: TokenIdx(
                                                239,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `upper_mass`,
                                                token_idx: TokenIdx(
                                                    240,
                                                ),
                                            },
                                        },
                                        Field {
                                            this_expr: 54,
                                            dot_token_idx: TokenIdx(
                                                243,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `lower_mass`,
                                                token_idx: TokenIdx(
                                                    244,
                                                ),
                                            },
                                        },
                                        BinaryOpn {
                                            lopd: 55,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                241,
                                            ),
                                            ropd: 56,
                                        },
                                        EntityPath {
                                            entity_path_expr: 11,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 58,
                                            dot_token_idx: TokenIdx(
                                                249,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `matches`,
                                                token_idx: TokenIdx(
                                                    250,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                252,
                                            ),
                                        ),
                                        NewBoxList {
                                            caller: Some(
                                                59,
                                            ),
                                            lbox_token_idx: TokenIdx(
                                                251,
                                            ),
                                            items: ArenaIdxRange(
                                                60..61,
                                            ),
                                            rbox_token_idx: TokenIdx(
                                                253,
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: `upper_arc`,
                                            token_idx: TokenIdx(
                                                255,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 2,
                                            },
                                        },
                                        Be {
                                            src: 62,
                                            be_token_idx: TokenIdx(
                                                256,
                                            ),
                                            target: Ok(
                                                BeVariableDeclPattern {
                                                    pattern_expr_idx: 13,
                                                },
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: `upper_arc`,
                                            token_idx: TokenIdx(
                                                259,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 2,
                                            },
                                        },
                                        MethodCall {
                                            this_expr: 64,
                                            dot_token_idx: TokenIdx(
                                                260,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    261,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                262,
                                            ),
                                            arguments: ArenaIdxRange(
                                                65..65,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                263,
                                            ),
                                        },
                                        Field {
                                            this_expr: 65,
                                            dot_token_idx: TokenIdx(
                                                264,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    265,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                267,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 66,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                266,
                                            ),
                                            ropd: 67,
                                        },
                                        CurrentSymbol {
                                            ident: `upper_arc`,
                                            token_idx: TokenIdx(
                                                269,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 2,
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                274,
                                            ),
                                        ),
                                        Field {
                                            this_expr: 69,
                                            dot_token_idx: TokenIdx(
                                                270,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `angle_change`,
                                                token_idx: TokenIdx(
                                                    271,
                                                ),
                                            },
                                        },
                                        PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                273,
                                            ),
                                            opd: 70,
                                        },
                                        BinaryOpn {
                                            lopd: 71,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                272,
                                            ),
                                            ropd: 72,
                                        },
                                        EntityPath {
                                            entity_path_expr: 12,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                                            ),
                                        },
                                        Field {
                                            this_expr: 74,
                                            dot_token_idx: TokenIdx(
                                                277,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `norm`,
                                                token_idx: TokenIdx(
                                                    278,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                280,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 75,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                279,
                                            ),
                                            ropd: 76,
                                        },
                                        EntityPath {
                                            entity_path_expr: 13,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Literal(
                                            TokenIdx(
                                                288,
                                            ),
                                        ),
                                        MethodCall {
                                            this_expr: 78,
                                            dot_token_idx: TokenIdx(
                                                285,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `top_k_row_right_mass_sum`,
                                                token_idx: TokenIdx(
                                                    286,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                287,
                                            ),
                                            arguments: ArenaIdxRange(
                                                79..80,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                289,
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: `a`,
                                            token_idx: TokenIdx(
                                                291,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 2,
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                293,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 81,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                292,
                                            ),
                                            ropd: 82,
                                        },
                                        CurrentSymbol {
                                            ident: `a`,
                                            token_idx: TokenIdx(
                                                295,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 2,
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                297,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 84,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                296,
                                            ),
                                            ropd: 85,
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
                                                            value: 122,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        BinaryOpn {
                                            lopd: 87,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                299,
                                            ),
                                            ropd: 88,
                                        },
                                        Err(
                                            UnrecognizedIdentifier {
                                                token_idx: TokenIdx(
                                                    301,
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
                                                    303,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 122,
                                                        },
                                                    ),
                                                ),
                                            },
                                        ),
                                        BinaryOpn {
                                            lopd: 90,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                302,
                                            ),
                                            ropd: 91,
                                        },
                                        Block {
                                            stmts: ArenaIdxRange(
                                                14..25,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        Root {
                                            token_idx: TokenIdx(
                                                129,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 408,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 43,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                138,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 408,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 43,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                149,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 362,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                164,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 410,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 45,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                187,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 362,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                191,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 362,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                209,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 359,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 70,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                219,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 412,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 46,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                230,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 412,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 46,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                238,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 362,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                242,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 362,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                248,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 412,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 46,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                276,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 412,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 46,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        Root {
                                            token_idx: TokenIdx(
                                                284,
                                            ),
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 362,
                                                    },
                                                ),
                                            ),
                                            entity_path: ModuleItem(
                                                Form(
                                                    FormPath(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    208,
                                                ),
                                            },
                                            condition: Ok(
                                                42,
                                            ),
                                        },
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    216,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 9,
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
                                                        218,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                46,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    225,
                                                ),
                                            },
                                            condition: Ok(
                                                48,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    229,
                                                ),
                                            },
                                            condition: Ok(
                                                52,
                                            ),
                                        },
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    235,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 11,
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
                                                        237,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                57,
                                            ),
                                        },
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    245,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 12,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        247,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                61,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    254,
                                                ),
                                            },
                                            condition: Ok(
                                                63,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    258,
                                                ),
                                            },
                                            condition: Ok(
                                                68,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    268,
                                                ),
                                            },
                                            condition: Ok(
                                                73,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    275,
                                                ),
                                            },
                                            condition: Ok(
                                                77,
                                            ),
                                        },
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    281,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 14,
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
                                                AssignToken {
                                                    token_idx: TokenIdx(
                                                        283,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                80,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    290,
                                                ),
                                            },
                                            condition: Ok(
                                                83,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    294,
                                                ),
                                            },
                                            condition: Ok(
                                                86,
                                            ),
                                        },
                                        Eval {
                                            expr: 89,
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    128,
                                                ),
                                            },
                                            condition: Ok(
                                                4,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    137,
                                                ),
                                            },
                                            condition: Ok(
                                                9,
                                            ),
                                        },
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    146,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 2,
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
                                                        148,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                11,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    152,
                                                ),
                                            },
                                            condition: Ok(
                                                16,
                                            ),
                                        },
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    161,
                                                ),
                                            },
                                            let_variable_pattern: Ok(
                                                LetVariablesPattern {
                                                    pattern: 4,
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
                                                        163,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                20,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    170,
                                                ),
                                            },
                                            condition: Ok(
                                                22,
                                            ),
                                        },
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    174,
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
                                                        176,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                25,
                                            ),
                                        },
                                        Let {
                                            let_token: LetToken {
                                                token_idx: TokenIdx(
                                                    184,
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
                                                        186,
                                                    ),
                                                },
                                            ),
                                            initial_value: Ok(
                                                30,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    194,
                                                ),
                                            },
                                            condition: Ok(
                                                33,
                                            ),
                                        },
                                        IfElse {
                                            if_branch: IfBranch {
                                                if_token: IfToken {
                                                    token_idx: TokenIdx(
                                                        198,
                                                    ),
                                                },
                                                condition: Ok(
                                                    38,
                                                ),
                                                eol_colon: Ok(
                                                    EolColonToken {
                                                        token_idx: TokenIdx(
                                                            207,
                                                        ),
                                                    },
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
                                        Eval {
                                            expr: 92,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 364,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        136,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 364,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        145,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 139,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        147,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 361,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        160,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 415,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        162,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 364,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        173,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 416,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        175,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 417,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        185,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 361,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        206,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 418,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        217,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 364,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        228,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 417,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        236,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 419,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        246,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 364,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        257,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 165,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        282,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                        ],
                                    },
                                    pattern_infos: [
                                        Be,
                                        Be,
                                        Let,
                                        Be,
                                        Let,
                                        Be,
                                        Let,
                                        Let,
                                        Be,
                                        Let,
                                        Be,
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
                                                            value: 364,
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
                                                            value: 139,
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
                                                            value: 361,
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
                                                            value: 415,
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
                                                            value: 364,
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
                                                            value: 416,
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
                                                            value: 417,
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
                                                            value: 361,
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
                                                            value: 418,
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
                                                            value: 364,
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
                                                            value: 417,
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
                                                            value: 419,
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
                                                            value: 364,
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
                                                            value: 165,
                                                        },
                                                    ),
                                                ),
                                                14,
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
                                            Atom(
                                                11,
                                            ),
                                            Atom(
                                                12,
                                            ),
                                            Atom(
                                                13,
                                            ),
                                            Atom(
                                                14,
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
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 139,
                                                        },
                                                    ),
                                                ),
                                                access_start: TokenIdx(
                                                    148,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            304,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 2,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 415,
                                                        },
                                                    ),
                                                ),
                                                access_start: TokenIdx(
                                                    163,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            304,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 4,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 416,
                                                        },
                                                    ),
                                                ),
                                                access_start: TokenIdx(
                                                    176,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            304,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 6,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 417,
                                                        },
                                                    ),
                                                ),
                                                access_start: TokenIdx(
                                                    186,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            304,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 7,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 418,
                                                        },
                                                    ),
                                                ),
                                                access_start: TokenIdx(
                                                    218,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            301,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 9,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 417,
                                                        },
                                                    ),
                                                ),
                                                access_start: TokenIdx(
                                                    237,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            301,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 11,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 419,
                                                        },
                                                    ),
                                                ),
                                                access_start: TokenIdx(
                                                    247,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            301,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 12,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 165,
                                                        },
                                                    ),
                                                ),
                                                access_start: TokenIdx(
                                                    283,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            301,
                                                        ),
                                                    ),
                                                ),
                                                variant: LetVariable {
                                                    pattern_symbol: 14,
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
                            93,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Function`),
                            ast_idx: 49,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Function`),
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
                                                    309,
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
                                                    313,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    310,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 287,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 31,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    314,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 42,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 13,
                                                            },
                                                        ),
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
                                                Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 179,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            307,
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
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 179,
                                                            },
                                                        ),
                                                    ),
                                                    access_start: TokenIdx(
                                                        308,
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
                                        306,
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
                                                308,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        311,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        312,
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
                                        315,
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
                                                    FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Function`),
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
                                                            309,
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
                                                            313,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    Root {
                                                        token_idx: TokenIdx(
                                                            310,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 287,
                                                                },
                                                            ),
                                                        ),
                                                        entity_path: ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Root {
                                                        token_idx: TokenIdx(
                                                            314,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 42,
                                                                },
                                                            ),
                                                        ),
                                                        entity_path: ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 13,
                                                                    },
                                                                ),
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
                                                        Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 179,
                                                                        },
                                                                    ),
                                                                ),
                                                                token_idx: TokenIdx(
                                                                    307,
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
                                                            ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 179,
                                                                    },
                                                                ),
                                                            ),
                                                            access_start: TokenIdx(
                                                                308,
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
                                        FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                319,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: Parameter,
                                        },
                                        MethodCall {
                                            this_expr: 0,
                                            dot_token_idx: TokenIdx(
                                                320,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    321,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                322,
                                            ),
                                            arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                323,
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                325,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Field {
                                            this_expr: 2,
                                            dot_token_idx: TokenIdx(
                                                326,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    327,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                329,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 3,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                328,
                                            ),
                                            ropd: 4,
                                        },
                                        CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                330,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Field {
                                            this_expr: 6,
                                            dot_token_idx: TokenIdx(
                                                331,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    332,
                                                ),
                                            },
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
                                                    316,
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
                                                        318,
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
                                                    324,
                                                ),
                                            },
                                            condition: Ok(
                                                5,
                                            ),
                                        },
                                        Eval {
                                            expr: 7,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 299,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        317,
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
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 179,
                                                        },
                                                    ),
                                                ),
                                                kind: Parameter,
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 299,
                                                        },
                                                    ),
                                                ),
                                                access_start: TokenIdx(
                                                    318,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            333,
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
                            8,
                        ),
                    },
                ),
            ),
            Form(
                Function(
                    FunctionDefn {
                        path: FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Function`),
                        decl: FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Function`),
                            ast_idx: 50,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Function`),
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
                                                    338,
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
                                                    342,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    339,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 287,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 31,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    343,
                                                ),
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 42,
                                                        },
                                                    ),
                                                ),
                                                entity_path: ModuleItem(
                                                    Type(
                                                        TypePath(
                                                            Id {
                                                                value: 13,
                                                            },
                                                        ),
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
                                                Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 179,
                                                                },
                                                            ),
                                                        ),
                                                        token_idx: TokenIdx(
                                                            336,
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
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 179,
                                                            },
                                                        ),
                                                    ),
                                                    access_start: TokenIdx(
                                                        337,
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
                                        335,
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
                                                337,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        340,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        341,
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
                                        344,
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
                                                    FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Function`),
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
                                                            338,
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
                                                            342,
                                                        ),
                                                        opd: 2,
                                                    },
                                                ],
                                            },
                                            entity_path_expr_arena: Arena {
                                                data: [
                                                    Root {
                                                        token_idx: TokenIdx(
                                                            339,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 287,
                                                                },
                                                            ),
                                                        ),
                                                        entity_path: ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 31,
                                                                    },
                                                                ),
                                                            ),
                                                        ),
                                                    },
                                                    Root {
                                                        token_idx: TokenIdx(
                                                            343,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 42,
                                                                },
                                                            ),
                                                        ),
                                                        entity_path: ModuleItem(
                                                            Type(
                                                                TypePath(
                                                                    Id {
                                                                        value: 13,
                                                                    },
                                                                ),
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
                                                        Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: Identifier(
                                                                    Word(
                                                                        Id {
                                                                            value: 179,
                                                                        },
                                                                    ),
                                                                ),
                                                                token_idx: TokenIdx(
                                                                    336,
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
                                                            ident: Identifier(
                                                                Word(
                                                                    Id {
                                                                        value: 179,
                                                                    },
                                                                ),
                                                            ),
                                                            access_start: TokenIdx(
                                                                337,
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
                                        FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                348,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: Parameter,
                                        },
                                        MethodCall {
                                            this_expr: 0,
                                            dot_token_idx: TokenIdx(
                                                349,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `displacement`,
                                                token_idx: TokenIdx(
                                                    350,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                351,
                                            ),
                                            arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                352,
                                            ),
                                        },
                                        CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                354,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: LetVariable {
                                                pattern_symbol: 0,
                                            },
                                        },
                                        Field {
                                            this_expr: 2,
                                            dot_token_idx: TokenIdx(
                                                355,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `y`,
                                                token_idx: TokenIdx(
                                                    356,
                                                ),
                                            },
                                        },
                                        Literal(
                                            TokenIdx(
                                                358,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 3,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                357,
                                            ),
                                            ropd: 4,
                                        },
                                        InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                360,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: Parameter,
                                        },
                                        Field {
                                            this_expr: 6,
                                            dot_token_idx: TokenIdx(
                                                361,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    362,
                                                ),
                                            },
                                        },
                                        MethodCall {
                                            this_expr: 7,
                                            dot_token_idx: TokenIdx(
                                                363,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    364,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                365,
                                            ),
                                            arguments: ArenaIdxRange(
                                                8..8,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                366,
                                            ),
                                        },
                                        Literal(
                                            TokenIdx(
                                                368,
                                            ),
                                        ),
                                        BinaryOpn {
                                            lopd: 8,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                367,
                                            ),
                                            ropd: 9,
                                        },
                                        InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                369,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: Parameter,
                                        },
                                        Field {
                                            this_expr: 11,
                                            dot_token_idx: TokenIdx(
                                                370,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `relative_bounding_box`,
                                                token_idx: TokenIdx(
                                                    371,
                                                ),
                                            },
                                        },
                                        MethodCall {
                                            this_expr: 12,
                                            dot_token_idx: TokenIdx(
                                                372,
                                            ),
                                            ident_token: IdentifierToken {
                                                ident: `ymin`,
                                                token_idx: TokenIdx(
                                                    373,
                                                ),
                                            },
                                            implicit_arguments: None,
                                            lpar_token_idx: TokenIdx(
                                                374,
                                            ),
                                            arguments: ArenaIdxRange(
                                                13..13,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                375,
                                            ),
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
                                                    345,
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
                                                        347,
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
                                                    353,
                                                ),
                                            },
                                            condition: Ok(
                                                5,
                                            ),
                                        },
                                        Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    359,
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
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 299,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        346,
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
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 179,
                                                        },
                                                    ),
                                                ),
                                                kind: Parameter,
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: Identifier(
                                                    Word(
                                                        Id {
                                                            value: 299,
                                                        },
                                                    ),
                                                ),
                                                access_start: TokenIdx(
                                                    347,
                                                ),
                                                access_end: Some(
                                                    TokenIdxRangeEnd(
                                                        TokenIdx(
                                                            376,
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
        ],
    },
)