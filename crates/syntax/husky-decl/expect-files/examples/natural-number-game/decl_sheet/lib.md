Ok(
    DeclSheet {
        decls: [
            (
                TypePath(`natural_number_game::Nat`, `Inductive`),
                Ok(
                    Type(
                        Inductive(
                            InductiveTypeDecl {
                                path: TypePath(`natural_number_game::Nat`, `Inductive`),
                            },
                        ),
                    ),
                ),
            ),
            (
                TypePath(`natural_number_game::OddNat`, `Structure`),
                Ok(
                    Type(
                        Structure(
                            StructureTypeDecl {
                                path: TypePath(`natural_number_game::OddNat`, `Structure`),
                            },
                        ),
                    ),
                ),
            ),
            (
                TypePath(`natural_number_game::EvenNat`, `Structure`),
                Ok(
                    Type(
                        Structure(
                            StructureTypeDecl {
                                path: TypePath(`natural_number_game::EvenNat`, `Structure`),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)