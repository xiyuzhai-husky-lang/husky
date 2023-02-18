Ok(
    DeclSheet {
        decls: [
            Ok(
                Form(
                    FormDecl::Feature(
                        FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Feature`),
                            ast_idx: 34,
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
                                            FormPath(`mnist_classifier::digits::three::three_fermi_match`, `Feature`),
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
                    ),
                ),
            ),
            Ok(
                Form(
                    FormDecl::Feature(
                        FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::three::is_three`, `Feature`),
                            ast_idx: 35,
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        81,
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
                                        84,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::three::is_three`, `Feature`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            Expr::Err(
                                                Original(
                                                    UnrecognizedIdentifier {
                                                        token_idx: TokenIdx(
                                                            83,
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
                                                    82,
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
                    ),
                ),
            ),
            Ok(
                Form(
                    FormDecl::Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::three::uparc`, `Function`),
                            ast_idx: 36,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::three::uparc`, `Function`),
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
                                                    214,
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
                                                    218,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    215,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    219,
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
                                                            212,
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
                                                        213,
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
                                        211,
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
                                                213,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        216,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        217,
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
                                        220,
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    FormDecl::Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::three::downarc`, `Function`),
                            ast_idx: 37,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::three::downarc`, `Function`),
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
                                                    248,
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
                                                    252,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    249,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    253,
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
                                                            246,
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
                                                        247,
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
                                        245,
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
                                                247,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        250,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        251,
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
                                        254,
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    FormDecl::Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::three::back`, `Function`),
                            ast_idx: 38,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: RegionPath::Decl(
                                        DeclExprPath::Entity(
                                            FormPath(`mnist_classifier::digits::three::back`, `Function`),
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
                                                    282,
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
                                                    286,
                                                ),
                                                opd: 2,
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    283,
                                                ),
                                                ident: `ConcaveComponent`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            },
                                            EntityPathExpr::Root {
                                                token_idx: TokenIdx(
                                                    287,
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
                                                            280,
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
                                                        281,
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
                                        279,
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
                                                281,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        284,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        285,
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
                                        288,
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
        ],
    },
)