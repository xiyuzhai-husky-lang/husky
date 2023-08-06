[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::slice::Slice`, `Extern`),
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
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                    module_path: `core::slice`,
                    ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
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
                        ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
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
                                ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            },
                            self_ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
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
                        module_path: `core::slice`,
                        ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `start`,
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
                                ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            },
                            self_ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
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
                        module_path: `core::slice`,
                        ty_path: TypePath(`core::slice::CyclicSliceLeashed`, `Extern`),
                        disambiguator: 0,
                    },
                    ident: `end`,
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
                                ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            },
                            self_ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
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
                                ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            },
                            self_ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
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
        ItemPath::AssociatedItem(
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
                                ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            },
                            self_ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_parameter: DeclarativeTermRitchieRegularParameter {
                                contract: None,
                                ty: DeclarativeTerm(`core::slice::CyclicSliceLeashed t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
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