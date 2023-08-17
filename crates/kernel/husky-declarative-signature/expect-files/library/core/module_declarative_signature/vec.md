[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::vec::Vec`, `Extern`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Extern(
                        ExternTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
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
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath {
                    module_path: `core::vec`,
                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                    disambiguator: 0,
                },
            ),
        ),
        Ok(
            SignatureTemplate::ImplBlock(
                ImplBlockDeclarativeSignatureTemplate::TypeImpl(
                    TypeImplBlockDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: DeclarativeTermSymbol(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                    traits: [],
                                },
                            ],
                        },
                        ty: DeclarativeTerm(`core::vec::Vec t`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::vec`,
                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
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
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 5,
                                                },
                                            ),
                                            traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::vec::Vec t`),
                            },
                            self_ty: DeclarativeTerm(`core::vec::Vec t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`core::vec::Vec t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
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
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::vec`,
                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
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
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 5,
                                                },
                                            ),
                                            traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::vec::Vec t`),
                            },
                            self_ty: DeclarativeTerm(`core::vec::Vec t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: BorrowMut,
                                ty: DeclarativeTerm(`core::vec::Vec t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
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
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::vec`,
                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
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
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 5,
                                                },
                                            ),
                                            traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::vec::Vec t`),
                            },
                            self_ty: DeclarativeTerm(`core::vec::Vec t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`core::vec::Vec t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
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
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::vec`,
                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
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
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 5,
                                                },
                                            ),
                                            traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::vec::Vec t`),
                            },
                            self_ty: DeclarativeTerm(`core::vec::Vec t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`core::vec::Vec t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
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
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::vec`,
                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
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
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 5,
                                                },
                                            ),
                                            traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::vec::Vec t`),
                            },
                            self_ty: DeclarativeTerm(`core::vec::Vec t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: BorrowMut,
                                ty: DeclarativeTerm(`core::vec::Vec t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
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
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::vec`,
                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `collect_leashes`,
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
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 5,
                                                },
                                            ),
                                            traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::vec::Vec t`),
                            },
                            self_ty: DeclarativeTerm(`core::vec::Vec t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: Leash,
                                ty: DeclarativeTerm(`core::vec::Vec t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DeclarativeTerm(`[] ~ t`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::vec`,
                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `cyclic_slice_leashed`,
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
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 5,
                                                },
                                            ),
                                            traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::vec::Vec t`),
                            },
                            self_ty: DeclarativeTerm(`core::vec::Vec t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: Leash,
                                ty: DeclarativeTerm(`core::vec::Vec t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`core::num::i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`~ core::slice::CyclicSlice t`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath {
                    impl_block: TypeImplBlockPath {
                        module_path: `core::vec`,
                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `pop_with_largest_opt_f32`,
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
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: DeclarativeTermSymbol(
                                                Id {
                                                    value: 5,
                                                },
                                            ),
                                            traits: [],
                                        },
                                    ],
                                },
                                ty: DeclarativeTerm(`core::vec::Vec t`),
                            },
                            self_ty: DeclarativeTerm(`core::vec::Vec t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: BorrowMut,
                                ty: DeclarativeTerm(`core::vec::Vec t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeTermRitchieParameter::Regular(
                                        DeclarativeTermRitchieRegularParameter {
                                            contract: None,
                                            ty: DeclarativeTerm(`fn(t) -> core::option::Option core::num::f32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DeclarativeTerm(`core::option::Option t`),
                        },
                    ),
                ),
            ),
        ),
    ),
]