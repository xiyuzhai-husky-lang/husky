[
    HirDecl::MajorItem(
        MajorItemHirDecl::Trait(
            TraitHirDecl {
                path: TraitPath(`core::marker::Copy`),
                template_parameters: HirTemplateParameters {
                    data: [
                        HirTemplateParameter {
                            symbol: Type(
                                SelfType,
                            ),
                            traits: [],
                        },
                    ],
                },
            },
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Trait(
            TraitHirDecl {
                path: TraitPath(`core::marker::Sized`),
                template_parameters: HirTemplateParameters {
                    data: [
                        HirTemplateParameter {
                            symbol: Type(
                                SelfType,
                            ),
                            traits: [],
                        },
                    ],
                },
            },
        ),
    ),
]