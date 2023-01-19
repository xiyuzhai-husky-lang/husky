Ok(
    DefnSheet {
        defns: [
            Type(
                Foreign(
                    AlienTypeDefn {
                        path: TypePath(`core::basic::bool`, `Alien`),
                        decl: AlienTypeDecl(
                            Id {
                                value: 1,
                            },
                        ),
                    },
                ),
            ),
            Type(
                Structure(
                    StructureTypeDefn {
                        path: TypePath(`core::basic::Trait`, `Structure`),
                        decl: StructureTypeDecl(
                            Id {
                                value: 1,
                            },
                        ),
                    },
                ),
            ),
            Type(
                Structure(
                    StructureTypeDefn {
                        path: TypePath(`core::basic::Module`, `Structure`),
                        decl: StructureTypeDecl(
                            Id {
                                value: 2,
                            },
                        ),
                    },
                ),
            ),
        ],
    },
)