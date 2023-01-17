Ok(
    DeclSheet {
        decls: [
            Ok(
                Trait(
                    TraitDecl {
                        path: TraitPath(`std::ops::Add`),
                        ast_idx: 3,
                        expr_page: ExprPage(
                            Id {
                                value: 41,
                            },
                        ),
                        implicit_parameter_decl_list: Some(
                            ImplicitParameterDeclList {
                                langle: LeftAngleBracketOrLessThanToken {
                                    token_idx: TokenIdx(
                                        8,
                                    ),
                                },
                                decls: [
                                    ImplicitParameterDecl {
                                        ident: IdentifierToken {
                                            ident: Identifier(
                                                Word(
                                                    Id {
                                                        value: 33,
                                                    },
                                                ),
                                            ),
                                            token_idx: TokenIdx(
                                                9,
                                            ),
                                        },
                                        traits: None,
                                    },
                                ],
                                commas: [],
                                rangle: RightAngleBracketToken {
                                    token_idx: TokenIdx(
                                        10,
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