[
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`core::mem::Ref`, `Extern`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`core::mem::RefMut`, `Extern`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`core::mem::Leash`, `Extern`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`core::mem::At`, `Extern`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TraitForTypeImplBlock(
            TraitForTypeImplBlockPath {
                module_path: `core::mem`,
                trai_path: TraitPath(`core::marker::Copy`),
                ty_sketch: TypeSketch::Path(
                    TypePath(`core::mem::Leash`, `Extern`),
                ),
                disambiguator: 0,
            },
        ),
    ),
]