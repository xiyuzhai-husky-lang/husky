Ok(
    DeclSheet {
        decls: [
            Ok(
                Form(
                    Feature(
                        FeatureDecl {
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
                            expr_region: ExprRegion(
                                Id {
                                    value: 92,
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
                                    value: 93,
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
                    ),
                ),
            ),
            Ok(
                Form(
                    Feature(
                        FeatureDecl {
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
                            expr_region: ExprRegion(
                                Id {
                                    value: 94,
                                },
                            ),
                        },
                    ),
                ),
            ),
        ],
    },
)