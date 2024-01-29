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
                                        path: TypePath(`core::option::Option`, `Enum`),
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
        ItemSynNodePath::Attr(
            Room32,
            AttrSynNodePath(
                ItemSynNodePathId {
                    data: ItemSynNodePathData::Attr(
                        AttrSynNodePathData {
                            parent_syn_node_path: ItemSynNodePath::MajorItem(
                                MajorItemSynNodePath::Type(
                                    TypeSynNodePath(
                                        ItemSynNodePathId {
                                            data: ItemSynNodePathData::MajorItem(
                                                MajorItemSynNodePathData::Type(
                                                    TypeSynNodePathData {
                                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                                            path: TypePath(`core::option::Option`, `Enum`),
                                                            disambiguator: 0,
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ),
                            maybe_ambiguous_path: MaybeAmbiguousPath {
                                path: AttrItemPath(
                                    ItemPathId {
                                        data: ItemPathData::Attr(
                                            AttrItemPathData {
                                                parent: ItemPath::MajorItem(
                                                    MajorItemPath::Type(
                                                        TypePath(`core::option::Option`, `Enum`),
                                                    ),
                                                ),
                                                ident: `derive`,
                                                disambiguator: 0,
                                            },
                                        ),
                                    },
                                ),
                                disambiguator: 0,
                            },
                        },
                    ),
                },
            ),
        ),
        None,
    ),
]