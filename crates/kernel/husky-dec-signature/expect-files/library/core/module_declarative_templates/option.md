[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::option::Option`, `Enum`),
            ),
        ),
        Ok(
            DecTemplate::MajorItem(
                MajorItemDecTemplate::Type(
                    TypeDecTemplate::Enum(
                        EnumTypeDecTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        svar: DecSvar(
                                            Id {
                                                value: 10,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_ty: Application(
                                DecApplication(
                                    Id {
                                        value: 28,
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