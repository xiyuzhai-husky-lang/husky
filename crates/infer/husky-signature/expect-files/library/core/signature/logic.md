[
    Ok(
        Type(
            TypeSignature::Foreign(
                AlienTypeSignature {
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
                        data: [
                            ImplicitParameterSignature {
                                annotated_variance: None,
                                term_symbol: TermSymbol(
                                    Id {
                                        value: 1,
                                    },
                                ),
                                ty: Application(
                                    TermApplication(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                                traits: [],
                            },
                            ImplicitParameterSignature {
                                annotated_variance: None,
                                term_symbol: TermSymbol(
                                    Id {
                                        value: 2,
                                    },
                                ),
                                ty: Application(
                                    TermApplication(
                                        Id {
                                            value: 1,
                                        },
                                    ),
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
        Type(
            TypeSignature::Inductive(
                InductiveTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        data: [
                            ImplicitParameterSignature {
                                annotated_variance: None,
                                term_symbol: TermSymbol(
                                    Id {
                                        value: 1,
                                    },
                                ),
                                ty: Application(
                                    TermApplication(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                                traits: [],
                            },
                            ImplicitParameterSignature {
                                annotated_variance: None,
                                term_symbol: TermSymbol(
                                    Id {
                                        value: 2,
                                    },
                                ),
                                ty: Application(
                                    TermApplication(
                                        Id {
                                            value: 1,
                                        },
                                    ),
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