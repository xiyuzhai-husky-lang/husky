Ok(
    DeclSheet {
        decls: [
            Ok(
                Form(
                    Feature(
                        FeatureDecl {
                            path: FormPath(`mnist_classifier::digits::eight::upper_mouth_match`, `Feature`),
                            ast_idx: 28,
                            expr_page: ExprPage(
                                Id {
                                    value: 64,
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
                            path: FormPath(`mnist_classifier::digits::eight::is_eight`, `Feature`),
                            ast_idx: 29,
                            expr_page: ExprPage(
                                Id {
                                    value: 65,
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
                            path: FormPath(`mnist_classifier::digits::eight::big_mouth`, `Function`),
                            ast_idx: 30,
                            expr_page: ExprPage(
                                Id {
                                    value: 66,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                            parameter_decl_list: ParameterDeclList {
                                lpar: LeftParenthesisToken {
                                    token_idx: TokenIdx(
                                        156,
                                    ),
                                },
                                decls: [
                                    ParameterDecl {
                                        pattern: ParameterPattern {
                                            pattern_expr_idx: 0,
                                            variables: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                        colon: ColonToken {
                                            token_idx: TokenIdx(
                                                158,
                                            ),
                                        },
                                        ty: 1,
                                    },
                                ],
                                commas: [],
                                rpar: RightParenthesisToken {
                                    token_idx: TokenIdx(
                                        161,
                                    ),
                                },
                            },
                            curry_token: Ok(
                                CurryToken {
                                    token_idx: TokenIdx(
                                        162,
                                    ),
                                },
                            ),
                            output_ty: Ok(
                                3,
                            ),
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        165,
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