Ok(
    [
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`malamute::OneVsAll`, `Enum`),
            ),
        ),
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`malamute::OneVsAllResult`, `Enum`),
            ),
        ),
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`malamute::narrow_down`, `Gn`),
            ),
        ),
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath {
                    module_path: `malamute`,
                    trai_path: TraitPath(`core::ops::Unveil`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`malamute::OneVsAll`, `Enum`),
                    ),
                    disambiguator: 0,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath {
                    impl_block: TraitForTypeImplBlockPath {
                        module_path: `malamute`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`malamute::OneVsAll`, `Enum`),
                        ),
                        disambiguator: 0,
                    },
                    ident: `Output`,
                    item_kind: AssociatedType,
                },
            ),
        ),
    ],
)