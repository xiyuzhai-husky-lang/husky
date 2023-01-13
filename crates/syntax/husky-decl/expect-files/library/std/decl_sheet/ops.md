Ok(
    DeclSheet {
        decls: [
            Ok(
                Trait(
                    TraitDecl {
                        path: TraitPath(`std::ops::Add`),
                        ast_idx: 3,
                        expr_sheet: ExprSheet(
                            Id {
                                value: 39,
                            },
                        ),
                        implicit_parameter_decl_list: Some(
                            ImplicitParameterDeclList {
                                langle: LeftAngleBracketToken {
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
                                                        value: 32,
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