[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::list::List`, `Extern`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Extern(
                        ExternDeclarativeSignatureTemplate {
                            implicit_parameters: DeclarativeGenericParameters {
                                data: [
                                    DeclarativeGenericParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                        traits: [],
                                    },
                                ],
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::ImplBlock(
            TypeImplBlock(
                TypeImplBlockPath(
                    Id {
                        value: 16,
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        implicit_parameters: DeclarativeGenericParameters {
                            data: [
                                DeclarativeGenericParameter {
                                    annotated_variance: None,
                                    symbol: DeclarativeTermSymbol(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                    traits: [],
                                },
                            ],
                        },
                        ty: DeclarativeTerm(`core::list::List t`),
                    },
                ),
            ),
        ),
    ),
    (
        EntityPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::list`,
                        ty_path: TypePath(`core::list::List`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `ilen`,
                    item_kind: MethodFn,
                },
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                implicit_parameters: DeclarativeGenericParameters {
                                    data: [
                                        DeclarativeGenericParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                            traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::list::List t`),
                            },
                            self_ty: DeclarativeTerm(`core::list::List t`),
                            implicit_parameters: DeclarativeGenericParameters {
                                data: [],
                            },
                            self_parameter: SpecificRegularParameterDeclarativeSignatureTemplate {
                                contract: None,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 6,
                                        },
                                    ),
                                ),
                            },
                            explicit_parameters: DeclarativeSpecificParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::num::i32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::list`,
                        ty_path: TypePath(`core::list::List`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `push`,
                    item_kind: MethodFn,
                },
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                implicit_parameters: DeclarativeGenericParameters {
                                    data: [
                                        DeclarativeGenericParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                            traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::list::List t`),
                            },
                            self_ty: DeclarativeTerm(`core::list::List t`),
                            implicit_parameters: DeclarativeGenericParameters {
                                data: [],
                            },
                            self_parameter: SpecificRegularParameterDeclarativeSignatureTemplate {
                                contract: BorrowMut,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 6,
                                        },
                                    ),
                                ),
                            },
                            explicit_parameters: DeclarativeSpecificParameters {
                                data: [
                                    Regular(
                                        SpecificRegularParameterDeclarativeSignatureTemplate {
                                            contract: None,
                                            ty: Symbol(
                                                DeclarativeTermSymbol(
                                                    Id {
                                                        value: 2,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::basic::unit`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        EntityPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::list`,
                        ty_path: TypePath(`core::list::List`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `first`,
                    item_kind: MethodFn,
                },
            ),
        ),
        Ok(
            SignatureTemplate::AssociatedItem(
                AssociatedItemDeclarativeSignatureTemplate::TypeItem(
                    TypeItemDeclarativeSignatureTemplate::MethodFn(
                        TypeMethodFnDeclarativeSignatureTemplate {
                            impl_block: TypeImplBlockDeclarativeSignatureTemplate {
                                implicit_parameters: DeclarativeGenericParameters {
                                    data: [
                                        DeclarativeGenericParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 2,
                                                },
                                            ),
                                            traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::list::List t`),
                            },
                            self_ty: DeclarativeTerm(`core::list::List t`),
                            implicit_parameters: DeclarativeGenericParameters {
                                data: [],
                            },
                            self_parameter: SpecificRegularParameterDeclarativeSignatureTemplate {
                                contract: None,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 6,
                                        },
                                    ),
                                ),
                            },
                            explicit_parameters: DeclarativeSpecificParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`t`),
                        },
                    ),
                ),
            ),
        ),
    ),
]