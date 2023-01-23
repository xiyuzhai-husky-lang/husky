[
    Ok(
        Type(
            Foreign(
                AlienTypeSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        parameters: [],
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
                        parameters: [
                            ImplicitParameterSignature {
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
                        parameters: [
                            ImplicitParameterSignature {
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