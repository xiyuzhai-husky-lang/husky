Ok(
    [
        ItemPath::MajorItem(
            MajarItemPath::Type(
                TypePath(`core::mem::Ref`, `Extern`),
            ),
        ),
        ItemPath::MajorItem(
            MajarItemPath::Type(
                TypePath(`core::mem::RefMut`, `Extern`),
            ),
        ),
        ItemPath::MajorItem(
            MajarItemPath::Type(
                TypePath(`core::mem::Leash`, `Extern`),
            ),
        ),
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath {
                    module_path: `core::mem`,
                    trai_path: TraitPath(`core::marker::Copy`),
                    ty_sketch: Path(
                        TypePath(
                            Id {
                                value: 13,
                            },
                        ),
                    ),
                    disambiguator: 0,
                },
            ),
        ),
    ],
)