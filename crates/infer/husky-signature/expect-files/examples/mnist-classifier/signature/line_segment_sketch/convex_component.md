[
    Type(
        RegularStruct(
            RegularStructTypeSignature {
                implicit_parameters: ImplicitParameterSignatures {
                    parameters: [],
                },
                fields: [
                    RegularStructFieldSignature {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 99,
                                },
                            ),
                        ),
                        ty: Success(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 17,
                                    },
                                ),
                            ),
                        ),
                    },
                    RegularStructFieldSignature {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 315,
                                },
                            ),
                        ),
                        ty: Abort(
                            TermAbortion,
                        ),
                    },
                ],
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
    ImplBlock(
        TypeImplBlock(
            TypeImplBlockSignature {
                ty: Success(
                    Entity(
                        ModuleItem(
                            Type(
                                TypePath(
                                    Id {
                                        value: 40,
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