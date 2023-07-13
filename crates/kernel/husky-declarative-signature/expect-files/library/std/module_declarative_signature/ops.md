[
    (
        EntityPath::ModuleItem(
            ModuleItemPath::Trait(
                TraitPath(`std::ops::Add`),
            ),
        ),
        Ok(
            SignatureTemplate::ModuleItem(
                ModuleItemDeclarativeSignatureTemplate::Trait(
                    TraitDeclarativeSignatureTemplate {
                        generic_parameters: DeclarativeGenericParameters {
                            data: [
                                DeclarativeGenericParameter {
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
                    },
                ),
            ),
        ),
    ),
]