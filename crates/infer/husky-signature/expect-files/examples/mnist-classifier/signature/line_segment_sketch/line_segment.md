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
                                    value: 201,
                                },
                            ),
                        ),
                        ty: Success(
                            Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 31,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    },
                    RegularStructFieldSignature {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 202,
                                },
                            ),
                        ),
                        ty: Success(
                            Entity(
                                ModuleItem(
                                    Type(
                                        TypePath(
                                            Id {
                                                value: 31,
                                            },
                                        ),
                                    ),
                                ),
                            ),
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
                                        value: 41,
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
    AssociatedItem(
        TypeItem(
            Method(
                TypeMethodSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        parameters: [],
                    },
                    parameters: ParameterSignatures {
                        parameters: [],
                    },
                    output_ty: Success(
                        Entity(
                            ModuleItem(
                                Type(
                                    TypePath(
                                        Id {
                                            value: 33,
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
    ),
    AssociatedItem(
        TypeItem(
            Method(
                TypeMethodSignature {
                    implicit_parameters: ImplicitParameterSignatures {
                        parameters: [],
                    },
                    parameters: ParameterSignatures {
                        parameters: [
                            ParameterSignature {
                                pattern: ParameterSignaturePattern,
                                ty: Success(
                                    Entity(
                                        ModuleItem(
                                            Type(
                                                TypePath(
                                                    Id {
                                                        value: 31,
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
                                            value: 13,
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
    ),
]