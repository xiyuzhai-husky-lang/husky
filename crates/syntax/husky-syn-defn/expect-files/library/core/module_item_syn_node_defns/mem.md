[
    (
        ItemSynNodePath::MajorItem(
            MajorItemSynNodePath::Type(
                TypeSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::MajorItem(
                            MajorItemSynNodePathData::Type(
                                TypeSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypePath(`core::mem::Ref`, `Extern`),
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
                                        path: TypePath(`core::mem::RefMut`, `Extern`),
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
                                        path: TypePath(`core::mem::Leash`, `Extern`),
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
                                        path: TypePath(`core::mem::At`, `Extern`),
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
                                            module_path: `core::mem`,
                                            trai_path: TraitPath(`core::marker::Copy`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`core::mem::Leash`, `Extern`),
                                            ),
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
]