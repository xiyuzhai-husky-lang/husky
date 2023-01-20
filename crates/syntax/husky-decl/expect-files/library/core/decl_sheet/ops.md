Ok(
    DeclSheet {
        decls: [
            Ok(
                Trait(
                    TraitDecl {
                        path: TraitPath(`core::ops::Add`),
                        ast_idx: 3,
                        expr_page: ExprPage(
                            Id {
                                value: 39,
                            },
                        ),
                        implicit_parameter_decl_list: Some(
                            ImplicitParameterDeclList {
                                langle: LeftAngleBracketOrLessThanToken {
                                    token_idx: TokenIdx(
                                        10,
                                    ),
                                },
                                implicit_parameters: [
                                    ImplicitParameterDecl {
                                        pattern: ImplicitParameterDeclPattern {
                                            symbol: 0,
                                            variant: Type0 {
                                                ident_token: IdentifierToken {
                                                    ident: Identifier(
                                                        Word(
                                                            Id {
                                                                value: 33,
                                                            },
                                                        ),
                                                    ),
                                                    token_idx: TokenIdx(
                                                        11,
                                                    ),
                                                },
                                            },
                                        },
                                        traits: None,
                                    },
                                ],
                                commas: [],
                                rangle: RightAngleBracketToken {
                                    token_idx: TokenIdx(
                                        12,
                                    ),
                                },
                            },
                        ),
                    },
                ),
            ),
        ],
    },
)