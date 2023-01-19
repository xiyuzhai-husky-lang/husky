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
                                    value: 129,
                                },
                            ),
                        ),
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
                    RegularStructFieldSignature {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 130,
                                },
                            ),
                        ),
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
                    RegularStructFieldSignature {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 131,
                                },
                            ),
                        ),
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
                    RegularStructFieldSignature {
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 132,
                                },
                            ),
                        ),
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
                                        value: 6,
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
                                            value: 4,
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
                                    value: 140,
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
                                                    value: 15,
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
                                                    value: 15,
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
                                        value: 15,
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
                            ty: Abort(
                                TermAbortion,
                            ),
                        },
                    ],
                },
                output_ty: Success(
                    Application(
                        TermApplication(
                            Id {
                                value: 8,
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
    ImplBlock(
        TypeImplBlock(
            TypeImplBlockSignature {
                ty: Success(
                    Entity(
                        ModuleItem(
                            Type(
                                TypePath(
                                    Id {
                                        value: 27,
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
                        Application(
                            TermApplication(
                                Id {
                                    value: 9,
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
                                            value: 26,
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
                                            value: 25,
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