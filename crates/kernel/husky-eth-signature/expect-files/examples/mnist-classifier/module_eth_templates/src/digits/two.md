```rust
[
    (
        ItemPath(`mnist_classifier::digits::two::two_match`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: FormPath(`mnist_classifier::digits::two::two_match`, `Val`),
                            return_ty: EthTerm(`FermiMatchResult`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::digits::two::left_cc_pattern`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: FormPath(`mnist_classifier::digits::two::left_cc_pattern`, `Ritchie(
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
                                parameter_contracted_tys: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: EthTerm(`Leash ConcaveComponent`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`Option f32`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::digits::two::right_cc_pattern`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: FormPath(`mnist_classifier::digits::two::right_cc_pattern`, `Ritchie(
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
                                parameter_contracted_tys: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: EthTerm(`Leash ConcaveComponent`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`Option f32`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::digits::two::down_cc_pattern`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: FormPath(`mnist_classifier::digits::two::down_cc_pattern`, `Ritchie(
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
                                parameter_contracted_tys: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: EthTerm(`Leash ConcaveComponent`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`Option f32`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::digits::two::is_two`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: FormPath(`mnist_classifier::digits::two::is_two`, `Val`),
                            return_ty: EthTerm(`OneVsAll MnistLabel Two`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```