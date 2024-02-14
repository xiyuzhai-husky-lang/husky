[
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::zero::open_one_match`, `Val`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Fugitive(
                    FugitiveDecTemplate::Val(
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
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::zero::almost_closed`, `Ritchie(
                    Fn,
                )`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Fugitive(
                    FugitiveDecTemplate::Fn(
                        MajorFnDecTemplate {
                            template_parameters: DecTemplateParameters {
                                data: [],
                            },
                            parenate_parameters: DeclarativeParenateParameters {
                                data: [
                                    DeclarativeRitchieParameter::Regular(
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
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::digits::zero::is_zero`, `Val`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Fugitive(
                    FugitiveDecTemplate::Val(
                        MajorValDecTemplate {
                            return_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 21,
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