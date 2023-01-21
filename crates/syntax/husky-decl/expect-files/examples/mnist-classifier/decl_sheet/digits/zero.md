Ok(
    DeclSheet {
        decls: [
            Ok(
                Form(
                    Feature(
                        FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::zero::open_one_match`, `Feature`),
                            ast_idx: 33,
                            expr_region: ExprRegion(
                                Id {
                                    value: 100,
                                },
                            ),
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Function(
                        FunctionDecl {
                            path: FormPath(`mnist_classifier::digits::zero::almost_closed`, `Function`),
                            ast_idx: 34,
                            expr_region: ExprRegion(
                                Id {
                                    value: 101,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        79,
                                    ),
                                },
                                parameters: [
                                    ParameterDecl {
                                        pattern: ParameterDeclPattern {
                                            pattern_expr_idx: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
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
                                3,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        88,
                                    ),
                                },
                            ),
                        },
                    ),
                ),
            ),
            Ok(
                Form(
                    Feature(
                        FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::zero::is_zero`, `Feature`),
                            ast_idx: 35,
                            expr_region: ExprRegion(
                                Id {
                                    value: 102,
                                },
                            ),
                        },
                    ),
                ),
            ),
        ],
    },
)