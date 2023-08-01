[
    HirDecl::ModuleItem(
        ModuleItemHirDecl::Trait(
            TraitHirDecl {
                path: TraitPath(`core::clone::Clone`),
                template_parameters: HirTemplateParameters {
                    data: [],
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