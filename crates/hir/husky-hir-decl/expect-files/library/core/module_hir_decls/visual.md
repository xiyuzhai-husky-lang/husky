[
    HirDecl::MajorItem(
        MajorItemHirDecl::Trait(
            TraitHirDecl {
                path: TraitPath(`core::visual::Visualize`),
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
        MajorItemHirDecl::Type(
            Extern(
                ExternTypeHirDecl(
                    Id {
                        value: 34,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TraitForTypeItem(
            MethodFn(
                TraitForTypeMethodFnHirDecl(
                    Id {
                        value: 17,
                    },
                ),
            ),
        ),
    ),
]