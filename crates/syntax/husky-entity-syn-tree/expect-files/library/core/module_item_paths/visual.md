Ok(
    [
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::visual::Visualize`),
            ),
        ),
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::visual::Html`, `Extern`),
            ),
        ),
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath {
                    module_path: `core::visual`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: TypeSketch::DeriveAny,
                    disambiguator: 0,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath {
                    impl_block: TraitForTypeImplBlockPath {
                        module_path: `core::visual`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_sketch: TypeSketch::DeriveAny,
                        disambiguator: 0,
                    },
                    ident: `clone`,
                    item_kind: MethodFn,
                },
            ),
        ),
    ],
)