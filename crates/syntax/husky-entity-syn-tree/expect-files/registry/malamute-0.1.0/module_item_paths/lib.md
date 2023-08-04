Ok(
    [
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`malamute::OneVsAll`, `Enum`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Type(
                TypePath(`malamute::OneVsAllResult`, `Enum`),
            ),
        ),
        ItemPath::ModuleItem(
            ModuleItemPath::Fugitive(
                FugitivePath(`malamute::narrow_down`, `Gn`),
            ),
        ),
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlockPath {
                    module_path: `malamute`,
                    trai_path: TraitPath(`core::ops::Unveil`),
                    ty_sketch: Path(
                        TypePath(
                            Id {
                                value: 104,
                            },
                        ),
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
                        ty_sketch: Path(
                            TypePath(
                                Id {
                                    value: 104,
                                },
                            ),
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