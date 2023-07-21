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
                        value: 18,
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
                                ty: DeclarativeTerm(`core::list::List t`),
                            },
                            self_ty: DeclarativeTerm(`core::list::List t`),
                            generic_parameters: DeclarativeGenericParameterTemplates {
                                data: [],
                            },
                            self_parameter: SpecificRegularDeclarativeParameterTemplate {
                                contract: None,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 6,
                                        },
                                    ),
                                ),
                            },
                            parenic_parameters: DeclarativeParenicParameters {
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
                                ty: DeclarativeTerm(`core::list::List t`),
                            },
                            self_ty: DeclarativeTerm(`core::list::List t`),
                            generic_parameters: DeclarativeGenericParameterTemplates {
                                data: [],
                            },
                            self_parameter: SpecificRegularDeclarativeParameterTemplate {
                                contract: BorrowMut,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 6,
                                        },
                                    ),
                                ),
                            },
                            parenic_parameters: DeclarativeParenicParameters {
                                data: [
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`t`),
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
                                ty: DeclarativeTerm(`core::list::List t`),
                            },
                            self_ty: DeclarativeTerm(`core::list::List t`),
                            generic_parameters: DeclarativeGenericParameterTemplates {
                                data: [],
                            },
                            self_parameter: SpecificRegularDeclarativeParameterTemplate {
                                contract: None,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 6,
                                        },
                                    ),
                                ),
                            },
                            parenic_parameters: DeclarativeParenicParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::option::Option t`),
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
                                ty: DeclarativeTerm(`core::list::List t`),
                            },
                            self_ty: DeclarativeTerm(`core::list::List t`),
                            generic_parameters: DeclarativeGenericParameterTemplates {
                                data: [],
                            },
                            self_parameter: SpecificRegularDeclarativeParameterTemplate {
                                contract: None,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 6,
                                        },
                                    ),
                                ),
                            },
                            parenic_parameters: DeclarativeParenicParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::option::Option t`),
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
                    ident: `pop`,
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
                                ty: DeclarativeTerm(`core::list::List t`),
                            },
                            self_ty: DeclarativeTerm(`core::list::List t`),
                            generic_parameters: DeclarativeGenericParameterTemplates {
                                data: [],
                            },
                            self_parameter: SpecificRegularDeclarativeParameterTemplate {
                                contract: BorrowMut,
                                ty: ExplicitApplication(
                                    DeclarativeTermExplicitApplication(
                                        Id {
                                            value: 6,
                                        },
                                    ),
                                ),
                            },
                            parenic_parameters: DeclarativeParenicParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`core::option::Option t`),
                        },
                    ),
                ),
            ),
        ),
    ),
]