[
    ItemPath::MajorItem(
        MajorItemPath::Trait(
            TraitPath(`core::visual::Visualize`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`core::visual::Visual`, `Extern`),
        ),
    ),
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
    ItemPath::AssocItem(
        AssocItemPath::TraitForTypeItem(
            TraitForTypeItemPath(
                ItemPathId {
                    data: ItemPathData::AssocItem(
                        AssocItemPathData::TraitForTypeItem(
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
                                item_kind: MethodRitchie(
                                    Fn,
                                ),
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]