[
    Type(
        Foreign(
            AlienTypeSignature {
                implicit_parameters: ImplicitParameterSignatures {
                    parameters: [],
                },
                term_sheet: SignatureTermSheet {
                    term_symbol_page: TermSymbolPage {
                        registry: TermSymbolRegistry {
                            tys: [],
                        },
                        inherited_symbol_terms: [],
                        current_symbol_terms: [],
                    },
                },
            },
        ),
    ),
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
                            ty: Success(
                                Application(
                                    TermApplication(
                                        Id {
                                            value: 1,
                                        },
                                    ),
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
                            ty: Success(
                                Application(
                                    TermApplication(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                            traits: [],
                        },
                    ],
                },
                term_sheet: SignatureTermSheet {
                    term_symbol_page: TermSymbolPage {
                        registry: TermSymbolRegistry {
                            tys: [
                                Ok(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                    ),
                                ),
                                Ok(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                    ),
                                ),
                            ],
                        },
                        inherited_symbol_terms: [],
                        current_symbol_terms: [
                            TermSymbol(
                                Id {
                                    value: 1,
                                },
                            ),
                            TermSymbol(
                                Id {
                                    value: 2,
                                },
                            ),
                        ],
                    },
                },
            },
        ),
    ),
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
                            ty: Success(
                                Application(
                                    TermApplication(
                                        Id {
                                            value: 1,
                                        },
                                    ),
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
                            ty: Success(
                                Application(
                                    TermApplication(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                            traits: [],
                        },
                    ],
                },
                term_sheet: SignatureTermSheet {
                    term_symbol_page: TermSymbolPage {
                        registry: TermSymbolRegistry {
                            tys: [
                                Ok(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                    ),
                                ),
                                Ok(
                                    Application(
                                        TermApplication(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                    ),
                                ),
                            ],
                        },
                        inherited_symbol_terms: [],
                        current_symbol_terms: [
                            TermSymbol(
                                Id {
                                    value: 1,
                                },
                            ),
                            TermSymbol(
                                Id {
                                    value: 2,
                                },
                            ),
                        ],
                    },
                },
            },
        ),
    ),
]