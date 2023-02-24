[
    Ok(
        Signature::Type(
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
        Signature::Type(
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
        Signature::Type(
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
        Signature::Impl(
            ImplSignature::TypeImpl(
                TypeImplSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    ty: Term(`natural_number_game::Nat`),
                },
            ),
        ),
    ),
    Err(
        OutputTypeTermError,
    ),
]