Ok(
    DefnSheet {
        defns: [
            Type(
                Foreign(
                    AlienTypeDefn {
                        path: TypePath(`core::logic::Prop`, `Foreign`),
                        decl: AlienTypeDecl(
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
                        path: TypePath(`core::logic::LogicAnd`, `Structure`),
                        decl: StructureTypeDecl(
                            Id {
                                value: 3,
                            },
                        ),
                    },
                ),
            ),
            Type(
                Inductive(
                    InductiveTypeDefn {
                        path: TypePath(`core::logic::LogicOr`, `Inductive`),
                        decl: InductiveTypeDecl(
                            Id {
                                value: 1,
                            },
                        ),
                    },
                ),
            ),
        ],
    },
)