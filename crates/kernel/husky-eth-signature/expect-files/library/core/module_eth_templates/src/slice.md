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
                                        symbol: EthSymbolicVariable(`E`),
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
                                        symbol: EthSymbolicVariable(`E`),
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
                                    symbol: EthSymbolicVariable(`E`),
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
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 32,
                            },
                        ),
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
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 33,
                            },
                        ),
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
                                    symbol: EthSymbolicVariable(`E`),
                                    traits: [],
                                },
                            ],
                        },
                        trai: EthTerm(`IntIndex`),
                        self_ty_refined: PathLeading(
                            Application(
                                EthApplication(
                                    Id {
                                        value: 25,
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
                    AssocType(
                        TraitForTypeAssocTypeEthTemplate(
                            Id {
                                value: 2,
                            },
                        ),
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
                                    symbol: EthSymbolicVariable(`E`),
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
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 34,
                            },
                        ),
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
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 35,
                            },
                        ),
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
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 36,
                            },
                        ),
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
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 37,
                            },
                        ),
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
                    MethodRitchie(
                        TypeMethodRitchieEthTemplate(
                            Id {
                                value: 38,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
]
```