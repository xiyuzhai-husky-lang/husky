```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::slice::Slice`, `Extern`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternTypeDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        svar: DecSvar(
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
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::slice::CyclicSlice`, `Extern`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Extern(
                        ExternTypeDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        svar: DecSvar(
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
                    ItemPathId(
                        Id {
                            value: 124,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            DecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSvar(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        ty: Application(
                            DecApplication(
                                Id {
                                    value: 39,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`<core::slice::Slice(0)>::len`, `MethodRitchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`<core::slice::Slice(0)>::len`, `MethodRitchie(
                                Fn,
                            )`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            svar: DecSvar(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: Application(
                                    DecApplication(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                ),
                            },
                            self_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 39,
                                    },
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: Application(
                                    DecApplication(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 53,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`<core::slice::Slice(0)>::swap`, `MethodRitchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`<core::slice::Slice(0)>::swap`, `MethodRitchie(
                                Fn,
                            )`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            svar: DecSvar(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: Application(
                                    DecApplication(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                ),
                            },
                            self_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 39,
                                    },
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSvar(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: BorrowMut,
                                ty: Application(
                                    DecApplication(
                                        Id {
                                            value: 39,
                                        },
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 53,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: EntityPath(
                                                Type(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 53,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 23,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath(`core::slice::CyclicSlice as core::ops::IntIndex(0)`),
            ),
        ),
        Ok(
            DecTemplate::ImplBlock(
                ImplBlockDecTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSvar(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        trai: EntityPath(
                            Trait(
                                TraitPath(
                                    ItemPathId(
                                        Id {
                                            value: 66,
                                        },
                                    ),
                                ),
                            ),
                        ),
                        self_ty: DeclarativeSelfType::Path(
                            Application(
                                DecApplication(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    `<core::slice::CyclicSlice as core::ops::IntIndex(0)>::Output`,
                    TraitItemKind::AssocType,
                ),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TraitForTypeItem(
                    TraitForTypeItemDecTemplate::AssocType(
                        TraitForTypeAssocTypeDecTemplate {
                            path: TraitForTypeItemPath(
                                `<core::slice::CyclicSlice as core::ops::IntIndex(0)>::Output`,
                                TraitItemKind::AssocType,
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            ty_term: Symbol(
                                DecSvar(
                                    Id {
                                        value: 10,
                                    },
                                ),
                            ),
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
                    ItemPathId(
                        Id {
                            value: 126,
                        },
                    ),
                ),
            ),
        ),
        Ok(
            DecTemplate::ImplBlock(
                ImplBlockDecTemplate::TypeImpl(
                    TypeImplBlockDecTemplate {
                        template_parameters: DecTemplateParameters {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    svar: DecSvar(
                                        Id {
                                            value: 10,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                        ty: Application(
                            DecApplication(
                                Id {
                                    value: 40,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`<core::slice::CyclicSlice(0)>::ilen`, `MethodRitchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`<core::slice::CyclicSlice(0)>::ilen`, `MethodRitchie(
                                Fn,
                            )`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            svar: DecSvar(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: Application(
                                    DecApplication(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                            self_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: Application(
                                    DecApplication(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 44,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`<core::slice::CyclicSlice(0)>::start`, `MethodRitchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`<core::slice::CyclicSlice(0)>::start`, `MethodRitchie(
                                Fn,
                            )`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            svar: DecSvar(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: Application(
                                    DecApplication(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                            self_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: Application(
                                    DecApplication(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 44,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`<core::slice::CyclicSlice(0)>::end`, `MethodRitchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`<core::slice::CyclicSlice(0)>::end`, `MethodRitchie(
                                Fn,
                            )`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            svar: DecSvar(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: Application(
                                    DecApplication(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                            self_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: Application(
                                    DecApplication(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 44,
                                            },
                                        ),
                                    ),
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`<core::slice::CyclicSlice(0)>::first`, `MethodRitchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`<core::slice::CyclicSlice(0)>::first`, `MethodRitchie(
                                Fn,
                            )`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            svar: DecSvar(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: Application(
                                    DecApplication(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                            self_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: Application(
                                    DecApplication(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`<core::slice::CyclicSlice(0)>::last`, `MethodRitchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            DecTemplate::AssocItem(
                AssocItemDecTemplate::TypeItem(
                    TypeItemDecTemplate::MethodFn(
                        TypeMethodFnDecTemplate {
                            path: TypeItemPath(`<core::slice::CyclicSlice(0)>::last`, `MethodRitchie(
                                Fn,
                            )`),
                            impl_block: TypeImplBlockDecTemplate {
                                template_parameters: DecTemplateParameters {
                                    data: [
                                        DeclarativeTemplateParameter {
                                            annotated_variance: None,
                                            svar: DecSvar(
                                                Id {
                                                    value: 10,
                                                },
                                            ),
                                            annotated_traits: [],
                                        },
                                    ],
                                },
                                ty: Application(
                                    DecApplication(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                            self_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 40,
                                    },
                                ),
                            ),
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: DeclarativeRitchieSimpleParameter {
                                contract: Pure,
                                ty: Application(
                                    DecApplication(
                                        Id {
                                            value: 40,
                                        },
                                    ),
                                ),
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 42,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```