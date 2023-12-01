[
    (
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Trait(
                TraitSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::MajorItem(
                            MajorItemSynNodePathData::Trait(
                                TraitSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitPath(`core::visual::Visualize`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        None,
    ),
    (
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Type(
                TypeSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::MajorItem(
                            MajorItemSynNodePathData::Type(
                                TypeSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::visual::Html`, `Extern`),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        None,
    ),
    (
        ItemSynNodePath::ImplBlock(
            ImplBlockSynNodePath::TraitForTypeImplBlock(
                TraitForTypeImplBlockSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::ImplBlock(
                            ImplBlockSynNodePathData::TraitForTypeImplBlock(
                                TraitForTypeImplBlockSynNodePathData {
                                    path: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `core::visual`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::DeriveAny,
                                            disambiguator: 0,
                                        },
                                    },
                                },
                            ),
                        ),
                    },
                ),
            ),
        ),
        None,
    ),
    (
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TraitForTypeItem(
                TraitForTypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TraitForTypeItem(
                                TraitForTypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TraitForTypeItemPath(
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
                                        disambiguator: 0,
                                    },
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