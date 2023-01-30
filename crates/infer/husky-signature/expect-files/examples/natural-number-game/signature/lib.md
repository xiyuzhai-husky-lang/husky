[
    Ok(
        Type(
            TypeSignature::Inductive(
                InductiveTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                },
            ),
        ),
    ),
    Ok(
        Type(
            TypeSignature::Structure(
                StructureTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                },
            ),
        ),
    ),
    Ok(
        Type(
            TypeSignature::Structure(
                StructureTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                },
            ),
        ),
    ),
    Ok(
        ImplBlock(
            TypeImplBlock(
                TypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    ty: Entity(
                        TypePath(`natural_number_game::Nat`, `Inductive`),
                    ),
                },
            ),
        ),
    ),
    Err(
        OutputTypeTermError,
    ),
]