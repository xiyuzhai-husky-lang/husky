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
                                    value: 134,
                                },
                            ),
                        ),
                        ty: Success(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 12,
                                    },
                                ),
                            ),
                        ),
                    },
                    RegularStructFieldSignature {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 351,
                                },
                            ),
                        ),
                        ty: Success(
                            Application(
                                TermApplication(
                                    Id {
                                        value: 13,
                                    },
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
                                            value: 15,
                                        },
                                    ),
                                ),
                            ),
                        },
                        ParameterSignature {
                            pattern: ParameterSignaturePattern,
                            ty: Success(
                                Application(
                                    TermApplication(
                                        Id {
                                            value: 16,
                                        },
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
                                        value: 42,
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
    ImplBlock(
        TypeImplBlock(
            TypeImplBlockSignature {
                ty: Success(
                    Entity(
                        ModuleItem(
                            Type(
                                TypePath(
                                    Id {
                                        value: 42,
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
            Memo(
                TypeMemoSignature {
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
            Memo(
                TypeMemoSignature {
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
            Memo(
                TypeMemoSignature {
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
]