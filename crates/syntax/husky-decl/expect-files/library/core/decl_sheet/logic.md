Ok(
    DeclSheet {
        decls: [
            Ok(
                Type(
                    Structure(
                        StructureTypeDecl {
                            path: TypePath(`core::logic::LogicAnd`, `Structure`),
                        },
                    ),
                ),
            ),
            Ok(
                Type(
                    Inductive(
                        InductiveTypeDecl {
                            path: TypePath(`core::logic::LogicOr`, `Inductive`),
                        },
                    ),
                ),
            ),
        ],
    },
)