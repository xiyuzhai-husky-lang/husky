```rust
[
    (
        ItemPath(`malamute::Class`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Enum(
                        EnumEthTemplate {
                            path: TypePath(`malamute::Class`, `Enum`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        symbol: EthSymbolicVariable(`Label`),
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
        ItemPath(`malamute::OneVsAll`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Enum(
                        EnumEthTemplate {
                            path: TypePath(`malamute::OneVsAll`, `Enum`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        symbol: EthSymbolicVariable(`Label`),
                                        traits: [],
                                    },
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        symbol: EthSymbolicVariable(`label`),
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
        ItemPath(`malamute::OneVsAllResult`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::Enum(
                        EnumEthTemplate {
                            path: TypePath(`malamute::OneVsAllResult`, `Enum`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        symbol: EthSymbolicVariable(`Label`),
                                        traits: [],
                                    },
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        symbol: EthSymbolicVariable(`label`),
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
        ItemPath(`malamute::narrow_down`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: FormPath(`malamute::narrow_down`, `Ritchie(
                                Gn,
                            )`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        symbol: EthSymbolicVariable(`Label`),
                                        traits: [],
                                    },
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        symbol: EthSymbolicVariable(`label`),
                                        traits: [],
                                    },
                                ],
                            },
                            ritchie_ty: EthRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                parameter_contracted_tys: [
                                    EtherealRitchieParameter::Variadic(
                                        EtherealRitchieVariadicParameter {
                                            contract: Pure,
                                            ty: EthTerm(`f32`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Keyed(
                                        EtherealRitchieKeyedParameter {
                                            key: Ident(
                                                Coword(
                                                    Id {
                                                        value: 41,
                                                    },
                                                ),
                                            ),
                                            contract: Pure,
                                            ty: ItemPath(
                                                TypeOntology(
                                                    TypePath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 18,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            has_default: true,
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`OneVsAllResult Label label`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`malamute::OneVsAll as core::default::Default(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::default::Default(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    symbol: EthSymbolicVariable(`Label`),
                                    traits: [],
                                },
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    symbol: EthSymbolicVariable(`label`),
                                    traits: [],
                                },
                            ],
                        },
                        trai: EthTerm(`Default`),
                        self_ty_refined: PathLeading(
                            Application(
                                EthApplication(
                                    Id {
                                        value: 4,
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
        ItemPath(`<malamute::OneVsAll as core::default::Default(0)>::default`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    AssocRitchie(
                        TraitForTypeAssocRitchieEthTemplate(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`malamute::Class as core::ops::Unveil(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`malamute::Class as core::ops::Unveil(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    symbol: EthSymbolicVariable(`Label`),
                                    traits: [],
                                },
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    symbol: EthSymbolicVariable(`label`),
                                    traits: [],
                                },
                            ],
                        },
                        trai: EthTerm(`Unveil OneVsAll Label label`),
                        self_ty_refined: PathLeading(
                            Application(
                                EthApplication(
                                    Id {
                                        value: 12,
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
        ItemPath(`<malamute::Class as core::ops::Unveil(0)>::Output`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    AssocType(
                        TraitForTypeAssocTypeEthTemplate(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`<malamute::Class as core::ops::Unveil(0)>::unveil`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    AssocRitchie(
                        TraitForTypeAssocRitchieEthTemplate(
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
        ItemPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TraitForTypeImpl(
                    TraitForTypeImplBlockEthTemplate {
                        path: TraitForTypeImplBlockPath(`malamute::OneVsAll as core::ops::Unveil(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    symbol: EthSymbolicVariable(`Label`),
                                    traits: [],
                                },
                                EthTemplateParameter {
                                    annotated_variance: None,
                                    symbol: EthSymbolicVariable(`label`),
                                    traits: [],
                                },
                            ],
                        },
                        trai: EthTerm(`Unveil OneVsAllResult Label label`),
                        self_ty_refined: PathLeading(
                            Application(
                                EthApplication(
                                    Id {
                                        value: 4,
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
        ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::Output`),
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
        ItemPath(`<malamute::OneVsAll as core::ops::Unveil(0)>::unveil`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::TraitForType(
                    AssocRitchie(
                        TraitForTypeAssocRitchieEthTemplate(
                            Id {
                                value: 3,
                            },
                        ),
                    ),
                ),
            ),
        ),
    ),
]
```