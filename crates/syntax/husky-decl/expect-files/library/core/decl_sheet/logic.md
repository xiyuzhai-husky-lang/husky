Ok(
    DeclSheet {
        decls: [
            (
                TypePath(`core::logic::LogicAnd`, `Structure`),
                Ok(
                    Type(
                        Structure(
                            StructureTypeDecl {
                                path: TypePath(`core::logic::LogicAnd`, `Structure`),
                            },
                        ),
                    ),
                ),
            ),
            (
                TypePath(`core::logic::LogicOr`, `Inductive`),
                Ok(
                    Type(
                        Inductive(
                            InductiveTypeDecl {
                                path: TypePath(`core::logic::LogicOr`, `Inductive`),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)