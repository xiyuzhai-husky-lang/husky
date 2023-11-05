[
    HirDecl::MajorItem(
        MajorItemHirDecl::Trait(
            TraitHirDecl {
                path: TraitPath(`core::clone::Clone`),
                template_parameters: HirTemplateParameters(
                    [],
                ),
            },
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath {
                    module_path: `core::clone`,
                    trai_path: TraitPath(`core::clone::Clone`),
                    ty_sketch: TypeSketch::DeriveAny,
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters(
                    [],
                ),
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