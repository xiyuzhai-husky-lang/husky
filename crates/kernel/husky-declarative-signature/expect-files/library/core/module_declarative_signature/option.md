[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`core::option::Option`, `Enum`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Type(
                    TypeDeclarativeSignatureTemplate::Enum(
                        EnumDeclarativeSignatureTemplate {
                            implicit_parameters: ImplicitParameterDeclarativeSignatures {
                                data: [
                                    ImplicitParameterDeclarativeSignature {
                                        annotated_variance: None,
                                        symbol: DeclarativeTermSymbol(
                                            Id {
                                                value: 2,
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