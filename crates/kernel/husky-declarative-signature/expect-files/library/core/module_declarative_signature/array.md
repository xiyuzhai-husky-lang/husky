[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::array::Array`, `Extern`),
            ),
        ),
        Ok(
            SignatureTemplate::MajorItem(
                MajorItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Extern(
                        ExternTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 1,
                                            },
                                        ),
                                        annotated_traits: [],
                                    },
                                    DeclarativeTemplateParameter {
                                        annotated_variance: Some(
                                            Covariant,
                                        ),
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 2,
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
    ),
]