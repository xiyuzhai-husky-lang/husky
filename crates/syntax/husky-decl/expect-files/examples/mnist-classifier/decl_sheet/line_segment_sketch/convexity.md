Ok(
    DeclSheet {
        decls: [
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                            ast_idx: 22,
                            expr_region: ExprRegion {
                                data: ExprRegionData {
                                    parent: None,
                                    path: Decl(
                                        Entity(
                                            FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
                                        ),
                                    ),
                                    expr_arena: Arena {
                                        data: [
                                            EntityPath {
                                                entity_path_expr: 0,
                                                entity_path: Some(
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                ),
                                            },
                                            EntityPath {
                                                entity_path_expr: 1,
                                                entity_path: Some(
                                                    TypePath(`core::num::i32`, `Alien`),
                                                ),
                                            },
                                            EntityPath {
                                                entity_path_expr: 2,
                                                entity_path: Some(
                                                    TypePath(`core::basic::bool`, `Alien`),
                                                ),
                                            },
                                        ],
                                    },
                                    entity_path_expr_arena: Arena {
                                        data: [
                                            Root {
                                                token_idx: TokenIdx(
                                                    24,
                                                ),
                                                ident: `LineSegmentSketch`,
                                                entity_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    28,
                                                ),
                                                ident: `i32`,
                                                entity_path: TypePath(`core::num::i32`, `Alien`),
                                            },
                                            Root {
                                                token_idx: TokenIdx(
                                                    31,
                                                ),
                                                ident: `bool`,
                                                entity_path: TypePath(`core::basic::bool`, `Alien`),
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
                                                        ident: `line_segment_sketch`,
                                                        token_idx: TokenIdx(
                                                            22,
                                                        ),
                                                    },
                                                    liason: None,
                                                },
                                                Identifier {
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
                                                                value: 96,
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
                                                                value: 334,
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
                                            data: [],
                                        },
                                        current_symbol_arena: Arena {
                                            data: [
                                                CurrentSymbol {
                                                    ident: `line_segment_sketch`,
                                                    access_start: TokenIdx(
                                                        23,
                                                    ),
                                                    access_end: None,
                                                    variant: Parameter {
                                                        pattern_symbol: 0,
                                                    },
                                                },
                                                CurrentSymbol {
                                                    ident: `index`,
                                                    access_start: TokenIdx(
                                                        27,
                                                    ),
                                                    access_end: None,
                                                    variant: Parameter {
                                                        pattern_symbol: 1,
                                                    },
                                                },
                                            ],
                                        },
                                        allow_self_type: False,
                                        allow_self_value: False,
                                        ty_constraints: [
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
                                            kind: OutputType,
                                            expr: 2,
                                        },
                                    ],
                                },
                            },
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        21,
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
                                                23,
                                            ),
                                        },
                                        ty: 0,
                                    },
                                    RegularParameterDeclPattern {
                                        pattern: 1,
                                        variables: ArenaIdxRange(
                                            1..2,
                                        ),
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                27,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [
                                    CommaToken {
                                        token_idx: TokenIdx(
                                            25,
                                        ),
                                    },
                                ],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        29,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        30,
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
                                        32,
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