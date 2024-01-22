[
    (
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::visual::Visualize`),
            ),
        ),
        None,
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::visual::Visual`, `Extern`),
            ),
        ),
        None,
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlock {
                    data: TraitForTypeImplBlockPathData {
                        module_path: `core::visual`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_sketch: TypeSketch::DeriveAny,
                        disambiguator: 0,
                    },
                },
            ),
        ),
        None,
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId {
                        data: ItemPathData::AssociatedItem(
                            AssociatedItemPathData::TraitForTypeItem(
                                TraitForTypeItemPathData {
                                    impl_block: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `core::visual`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::DeriveAny,
                                            disambiguator: 0,
                                        },
                                    },
                                    ident: `visualize`,
                                    item_kind: MethodFn,
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        None,
    ),
]