[
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
                                Entity(
                                    ModuleItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 38,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                        ParameterSignature {
                            pattern: ParameterSignaturePattern,
                            ty: Success(
                                Entity(
                                    ModuleItem(
                                        Type(
                                            TypePath(
                                                Id {
                                                    value: 9,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ],
                },
                output_ty: Success(
                    Entity(
                        ModuleItem(
                            Type(
                                TypePath(
                                    Id {
                                        value: 1,
                                    },
                                ),
                            ),
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
                            next_parameter: 2,
                        },
                        inherited_symbol_terms: [],
                        current_symbol_terms: [
                            TermSymbol {
                                idx: 0,
                                kind: Parameter,
                            },
                            TermSymbol {
                                idx: 1,
                                kind: Parameter,
                            },
                        ],
                    },
                },
            },
        ),
    ),
]