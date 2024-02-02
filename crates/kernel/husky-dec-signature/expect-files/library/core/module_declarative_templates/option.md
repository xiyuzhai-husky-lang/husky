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
                                        symbol: SymbolDecTerm(
                                            Id {
                                                value: 10,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                ],
                            },
                            self_ty: Application(
                                ApplicationDecTerm(
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