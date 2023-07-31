[
    (
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::option::Option`, `Enum`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Enum(
                        EnumTypeDeclarativeSignatureTemplate {
                            template_parameters: DeclarativeTemplateParameterTemplates {
                                data: [
                                    DeclarativeTemplateParameter {
                                        annotated_variance: None,
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 5,
                                            },
                                        ),
                                        traits: [],
                                    },
                                ],
                            },
                            self_ty: DeclarativeTerm(`core::option::Option t`),
                        },
                    ),
                ),
            ),
        ),
    ),
]