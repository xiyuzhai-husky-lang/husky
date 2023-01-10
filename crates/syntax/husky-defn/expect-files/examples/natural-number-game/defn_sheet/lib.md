Ok(
    DefnSheet {
        defns: [
            Type(
                Inductive(
                    InductiveTypeDefn {
                        path: TypePath(`natural_number_game::Nat`, `Inductive`),
                    },
                ),
            ),
            Type(
                Structure(
                    StructureTypeDefn {
                        path: TypePath(`natural_number_game::OddNat`, `Structure`),
                    },
                ),
            ),
            Type(
                Structure(
                    StructureTypeDefn {
                        path: TypePath(`natural_number_game::EvenNat`, `Structure`),
                    },
                ),
            ),
            ImplBlock(
                TypeImplBlock(
                    TypeImplBlockDecl,
                ),
            ),
        ],
    },
)