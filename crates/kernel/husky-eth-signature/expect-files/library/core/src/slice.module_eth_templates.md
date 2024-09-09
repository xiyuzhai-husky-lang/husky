```rust
[
    (
        ItemPath(`core::slice::Slice`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::slice::Slice`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        variable: EthSymbolicVariable(`E`, `mono`),
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
        ItemPath(`core::slice::CyclicSlice`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Extern(
                        ExternTypeEthTemplate {
                            path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        variable: EthSymbolicVariable(`E`, `mono`),
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
        ItemPath(`core::slice::Slice(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::slice::Slice(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    variable: EthSymbolicVariable(`E`, `mono`),
                                    traits: [],
                                },
                            ],
                        },
                        self_ty: EthTerm(`Slice E`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::slice::Slice(0)::len`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `core::slice::Slice(0)::len`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`Slice E`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`Slice E`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`usize`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::slice::Slice(0)::swap`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `core::slice::Slice(0)::swap`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`Slice E`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        variable: EthSymbolicVariable(`'self_lifetime`, `nil`),
                                        traits: [],
                                    },
                                ],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::BorrowMut,
                                ty: EthTerm(`Slice E`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`usize`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`usize`),
                                        },
                                    ),
                                ],
                            },
                            return_ty: EthTerm(`unit`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::slice::CyclicSlice as core::ops::IntIndex(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`core::slice::CyclicSlice as core::ops::IntIndex(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    variable: EthSymbolicVariable(`E`, `mono`),
                                    traits: [],
                                },
                            ],
                        },
                        trai: EthTerm(`IntIndex`),
                        self_ty_refined: PathLeading(
                            Application(
                                EthApplication(
                                    Id {
                                        value: 26,
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
        ItemPath(`<core::slice::CyclicSlice as core::ops::IntIndex(0)>::Output`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    TraitForTypeItemEthTemplate::AssocType(
                        TraitForTypeAssocTypeEthTemplate {
                            path: TraitForTypeItemPath(
                                `<core::slice::CyclicSlice as core::ops::IntIndex(0)>::Output`,
                                TraitItemKind::AssocType,
                            ),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            assoc_ty: EthTerm(`E`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::slice::CyclicSlice(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`core::slice::CyclicSlice(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    variable: EthSymbolicVariable(`E`, `mono`),
                                    traits: [],
                                },
                            ],
                        },
                        self_ty: EthTerm(`CyclicSlice E`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::slice::CyclicSlice(0)::ilen`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `core::slice::CyclicSlice(0)::ilen`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`CyclicSlice E`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`CyclicSlice E`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`i32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::slice::CyclicSlice(0)::start`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `core::slice::CyclicSlice(0)::start`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`CyclicSlice E`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`CyclicSlice E`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`i32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::slice::CyclicSlice(0)::end`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `core::slice::CyclicSlice(0)::end`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`CyclicSlice E`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`CyclicSlice E`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`i32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::slice::CyclicSlice(0)::first`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `core::slice::CyclicSlice(0)::first`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`CyclicSlice E`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`CyclicSlice E`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`Option Leash E`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`core::slice::CyclicSlice(0)::last`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MethodRitchie(
                        TypeMethodRitchieEthTemplate {
                            path: TypeItemPath(
                                `core::slice::CyclicSlice(0)::last`,
                                TypeItemKind::MethodRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                            self_ty: EthTerm(`CyclicSlice E`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            self_value_parameter: EthRitchieSimpleParameter {
                                contract: Contract::Pure,
                                ty: EthTerm(`CyclicSlice E`),
                            },
                            parenate_parameters: EtherealParenateParameters {
                                data: [],
                            },
                            return_ty: EthTerm(`Option Leash E`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```