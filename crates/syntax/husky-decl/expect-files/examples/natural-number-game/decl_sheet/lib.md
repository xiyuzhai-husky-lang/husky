Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    Inductive(
                        InductiveTypeDecl {
                            path: TypePath(`natural_number_game::Nat`, `Inductive`),
                            ast_idx: 3,
                            expr_region: ExprRegion(
                                Id {
                                    value: 192,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Structure(
                        StructureTypeDecl {
                            path: TypePath(`natural_number_game::OddNat`, `Structure`),
                            ast_idx: 9,
                            expr_region: ExprRegion(
                                Id {
                                    value: 193,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Structure(
                        StructureTypeDecl {
                            path: TypePath(`natural_number_game::EvenNat`, `Structure`),
                            ast_idx: 10,
                            expr_region: ExprRegion(
                                Id {
                                    value: 194,
                                },
                            ),
                            implicit_parameter_decl_list: None,
                        },
                    ),
                ),
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl {
                            ast_idx: 6,
                            impl_block: ImplBlock(
                                Id {
                                    value: 30,
                                },
                            ),
                            impl_token: ImplToken {
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                            implicit_parameter_decl_list: None,
                            ty: TypeExpr {
                                expr: 0,
                            },
                            eol_colon: Ok(
                                EolColonToken {
                                    token_idx: TokenIdx(
                                        11,
                                    ),
                                },
                            ),
                            expr_region: ExprRegion(
                                Id {
                                    value: 195,
                                },
                            ),
                        },
                    ),
                ),
            ),
            Ok(
                AssociatedItem(
                    TypeItem(
                        Memo(
                            TypeMemoDecl {
                                path: Some(
                                    TypeItemPath(
                                        Id {
                                            value: 69,
                                        },
                                    ),
                                ),
                                associated_item: AssociatedItem(
                                    Id {
                                        value: 79,
                                    },
                                ),
                                ast_idx: 0,
                                expr_region: ExprRegion(
                                    Id {
                                        value: 196,
                                    },
                                ),
                                curry_token: Err(
                                    MissingCurry(
                                        TokenIdx(
                                            14,
                                        ),
                                    ),
                                ),
                                output_ty: Ok(
                                    4,
                                ),
                                eol_colon: Err(
                                    MissingEolColon(
                                        TokenIdx(
                                            18,
                                        ),
                                    ),
                                ),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)