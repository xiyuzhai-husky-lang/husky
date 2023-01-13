Ok(
    DeclSheet {
        decls: [
            Ok(
                Trait(
                    TraitDecl {
                        path: TraitPath(`core::ops::Add`),
                        ast_idx: 3,
                        expr_sheet: ExprSheet(
                            Id {
                                value: 38,
                            },
                        ),
                        implicit_parameter_decl_list: Some(
                            ImplicitParameterDeclList {
                                langle: LeftAngleBracketOrLessThanToken {
                                    token_idx: TokenIdx(
                                        10,
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
                                                11,
                                            ),
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