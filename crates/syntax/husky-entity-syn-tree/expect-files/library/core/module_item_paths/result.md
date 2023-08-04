Ok(
    [
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::result::Result`, `Enum`),
            ),
        ),
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath {
                    module_path: `core::result`,
                    trai_path: TraitPath(`core::ops::Unveil`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`core::result::Result`, `Enum`),
                    ),
                    disambiguator: 0,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath {
                    impl_block: TraitForTypeImplBlockPath {
                        module_path: `core::result`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`core::result::Result`, `Enum`),
                        ),
                        disambiguator: 0,
                    },
                    ident: `Continue`,
                    item_kind: AssociatedType,
                },
            ),
        ),
        ItemPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath {
                    impl_block: TraitForTypeImplBlockPath {
                        module_path: `core::result`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`core::result::Result`, `Enum`),
                        ),
                        disambiguator: 0,
                    },
                    ident: `branch`,
                    item_kind: MethodFn,
                },
            ),
        ),
    ],
)