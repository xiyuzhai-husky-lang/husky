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
                            return_ty: Ok(
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
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::Err(
                                                Original(
                                                    UnrecognizedIdentifier {
                                                        token_idx: TokenIdx(
                                                            65,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 362,
                                                                },
                                                            ),
                                                        ),
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
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 3,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                            ),
                                        },
                                        Expr::NewBoxList {
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
                                        Expr::RitchieCall {
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
                                                67,
                                            ),
                                            ident: `fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                69,
                                            ),
                                            ident: `major_concave_components`,
                                            entity_path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                72,
                                            ),
                                            ident: `left_coordinate_max`,
                                            entity_path: FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                74,
                                            ),
                                            ident: `left_coordinate_max`,
                                            entity_path: FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
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
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            },
                                            Expr::PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    82,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                            Expr::PrefixOpn {
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
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
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
                                                                value: 195,
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
                                                    ident: `cc`,
                                                    access_start: TokenIdx(
                                                        81,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
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
                            return_ty: Ok(
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
                                            path: RegionPath::Decl(
                                                DeclExprPath::Entity(
                                                    FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    },
                                                    Expr::PrefixOpn {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            82,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`core::num::f32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::PrefixOpn {
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
                                                        entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    EntityPathExpr::Root {
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
                                                                        value: 195,
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
                                                            ident: `cc`,
                                                            access_start: TokenIdx(
                                                                81,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
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
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                89,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            owner: 0,
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
                                        Expr::MethodCall {
                                            self_argument: 1,
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
                                            nonself_arguments: ArenaIdxRange(
                                                2..2,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                95,
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
                                        data: [
                                            InheritedSymbol {
                                                ident: `cc`,
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
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
                            return_ty: Ok(
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
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::Err(
                                                Original(
                                                    UnrecognizedIdentifier {
                                                        token_idx: TokenIdx(
                                                            99,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 362,
                                                                },
                                                            ),
                                                        ),
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
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Function`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                            ),
                                        },
                                        Expr::NewBoxList {
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
                                        Expr::RitchieCall {
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
                                                101,
                                            ),
                                            ident: `fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                103,
                                            ),
                                            ident: `major_concave_components`,
                                            entity_path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                106,
                                            ),
                                            ident: `displacement_downwards`,
                                            entity_path: FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Function`),
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
                            return_ty: Ok(
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
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::Err(
                                                Original(
                                                    UnrecognizedIdentifier {
                                                        token_idx: TokenIdx(
                                                            112,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 362,
                                                                },
                                                            ),
                                                        ),
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
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Function`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                            ),
                                        },
                                        Expr::NewBoxList {
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
                                        Expr::RitchieCall {
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
                                                114,
                                            ),
                                            ident: `fermi_match`,
                                            entity_path: FormPath(`mnist_classifier::fermi::fermi_match`, `Function`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                116,
                                            ),
                                            ident: `major_concave_components`,
                                            entity_path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                119,
                                            ),
                                            ident: `cc_box_heights`,
                                            entity_path: FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Function`),
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
                            return_ty: Ok(
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
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::four::is_four`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::Err(
                                                Original(
                                                    UnrecognizedIdentifier {
                                                        token_idx: TokenIdx(
                                                            126,
                                                        ),
                                                        ident: Identifier(
                                                            Word(
                                                                Id {
                                                                    value: 123,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ),
                                            Expr::PrefixOpn {
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
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::digits::four::is_four`, `Feature`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::EntityPath {
                                            entity_path_expr: 0,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 0,
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
                                        Expr::Literal(
                                            TokenIdx(
                                                133,
                                            ),
                                        ),
                                        Expr::NewBoxList {
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
                                        Expr::Be {
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
                                        Expr::EntityPath {
                                            entity_path_expr: 1,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 5,
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
                                        Expr::Literal(
                                            TokenIdx(
                                                142,
                                            ),
                                        ),
                                        Expr::NewBoxList {
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
                                        Expr::Be {
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
                                        Expr::EntityPath {
                                            entity_path_expr: 2,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 10,
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
                                        Expr::CurrentSymbol {
                                            ident: `eff_holes`,
                                            token_idx: TokenIdx(
                                                153,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 12,
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
                                        Expr::NewBoxList {
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
                                        Expr::Be {
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
                                        Expr::EntityPath {
                                            entity_path_expr: 3,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 17,
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
                                        Expr::Literal(
                                            TokenIdx(
                                                168,
                                            ),
                                        ),
                                        Expr::NewBoxList {
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
                                        Expr::CurrentSymbol {
                                            ident: `down_match`,
                                            token_idx: TokenIdx(
                                                171,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::Be {
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
                                        Expr::CurrentSymbol {
                                            ident: `down_match`,
                                            token_idx: TokenIdx(
                                                177,
                                            ),
                                            current_symbol_idx: 1,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 4,
                                            },
                                        },
                                        Expr::MethodCall {
                                            self_argument: 23,
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
                                            nonself_arguments: ArenaIdxRange(
                                                24..24,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                181,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 24,
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
                                        Expr::EntityPath {
                                            entity_path_expr: 4,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 5,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 26,
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
                                        Expr::Field {
                                            owner: 27,
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
                                        Expr::BinaryOpn {
                                            lopd: 28,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                190,
                                            ),
                                            ropd: 29,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `higher_excess`,
                                            token_idx: TokenIdx(
                                                195,
                                            ),
                                            current_symbol_idx: 3,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 7,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                197,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 31,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                196,
                                            ),
                                            ropd: 32,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `eff_holes`,
                                            token_idx: TokenIdx(
                                                199,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 2,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 34,
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
                                        Expr::Literal(
                                            TokenIdx(
                                                203,
                                            ),
                                        ),
                                        Expr::NewBoxList {
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
                                        Expr::Be {
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
                                        Expr::EntityPath {
                                            entity_path_expr: 6,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                            ),
                                        },
                                        Expr::MethodCall {
                                            self_argument: 39,
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
                                            nonself_arguments: ArenaIdxRange(
                                                40..40,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                213,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                215,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 40,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                214,
                                            ),
                                            ropd: 41,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 7,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 43,
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
                                        Expr::Literal(
                                            TokenIdx(
                                                223,
                                            ),
                                        ),
                                        Expr::NewBoxList {
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
                                        Expr::CurrentSymbol {
                                            ident: `four_match_refine_result`,
                                            token_idx: TokenIdx(
                                                226,
                                            ),
                                            current_symbol_idx: 4,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 9,
                                            },
                                        },
                                        Expr::Be {
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
                                        Expr::EntityPath {
                                            entity_path_expr: 8,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 49,
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
                                        Expr::Literal(
                                            TokenIdx(
                                                234,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 50,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                233,
                                            ),
                                            ropd: 51,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 9,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 10,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 53,
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
                                        Expr::Field {
                                            owner: 54,
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
                                        Expr::BinaryOpn {
                                            lopd: 55,
                                            opr: PureClosed(
                                                Sub,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                241,
                                            ),
                                            ropd: 56,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 11,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 58,
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
                                        Expr::Literal(
                                            TokenIdx(
                                                252,
                                            ),
                                        ),
                                        Expr::NewBoxList {
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
                                        Expr::CurrentSymbol {
                                            ident: `upper_arc`,
                                            token_idx: TokenIdx(
                                                255,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        Expr::Be {
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
                                        Expr::CurrentSymbol {
                                            ident: `upper_arc`,
                                            token_idx: TokenIdx(
                                                259,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        Expr::MethodCall {
                                            self_argument: 64,
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
                                            nonself_arguments: ArenaIdxRange(
                                                65..65,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                263,
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 65,
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
                                        Expr::Literal(
                                            TokenIdx(
                                                267,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 66,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                266,
                                            ),
                                            ropd: 67,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `upper_arc`,
                                            token_idx: TokenIdx(
                                                269,
                                            ),
                                            current_symbol_idx: 6,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 12,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                274,
                                            ),
                                        ),
                                        Expr::Field {
                                            owner: 69,
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
                                        Expr::PrefixOpn {
                                            opr: Minus,
                                            opr_token_idx: TokenIdx(
                                                273,
                                            ),
                                            opd: 70,
                                        },
                                        Expr::BinaryOpn {
                                            lopd: 71,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                272,
                                            ),
                                            ropd: 72,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 12,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                                            ),
                                        },
                                        Expr::Field {
                                            owner: 74,
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
                                        Expr::Literal(
                                            TokenIdx(
                                                280,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 75,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                279,
                                            ),
                                            ropd: 76,
                                        },
                                        Expr::EntityPath {
                                            entity_path_expr: 13,
                                            entity_path: Some(
                                                FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                288,
                                            ),
                                        ),
                                        Expr::MethodCall {
                                            self_argument: 78,
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
                                            nonself_arguments: ArenaIdxRange(
                                                79..80,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                289,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `a`,
                                            token_idx: TokenIdx(
                                                291,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 14,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                293,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 81,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                292,
                                            ),
                                            ropd: 82,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `a`,
                                            token_idx: TokenIdx(
                                                295,
                                            ),
                                            current_symbol_idx: 7,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 14,
                                            },
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                297,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 84,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                296,
                                            ),
                                            ropd: 85,
                                        },
                                        Expr::Err(
                                            Original(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        298,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 123,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            Original(
                                                UnresolvedSubentity {
                                                    token_idx: TokenIdx(
                                                        300,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 139,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 87,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                299,
                                            ),
                                            ropd: 88,
                                        },
                                        Expr::Err(
                                            Original(
                                                UnrecognizedIdentifier {
                                                    token_idx: TokenIdx(
                                                        301,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 123,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::Err(
                                            Original(
                                                UnresolvedSubentity {
                                                    token_idx: TokenIdx(
                                                        303,
                                                    ),
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 139,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 90,
                                            opr: ScopeResolution,
                                            opr_token_idx: TokenIdx(
                                                302,
                                            ),
                                            ropd: 91,
                                        },
                                        Expr::Block {
                                            stmts: ArenaIdxRange(
                                                14..25,
                                            ),
                                        },
                                    ],
                                },
                                entity_path_expr_arena: Arena {
                                    data: [
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                129,
                                            ),
                                            ident: `left_components`,
                                            entity_path: FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                138,
                                            ),
                                            ident: `left_components`,
                                            entity_path: FormPath(`mnist_classifier::digits::four::left_components`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                149,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                164,
                                            ),
                                            ident: `components_max_downwards`,
                                            entity_path: FormPath(`mnist_classifier::digits::four::components_max_downwards`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                187,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                191,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                209,
                                            ),
                                            ident: `major_concave_components`,
                                            entity_path: FormPath(`mnist_classifier::major::major_concave_components`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                219,
                                            ),
                                            ident: `components_max_heights`,
                                            entity_path: FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                230,
                                            ),
                                            ident: `components_max_heights`,
                                            entity_path: FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                238,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                242,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                248,
                                            ),
                                            ident: `components_max_heights`,
                                            entity_path: FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                276,
                                            ),
                                            ident: `components_max_heights`,
                                            entity_path: FormPath(`mnist_classifier::digits::four::components_max_heights`, `Feature`),
                                        },
                                        EntityPathExpr::Root {
                                            token_idx: TokenIdx(
                                                284,
                                            ),
                                            ident: `major_connected_component`,
                                            entity_path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    208,
                                                ),
                                            },
                                            condition: Ok(
                                                42,
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
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    225,
                                                ),
                                            },
                                            condition: Ok(
                                                48,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    229,
                                                ),
                                            },
                                            condition: Ok(
                                                52,
                                            ),
                                        },
                                        Stmt::Let {
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
                                        Stmt::Let {
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
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    254,
                                                ),
                                            },
                                            condition: Ok(
                                                63,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    258,
                                                ),
                                            },
                                            condition: Ok(
                                                68,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    268,
                                                ),
                                            },
                                            condition: Ok(
                                                73,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    275,
                                                ),
                                            },
                                            condition: Ok(
                                                77,
                                            ),
                                        },
                                        Stmt::Let {
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
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    290,
                                                ),
                                            },
                                            condition: Ok(
                                                83,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    294,
                                                ),
                                            },
                                            condition: Ok(
                                                86,
                                            ),
                                        },
                                        Stmt::Eval {
                                            expr_idx: 89,
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    128,
                                                ),
                                            },
                                            condition: Ok(
                                                4,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    137,
                                                ),
                                            },
                                            condition: Ok(
                                                9,
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
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    152,
                                                ),
                                            },
                                            condition: Ok(
                                                16,
                                            ),
                                        },
                                        Stmt::Let {
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
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    170,
                                                ),
                                            },
                                            condition: Ok(
                                                22,
                                            ),
                                        },
                                        Stmt::Let {
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
                                        Stmt::Let {
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
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    194,
                                                ),
                                            },
                                            condition: Ok(
                                                33,
                                            ),
                                        },
                                        Stmt::IfElse {
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
                                        Stmt::Eval {
                                            expr_idx: 92,
                                        },
                                    ],
                                },
                                pattern_expr_region: PatternExprRegion {
                                    pattern_expr_arena: Arena {
                                        data: [
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        136,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        145,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `eff_holes`,
                                                    token_idx: TokenIdx(
                                                        147,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        160,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `down_match`,
                                                    token_idx: TokenIdx(
                                                        162,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        173,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `down_match_dp_y`,
                                                    token_idx: TokenIdx(
                                                        175,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `higher_excess`,
                                                    token_idx: TokenIdx(
                                                        185,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `none`,
                                                    token_idx: TokenIdx(
                                                        206,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `four_match_refine_result`,
                                                    token_idx: TokenIdx(
                                                        217,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        228,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `higher_excess`,
                                                    token_idx: TokenIdx(
                                                        236,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `upper_arc`,
                                                    token_idx: TokenIdx(
                                                        246,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `some`,
                                                    token_idx: TokenIdx(
                                                        257,
                                                    ),
                                                },
                                                liason: None,
                                            },
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `a`,
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
                                                            value: 380,
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
                                                            value: 380,
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
                                                            value: 156,
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
                                                            value: 377,
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
                                                            value: 431,
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
                                                            value: 380,
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
                                                            value: 432,
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
                                                            value: 433,
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
                                                            value: 377,
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
                                                            value: 434,
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
                                                            value: 380,
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
                                                            value: 433,
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
                                                            value: 435,
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
                                                            value: 380,
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
                                                            value: 27,
                                                        },
                                                    ),
                                                ),
                                                14,
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
                                                ident: `eff_holes`,
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 2,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `down_match`,
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 4,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `down_match_dp_y`,
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 6,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `higher_excess`,
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 7,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `four_match_refine_result`,
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 9,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `higher_excess`,
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 11,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `upper_arc`,
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 12,
                                                },
                                            },
                                            CurrentSymbol {
                                                ident: `a`,
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
                                                variant: CurrentSymbolVariant::LetVariable {
                                                    pattern_symbol_idx: 14,
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
                                        expr: 93,
                                    },
                                ],
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
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            },
                                            Expr::PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    309,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                            Expr::PrefixOpn {
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
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    310,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    314,
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
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `cc`,
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
                                                                value: 195,
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
                                                    ident: `cc`,
                                                    access_start: TokenIdx(
                                                        308,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
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
                            return_ty: Ok(
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
                                            path: RegionPath::Decl(
                                                DeclExprPath::Entity(
                                                    FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    },
                                                    Expr::PrefixOpn {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            309,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`core::num::f32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::PrefixOpn {
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
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            310,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            314,
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
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `cc`,
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
                                                                        value: 195,
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
                                                            ident: `cc`,
                                                            access_start: TokenIdx(
                                                                308,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
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
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::digits::four::displacement_downwards`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                319,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::MethodCall {
                                            self_argument: 0,
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
                                            nonself_arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                323,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                325,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 2,
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
                                        Expr::Literal(
                                            TokenIdx(
                                                329,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 3,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                328,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                330,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 6,
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
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    324,
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
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `dp`,
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
                                                            value: 315,
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
                                        data: [
                                            InheritedSymbol {
                                                ident: `cc`,
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: `dp`,
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
                                                variant: CurrentSymbolVariant::LetVariable {
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
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            },
                                            Expr::PrefixOpn {
                                                opr: Ref,
                                                opr_token_idx: TokenIdx(
                                                    338,
                                                ),
                                                opd: 0,
                                            },
                                            Expr::EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::f32`, `Alien`),
                                                ),
                                            },
                                            Expr::PrefixOpn {
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
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    339,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    343,
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
                                                PatternExpr::Identifier {
                                                    ident_token: IdentifierToken {
                                                        ident: `cc`,
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
                                                                value: 195,
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
                                                    ident: `cc`,
                                                    access_start: TokenIdx(
                                                        337,
                                                    ),
                                                    access_end: None,
                                                    variant: CurrentSymbolVariant::RegularParameter {
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
                            return_ty: Ok(
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
                                            path: RegionPath::Decl(
                                                DeclExprPath::Entity(
                                                    FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Function`),
                                                ),
                                            ),
                                            expr_arena: Arena {
                                                data: [
                                                    Expr::EntityPath {
                                                        entity_path_expr: 0,
                                                        entity_path: Some(
                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        ),
                                                    },
                                                    Expr::PrefixOpn {
                                                        opr: Ref,
                                                        opr_token_idx: TokenIdx(
                                                            338,
                                                        ),
                                                        opd: 0,
                                                    },
                                                    Expr::EntityPath {
                                                        entity_path_expr: 1,
                                                        entity_path: Some(
                                                            TypePath(`core::num::f32`, `Alien`),
                                                        ),
                                                    },
                                                    Expr::PrefixOpn {
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
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            339,
                                                        ),
                                                        ident: `ConcaveComponent`,
                                                        entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    },
                                                    EntityPathExpr::Root {
                                                        token_idx: TokenIdx(
                                                            343,
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
                                                        PatternExpr::Identifier {
                                                            ident_token: IdentifierToken {
                                                                ident: `cc`,
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
                                                                        value: 195,
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
                                                            ident: `cc`,
                                                            access_start: TokenIdx(
                                                                337,
                                                            ),
                                                            access_end: None,
                                                            variant: CurrentSymbolVariant::RegularParameter {
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
                                    DefnExprPath::Entity(
                                        FormPath(`mnist_classifier::digits::four::cc_box_heights`, `Function`),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                348,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::MethodCall {
                                            self_argument: 0,
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
                                            nonself_arguments: ArenaIdxRange(
                                                1..1,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                352,
                                            ),
                                        },
                                        Expr::CurrentSymbol {
                                            ident: `dp`,
                                            token_idx: TokenIdx(
                                                354,
                                            ),
                                            current_symbol_idx: 0,
                                            current_symbol_kind: CurrentSymbolKind::LetVariable {
                                                pattern_symbol_idx: 0,
                                            },
                                        },
                                        Expr::Field {
                                            owner: 2,
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
                                        Expr::Literal(
                                            TokenIdx(
                                                358,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 3,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                357,
                                            ),
                                            ropd: 4,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                360,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            owner: 6,
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
                                        Expr::MethodCall {
                                            self_argument: 7,
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
                                            nonself_arguments: ArenaIdxRange(
                                                8..8,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                366,
                                            ),
                                        },
                                        Expr::Literal(
                                            TokenIdx(
                                                368,
                                            ),
                                        ),
                                        Expr::BinaryOpn {
                                            lopd: 8,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            opr_token_idx: TokenIdx(
                                                367,
                                            ),
                                            ropd: 9,
                                        },
                                        Expr::InheritedSymbol {
                                            ident: `cc`,
                                            token_idx: TokenIdx(
                                                369,
                                            ),
                                            inherited_symbol_idx: 0,
                                            inherited_symbol_kind: InheritedSymbolKind::RegularParameter,
                                        },
                                        Expr::Field {
                                            owner: 11,
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
                                        Expr::MethodCall {
                                            self_argument: 12,
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
                                            nonself_arguments: ArenaIdxRange(
                                                13..13,
                                            ),
                                            rpar_token_idx: TokenIdx(
                                                375,
                                            ),
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
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    353,
                                                ),
                                            },
                                            condition: Ok(
                                                5,
                                            ),
                                        },
                                        Stmt::Require {
                                            require_token: RequireToken {
                                                token_idx: TokenIdx(
                                                    359,
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
                                            PatternExpr::Identifier {
                                                ident_token: IdentifierToken {
                                                    ident: `dp`,
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
                                                            value: 315,
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
                                        data: [
                                            InheritedSymbol {
                                                ident: `cc`,
                                                parent_symbol_idx: Current(
                                                    0,
                                                ),
                                                kind: InheritedSymbolKind::RegularParameter,
                                            },
                                        ],
                                    },
                                    current_symbol_arena: Arena {
                                        data: [
                                            CurrentSymbol {
                                                ident: `dp`,
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
                                                variant: CurrentSymbolVariant::LetVariable {
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
                        body: Ok(
                            14,
                        ),
                    },
                ),
            ),
        ],
    },
)