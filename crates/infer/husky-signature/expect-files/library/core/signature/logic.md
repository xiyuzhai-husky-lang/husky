[
    Ok(
        Type(
            Foreign(
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
            Structure(
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
            Inductive(
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