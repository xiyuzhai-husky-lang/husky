[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::slice::Slice`, `Extern`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Extern(
                        ExternDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameterTemplates {
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
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Extern(
                        ExternDeclarativeSignatureTemplate {
                            generic_parameters: DeclarativeGenericParameterTemplates {
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
                        value: 34,
                    },
                ),
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        generic_parameters: DeclarativeGenericParameterTemplates {
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
                        ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
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
                        module_path: `core::slice`,
                        ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                generic_parameters: DeclarativeGenericParameterTemplates {
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
                                ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            },
                            self_ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            generic_parameters: DeclarativeGenericParameterTemplates {
                                data: [],
                            },
                            self_parameter: SpecificRegularDeclarativeParameterTemplate {
                                contract: None,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 42,
                                        },
                                    ),
                                ),
                            },
                            explicit_parameters: DeclarativeParenicParameters {
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
                        module_path: `core::slice`,
                        ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                                generic_parameters: DeclarativeGenericParameterTemplates {
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
                                ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            },
                            self_ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            generic_parameters: DeclarativeGenericParameterTemplates {
                                data: [],
                            },
                            self_parameter: SpecificRegularDeclarativeParameterTemplate {
                                contract: None,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 42,
                                        },
                                    ),
                                ),
                            },
                            explicit_parameters: DeclarativeParenicParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::option::Option ~ t`),
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
                        module_path: `core::slice`,
                        ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `last`,
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
                                generic_parameters: DeclarativeGenericParameterTemplates {
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
                                ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            },
                            self_ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            generic_parameters: DeclarativeGenericParameterTemplates {
                                data: [],
                            },
                            self_parameter: SpecificRegularDeclarativeParameterTemplate {
                                contract: None,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 42,
                                        },
                                    ),
                                ),
                            },
                            explicit_parameters: DeclarativeParenicParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::option::Option ~ t`),
                        },
                    ),
                ),
            ),
        ),
    ),
]