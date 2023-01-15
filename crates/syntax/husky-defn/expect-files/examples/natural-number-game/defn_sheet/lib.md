Ok(
    DefnSheet {
        defns: [
            Type(
                Inductive(
                    InductiveTypeDefn {
                        path: TypePath(`natural_number_game::Nat`, `Inductive`),
                        decl: InductiveTypeDecl(
                            Id {
                                value: 2,
                            },
                        ),
                    },
                ),
            ),
            Type(
                Structure(
                    StructureTypeDefn {
                        path: TypePath(`natural_number_game::OddNat`, `Structure`),
                        decl: StructureTypeDecl(
                            Id {
                                value: 4,
                            },
                        ),
                    },
                ),
            ),
            Type(
                Structure(
                    StructureTypeDefn {
                        path: TypePath(`natural_number_game::EvenNat`, `Structure`),
                        decl: StructureTypeDecl(
                            Id {
                                value: 5,
                            },
                        ),
                    },
                ),
            ),
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
                        ty: 0,
                        eol_colon: Ok(
                            EolColonToken {
                                token_idx: TokenIdx(
                                    11,
                                ),
                            },
                        ),
                        expr_sheet: ExprSheet(
                            Id {
                                value: 353,
                            },
                        ),
                    },
                ),
            ),
            AssociatedItem(
                TypeItem(
                    Memo(
                        TypeMemoDefn {
                            path: Some(
                                TypeItemPath(
                                    Id {
                                        value: 69,
                                    },
                                ),
                            ),
                            decl: TypeMemoDecl(
                                Id {
                                    value: 24,
                                },
                            ),
                            expr_sheet: ExprSheet(
                                Id {
                                    value: 355,
                                },
                            ),
                            body: Err(
                                MissingBody,
                            ),
                        },
                    ),
                ),
            ),
        ],
    },
)