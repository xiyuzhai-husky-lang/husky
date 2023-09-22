[
    (
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::marker::Copy`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Trait(
                    TraitDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: DeclarativeTermSymbol(
                                        Id {
                                            value: 3,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::marker::Sized`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Trait(
                    TraitDeclarativeSignatureTemplate {
                        template_parameters: DeclarativeTemplateParameterTemplates {
                            data: [
                                DeclarativeTemplateParameter {
                                    annotated_variance: None,
                                    symbol: DeclarativeTermSymbol(
                                        Id {
                                            value: 3,
                                        },
                                    ),
                                    annotated_traits: [],
                                },
                            ],
                        },
                    },
                ),
            ),
        ),
    ),
]