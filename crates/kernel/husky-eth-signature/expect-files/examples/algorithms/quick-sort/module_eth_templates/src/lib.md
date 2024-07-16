```rust
[
    (
        ItemPath(`quick_sort::quick_sort`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`quick_sort::quick_sort`, `Ritchie(
                                Fn,
                            )`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        symbol: EthSymbolicVariable(`T`),
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
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::BorrowMut,
                                            ty: EthTerm(`Slice T`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`unit`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`quick_sort::quick_sort_aux`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                                Fn,
                            )`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        symbol: EthSymbolicVariable(`T`),
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
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::BorrowMut,
                                            ty: EthTerm(`Slice T`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`isize`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`isize`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`unit`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`quick_sort::partition`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`quick_sort::partition`, `Ritchie(
                                Fn,
                            )`),
                            template_parameters: EthTemplateParameters {
                                data: [
                                    EthTemplateParameter {
                                        annotated_variance: None,
                                        symbol: EthSymbolicVariable(`T`),
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
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::BorrowMut,
                                            ty: EthTerm(`Slice T`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`isize`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`isize`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`isize`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`quick_sort::quick_sort_works_for_integers`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                                Fn,
                            )`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            ritchie_ty: EthRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                parameter_contracted_tys: [],
                                return_ty: EthTerm(`unit`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`quick_sort::quick_sort_works_for_strs`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                                Fn,
                            )`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            ritchie_ty: EthRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                parameter_contracted_tys: [],
                                return_ty: EthTerm(`unit`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
]
```