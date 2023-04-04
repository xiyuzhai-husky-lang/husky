[
    Ok(
        Signature::Type(
            TypeSignature::Foreign(
                ExternTypeSignature {
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
                        data: [
                            ImplicitParameterSignature {
                                annotated_variance: None,
                                symbol: RawTermSymbol(
                                    Id {
                                        value: 1,
                                    },
                                ),
                                traits: [],
                            },
                            ImplicitParameterSignature {
                                annotated_variance: None,
                                symbol: RawTermSymbol(
                                    Id {
                                        value: 2,
                                    },
                                ),
                                traits: [],
                            },
                        ],
                    },
                },
            ),
        ),
    ),
    Ok(
        Signature::Type(
            TypeSignature::Inductive(
                InductiveTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [
                            ImplicitParameterSignature {
                                annotated_variance: None,
                                symbol: RawTermSymbol(
                                    Id {
                                        value: 1,
                                    },
                                ),
                                traits: [],
                            },
                            ImplicitParameterSignature {
                                annotated_variance: None,
                                symbol: RawTermSymbol(
                                    Id {
                                        value: 2,
                                    },
                                ),
                                traits: [],
                            },
                        ],
                    },
                },
            ),
        ),
    ),
]