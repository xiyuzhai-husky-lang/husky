```rust
[
    (
        ItemPath(`mnist_classifier::digits::eight::upper_mouth_match`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Val(
                        MajorValDecTemplate {
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 52,
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
        ItemPath(`mnist_classifier::digits::eight::is_eight`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Val(
                        MajorValDecTemplate {
                            return_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 11,
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
        ItemPath(`mnist_classifier::digits::eight::big_mouth`),
        Ok(
            ItemDecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorFunctionRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: Pure,
                                            ty: Application(
                                                DecApplication(
                                                    Id {
                                                        value: 12,
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ],
                            },
                            return_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 7,
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