Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    Inductive(
                        InductiveTypeDecl {
                            path: TypePath(`natural_number_game::Nat`, `Inductive`),
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Structure(
                        StructureTypeDecl {
                            path: TypePath(`natural_number_game::OddNat`, `Structure`),
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Structure(
                        StructureTypeDecl {
                            path: TypePath(`natural_number_game::EvenNat`, `Structure`),
                        },
                    ),
                ),
            ),
            Ok(
                ImplBlock(
                    TypeImplBlock(
                        TypeImplBlockDecl,
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
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)