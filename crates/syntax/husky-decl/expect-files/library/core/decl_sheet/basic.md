Ok(
    DeclSheet {
        decls: [
            (
                TypePath(`core::basic::bool`, `Foreign`),
                Ok(
                    Type(
                        Foreign(
                            AlienTypeDecl {
                                path: TypePath(`core::basic::bool`, `Foreign`),
                            },
                        ),
                    ),
                ),
            ),
            (
                TypePath(`core::basic::Trait`, `Structure`),
                Ok(
                    Type(
                        Structure(
                            StructureTypeDecl {
                                path: TypePath(`core::basic::Trait`, `Structure`),
                            },
                        ),
                    ),
                ),
            ),
            (
                TypePath(`core::basic::Module`, `Structure`),
                Ok(
                    Type(
                        Structure(
                            StructureTypeDecl {
                                path: TypePath(`core::basic::Module`, `Structure`),
                            },
                        ),
                    ),
                ),
            ),
        ],
    },
)