```rust
[
    (
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`quick_sort::quick_sort`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: BorrowMut,
                                            ty: Application(
                                                DecApplication(
                                                    Id {
                                                        value: 3,
                                                    },
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
                                                value: 8,
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
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`quick_sort::quick_sort_aux`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: BorrowMut,
                                            ty: Application(
                                                DecApplication(
                                                    Id {
                                                        value: 3,
                                                    },
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
                                                                value: 16,
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
                                                                value: 16,
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
                                                value: 8,
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
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`quick_sort::partition`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSymbolicVariable(
                                            Id {
                                                value: 3,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Simple(
                                        DeclarativeRitchieSimpleParameter {
                                            contract: BorrowMut,
                                            ty: Application(
                                                DecApplication(
                                                    Id {
                                                        value: 3,
                                                    },
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
                                                                value: 16,
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
                                                                value: 16,
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
                                                value: 16,
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
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`quick_sort::quick_sort_works_for_integers`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 8,
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
        ItemPath::MajorItem(
            MajorItemPath::Form(
                FormPath(`quick_sort::quick_sort_works_for_strs`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Form(
                    MajorFormDecTemplate::Ritchie(
                        MajorRitchieDecTemplate {
                            ritchie_item_kind: RitchieItemKind::Fn,
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [],
                            },
                            return_ty: EntityPath(
                                Type(
                                    TypePath(
                                        ItemPathId(
                                            Id {
                                                value: 8,
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
]
```