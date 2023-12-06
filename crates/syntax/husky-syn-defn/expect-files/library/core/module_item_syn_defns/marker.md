[
    (
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::marker::Copy`),
            ),
        ),
        None,
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Trait(
                TraitPath(`core::marker::Sized`),
            ),
        ),
        None,
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlock {
                    data: TraitForTypeImplBlockPathData {
                        module_path: `core::marker`,
                        trai_path: TraitPath(`core::marker::Copy`),
                        ty_sketch: TypeSketch::DeriveAny,
                        disambiguator: 0,
                    },
                },
            ),
        ),
        None,
    ),
]