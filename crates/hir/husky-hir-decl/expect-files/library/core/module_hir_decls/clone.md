[
    HirDecl::MajorItem(
        MajorItemHirDecl::Trait(
            TraitHirDecl {
                path: TraitPath(`core::clone::Clone`),
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
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TraitForTypeItem(
            MethodFn(
                TraitForTypeMethodFnHirDecl(
                    Id {
                        value: 1,
                    },
                ),
            ),
        ),
    ),
]