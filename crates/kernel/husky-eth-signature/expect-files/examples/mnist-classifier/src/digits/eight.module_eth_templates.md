```rust
[
    (
        ItemPath(`mnist_classifier::digits::eight::upper_mouth_match`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: MajorFormPath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                            return_ty: EthTerm(`FermiMatchResult`),
                            expr_ty: EthTerm(`Leash FermiMatchResult`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::digits::eight::is_eight`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Val(
                        MajorValEthTemplate {
                            path: MajorFormPath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                            return_ty: EthTerm(`OneVsAll MnistLabel Eight`),
                            expr_ty: EthTerm(`OneVsAll MnistLabel Eight`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::digits::eight::big_mouth`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`mnist_classifier::digits::eight::big_mouth`, `Ritchie(
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
                                            contract: Contract::Pure,
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
]
```