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
        Signature::ImplBlock(
            ImplBlockSignature::TypeImplBlock(
                TypeImplBlockSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [],
                    },
                    ty: Term(`Nat`),
                },
            ),
        ),
    ),
    Err(
        OutputTypeTermError,
    ),
]