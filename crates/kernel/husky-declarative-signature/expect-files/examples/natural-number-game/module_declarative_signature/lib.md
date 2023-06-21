[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`natural_number_game::Nat`, `Inductive`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Inductive(
                        InductiveDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`natural_number_game::OddNat`, `Structure`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Structure(
                        StructureDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`natural_number_game::EvenNat`, `Structure`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Structure(
                        StructureDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::ImplBlock(
            TypeImplBlock(
                TypeImplBlockPath(
                    Id {
                        value: 43,
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        implicit_parameters: ImplicitParameterDeclarativeSignatures {
                            data: [],
                        },
                        ty: DeclarativeTerm(`natural_number_game::Nat`),
                    },
                ),
            ),
        ),
    ),
]