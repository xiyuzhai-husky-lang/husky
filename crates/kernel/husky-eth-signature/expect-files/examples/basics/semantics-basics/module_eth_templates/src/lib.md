```rust
[
    (
        ItemPath(`semantics_basics::some_neural_network`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`semantics_basics::some_neural_network`, `Ritchie(
                                Gn,
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
                                            ty: EthTerm(`Array 3 f32`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`Array 3 f32`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
]
```