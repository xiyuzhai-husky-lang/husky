[
    Form(
        Feature(
            FeatureSignature {
                term_sheet: SignatureTermSheet {
                    term_symbol_page: TermSymbolPage {
                        registry: TermSymbolRegistry {
                            next_ty0: 0,
                            next_lifetime: 0,
                            next_binding: 0,
                            next_usize: 0,
                            next_parameter: 0,
                        },
                        inherited_symbol_terms: [],
                        current_symbol_terms: [],
                    },
                },
            },
        ),
    ),
    Form(
        Function(
            FunctionSignature {
                implicit_parameters: ImplicitParameterSignatures {
                    parameters: [],
                },
                parameter_decl_list: ParameterSignatures {
                    parameters: [
                        ParameterSignature {
                            pattern: ParameterSignaturePattern,
                            ty: Success(
                                Application(
                                    TermApplication(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                output_ty: Success(
                    Application(
                        TermApplication(
                            Id {
                                value: 7,
                            },
                        ),
                    ),
                ),
                term_sheet: SignatureTermSheet {
                    term_symbol_page: TermSymbolPage {
                        registry: TermSymbolRegistry {
                            next_ty0: 0,
                            next_lifetime: 0,
                            next_binding: 0,
                            next_usize: 0,
                            next_parameter: 1,
                        },
                        inherited_symbol_terms: [],
                        current_symbol_terms: [
                            TermSymbol {
                                idx: 0,
                                kind: Parameter,
                            },
                        ],
                    },
                },
            },
        ),
    ),
    Form(
        Feature(
            FeatureSignature {
                term_sheet: SignatureTermSheet {
                    term_symbol_page: TermSymbolPage {
                        registry: TermSymbolRegistry {
                            next_ty0: 0,
                            next_lifetime: 0,
                            next_binding: 0,
                            next_usize: 0,
                            next_parameter: 0,
                        },
                        inherited_symbol_terms: [],
                        current_symbol_terms: [],
                    },
                },
            },
        ),
    ),
]