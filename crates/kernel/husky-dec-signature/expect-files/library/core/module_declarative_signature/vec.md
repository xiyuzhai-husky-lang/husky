[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::vec::Vec`, `Extern`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternTypeDecTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        symbol: SymbolDecTerm(
                                            Id {
                                                value: 4,
                                            },
                                        ),
                                        annotated_traits: [],
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
                TypeImplBlockPath(
                    ItemPathId {
                        data: ItemPathData::ImplBlock(
                            ImplBlockPathData::TypeImplBlock(
                                TypeImplBlockPathData {
                                    module_path: `core::vec`,
                                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                    disambiguator: 0,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        Ok(
            DecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: SymbolDecTerm(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        ty: DecTerm(`core::vec::Vec t`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: SymbolDecTerm(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: DecTerm(`core::vec::Vec t`),
                            },
                            self_ty: DecTerm(`core::vec::Vec t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Pure,
                                ty: DecTerm(`core::vec::Vec t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm(`core::num::i32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: SymbolDecTerm(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: DecTerm(`core::vec::Vec t`),
                            },
                            self_ty: DecTerm(`core::vec::Vec t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: SymbolDecTerm(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: BorrowMut,
                                ty: DecTerm(`core::vec::Vec t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Move,
                                            ty: DecTerm(`t`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm(`core::basic::unit`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::first`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::vec::Vec(0)::first`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: SymbolDecTerm(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: DecTerm(`core::vec::Vec t`),
                            },
                            self_ty: DecTerm(`core::vec::Vec t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: SymbolDecTerm(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: At,
                                ty: DecTerm(`core::vec::Vec t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm(`core::option::Option core::mem::At 'α t`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::last`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::vec::Vec(0)::last`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: SymbolDecTerm(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: DecTerm(`core::vec::Vec t`),
                            },
                            self_ty: DecTerm(`core::vec::Vec t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: SymbolDecTerm(
                                            Id {
                                                value: 2,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: At,
                                ty: DecTerm(`core::vec::Vec t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm(`core::option::Option core::mem::At 'α t`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: SymbolDecTerm(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: DecTerm(`core::vec::Vec t`),
                            },
                            self_ty: DecTerm(`core::vec::Vec t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: SymbolDecTerm(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: BorrowMut,
                                ty: DecTerm(`core::vec::Vec t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm(`core::option::Option t`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::collect_leashes`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::vec::Vec(0)::collect_leashes`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: SymbolDecTerm(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: DecTerm(`core::vec::Vec t`),
                            },
                            self_ty: DecTerm(`core::vec::Vec t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Leash,
                                ty: DecTerm(`core::vec::Vec t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: DecTerm(`[] ~ t`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::cyclic_slice_leashed`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::vec::Vec(0)::cyclic_slice_leashed`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: SymbolDecTerm(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: DecTerm(`core::vec::Vec t`),
                            },
                            self_ty: DecTerm(`core::vec::Vec t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: Leash,
                                ty: DecTerm(`core::vec::Vec t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Pure,
                                            ty: DecTerm(`core::num::i32`),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Pure,
                                            ty: DecTerm(`core::num::i32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm(`~ core::slice::CyclicSlice t`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(core::vec::Vec(0)::pop_with_largest_opt_f32`, `MethodFn`),
            ),
        ),
        Ok(
            DecTemplate::AssociatedItem(
                AssociatedItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`(core::vec::Vec(0)::pop_with_largest_opt_f32`, `MethodFn`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DeclarativeTemplateParameterTemplates {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            symbol: SymbolDecTerm(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: DecTerm(`core::vec::Vec t`),
                            },
                            self_ty: DecTerm(`core::vec::Vec t`),
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: SymbolDecTerm(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: DeclarativeRitchieRegularParameter {
                                contract: BorrowMut,
                                ty: DecTerm(`core::vec::Vec t`),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
                                        DeclarativeRitchieRegularParameter {
                                            contract: Pure,
                                            ty: DecTerm(`fn((t) -> core::option::Option core::num::f32`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: DecTerm(`core::option::Option t`),
                        },
                    ),
                ),
            ),
        ),
    ),
]