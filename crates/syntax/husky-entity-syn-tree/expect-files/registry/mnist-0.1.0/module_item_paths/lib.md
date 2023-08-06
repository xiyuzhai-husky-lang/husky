Ok(
    [
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist::MnistLabel`, `Enum`),
            ),
        ),
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist::BinaryImage28`, `Struct`),
            ),
        ),
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist::input`, `Val`),
            ),
        ),
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath {
                    module_path: `mnist`,
                    trai_path: TraitPath(`core::ops::IntIndex`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist::BinaryImage28`, `Struct`),
                    ),
                    disambiguator: 0,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath {
                    impl_block: TraitForTypeImplBlockPath {
                        module_path: `mnist`,
                        trai_path: TraitPath(`core::ops::IntIndex`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`mnist::BinaryImage28`, `Struct`),
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