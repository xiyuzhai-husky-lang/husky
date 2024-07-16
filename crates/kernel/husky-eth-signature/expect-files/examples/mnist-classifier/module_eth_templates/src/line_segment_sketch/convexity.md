```rust
[
    (
        ItemPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Ritchie(
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
                                            ty: EthTerm(`LineSegmentSketch`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: EthTerm(`i32`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`bool`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
]
```