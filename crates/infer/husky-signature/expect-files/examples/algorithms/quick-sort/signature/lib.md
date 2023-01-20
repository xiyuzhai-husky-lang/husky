[
    Form(
        Function(
            FunctionSignature {
                implicit_parameters: ImplicitParameterSignatures {
                    parameters: [
                        ImplicitParameterSignature {
                            term_symbol: TermSymbol {
                                idx: 0,
                                kind: Type0,
                            },
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
                parameter_decl_list: ParameterSignatures {
                    parameters: [
                        ParameterSignature {
                            pattern: ParameterSignaturePattern,
                            ty: Success(
                                Application(
                                    TermApplication(
                                        Id {
                                            value: 3,
                                        },
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                output_ty: Abort(
                    ExprError,
                ),
                term_sheet: SignatureTermSheet {
                    term_symbol_page: TermSymbolPage {
                        registry: TermSymbolRegistry {
                            next_ty0: 1,
                            next_lifetime: 0,
                            next_binding: 0,
                            next_usize: 0,
                            next_parameter: 1,
                        },
                        inherited_symbol_terms: [],
                        current_symbol_terms: [
                            TermSymbol {
                                idx: 0,
                                kind: Type0,
                            },
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
        Function(
            FunctionSignature {
                implicit_parameters: ImplicitParameterSignatures {
                    parameters: [
                        ImplicitParameterSignature {
                            term_symbol: TermSymbol {
                                idx: 0,
                                kind: Type0,
                            },
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
                parameter_decl_list: ParameterSignatures {
                    parameters: [
                        ParameterSignature {
                            pattern: ParameterSignaturePattern,
                            ty: Success(
                                Application(
                                    TermApplication(
                                        Id {
                                            value: 3,
                                        },
                                    ),
                                ),
                            ),
                        },
                        ParameterSignature {
                            pattern: ParameterSignaturePattern,
                            ty: Abort(
                                TermAbortion,
                            ),
                        },
                        ParameterSignature {
                            pattern: ParameterSignaturePattern,
                            ty: Abort(
                                TermAbortion,
                            ),
                        },
                    ],
                },
                output_ty: Abort(
                    ExprError,
                ),
                term_sheet: SignatureTermSheet {
                    term_symbol_page: TermSymbolPage {
                        registry: TermSymbolRegistry {
                            next_ty0: 1,
                            next_lifetime: 0,
                            next_binding: 0,
                            next_usize: 0,
                            next_parameter: 3,
                        },
                        inherited_symbol_terms: [],
                        current_symbol_terms: [
                            TermSymbol {
                                idx: 0,
                                kind: Type0,
                            },
                            TermSymbol {
                                idx: 0,
                                kind: Parameter,
                            },
                            TermSymbol {
                                idx: 1,
                                kind: Parameter,
                            },
                            TermSymbol {
                                idx: 2,
                                kind: Parameter,
                            },
                        ],
                    },
                },
            },
        ),
    ),
    Form(
        Function(
            FunctionSignature {
                implicit_parameters: ImplicitParameterSignatures {
                    parameters: [
                        ImplicitParameterSignature {
                            term_symbol: TermSymbol {
                                idx: 0,
                                kind: Type0,
                            },
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
                parameter_decl_list: ParameterSignatures {
                    parameters: [
                        ParameterSignature {
                            pattern: ParameterSignaturePattern,
                            ty: Success(
                                Application(
                                    TermApplication(
                                        Id {
                                            value: 3,
                                        },
                                    ),
                                ),
                            ),
                        },
                        ParameterSignature {
                            pattern: ParameterSignaturePattern,
                            ty: Abort(
                                TermAbortion,
                            ),
                        },
                        ParameterSignature {
                            pattern: ParameterSignaturePattern,
                            ty: Abort(
                                TermAbortion,
                            ),
                        },
                    ],
                },
                output_ty: Abort(
                    TermAbortion,
                ),
                term_sheet: SignatureTermSheet {
                    term_symbol_page: TermSymbolPage {
                        registry: TermSymbolRegistry {
                            next_ty0: 1,
                            next_lifetime: 0,
                            next_binding: 0,
                            next_usize: 0,
                            next_parameter: 3,
                        },
                        inherited_symbol_terms: [],
                        current_symbol_terms: [
                            TermSymbol {
                                idx: 0,
                                kind: Type0,
                            },
                            TermSymbol {
                                idx: 0,
                                kind: Parameter,
                            },
                            TermSymbol {
                                idx: 1,
                                kind: Parameter,
                            },
                            TermSymbol {
                                idx: 2,
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