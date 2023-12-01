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
                                        path: TypePath(`core::slice::Slice`, `Extern`),
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
                                        path: TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                                            path: TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                    ItemPathId(
                                        Id {
                                            value: 193,
                                        },
                                    ),
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
    (
        ItemSynNodePath::ImplBlock(
            ImplBlockSynNodePath::TypeImplBlock(
                TypeImplBlockSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::ImplBlock(
                            ImplBlockSynNodePathData::TypeImplBlock(
                                TypeImplBlockSynNodePathData {
                                    path: TypeImplBlockPath(
                                        ItemPathId(
                                            Id {
                                                value: 126,
                                            },
                                        ),
                                    ),
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
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TypeItem(
                                                        TypeItemPathData {
                                                            impl_block: TypeImplBlockPath(
                                                                ItemPathId {
                                                                    data: ItemPathData::ImplBlock(
                                                                        ImplBlockPathData::TypeImplBlock(
                                                                            TypeImplBlockPathData {
                                                                                module_path: `core::slice`,
                                                                                ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                                                                disambiguator: 0,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            ident: `len`,
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
    (
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TypeItem(
                                                        TypeItemPathData {
                                                            impl_block: TypeImplBlockPath(
                                                                ItemPathId {
                                                                    data: ItemPathData::ImplBlock(
                                                                        ImplBlockPathData::TypeImplBlock(
                                                                            TypeImplBlockPathData {
                                                                                module_path: `core::slice`,
                                                                                ty_path: TypePath(`core::slice::Slice`, `Extern`),
                                                                                disambiguator: 0,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            ident: `swap`,
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
                                            module_path: `core::slice`,
                                            trai_path: TraitPath(`core::ops::IntIndex`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`core::slice::CyclicSlice`, `Extern`),
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
                                                                    module_path: `core::slice`,
                                                                    trai_path: TraitPath(`core::ops::IntIndex`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                            ident: `Output`,
                                                            item_kind: AssociatedType,
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
    (
        ItemSynNodePath::ImplBlock(
            ImplBlockSynNodePath::TypeImplBlock(
                TypeImplBlockSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::ImplBlock(
                            ImplBlockSynNodePathData::TypeImplBlock(
                                TypeImplBlockSynNodePathData {
                                    path: TypeImplBlockPath(
                                        ItemPathId(
                                            Id {
                                                value: 128,
                                            },
                                        ),
                                    ),
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
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TypeItem(
                                                        TypeItemPathData {
                                                            impl_block: TypeImplBlockPath(
                                                                ItemPathId {
                                                                    data: ItemPathData::ImplBlock(
                                                                        ImplBlockPathData::TypeImplBlock(
                                                                            TypeImplBlockPathData {
                                                                                module_path: `core::slice`,
                                                                                ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                                disambiguator: 0,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            ident: `ilen`,
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
    (
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TypeItem(
                                                        TypeItemPathData {
                                                            impl_block: TypeImplBlockPath(
                                                                ItemPathId {
                                                                    data: ItemPathData::ImplBlock(
                                                                        ImplBlockPathData::TypeImplBlock(
                                                                            TypeImplBlockPathData {
                                                                                module_path: `core::slice`,
                                                                                ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                                disambiguator: 0,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            ident: `start`,
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
    (
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TypeItem(
                                                        TypeItemPathData {
                                                            impl_block: TypeImplBlockPath(
                                                                ItemPathId {
                                                                    data: ItemPathData::ImplBlock(
                                                                        ImplBlockPathData::TypeImplBlock(
                                                                            TypeImplBlockPathData {
                                                                                module_path: `core::slice`,
                                                                                ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                                disambiguator: 0,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            ident: `end`,
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
    (
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TypeItem(
                                                        TypeItemPathData {
                                                            impl_block: TypeImplBlockPath(
                                                                ItemPathId {
                                                                    data: ItemPathData::ImplBlock(
                                                                        ImplBlockPathData::TypeImplBlock(
                                                                            TypeImplBlockPathData {
                                                                                module_path: `core::slice`,
                                                                                ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                                disambiguator: 0,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            ident: `first`,
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
    (
        ItemSynNodePath::AssociatedItem(
            AssociatedItemSynNodePath::TypeItem(
                TypeItemSynNodePath(
                    ItemSynNodePathId {
                        data: ItemSynNodePathData::AssociatedItem(
                            AssociatedItemSynNodePathData::TypeItem(
                                TypeItemSynNodePathData {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: TypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TypeItem(
                                                        TypeItemPathData {
                                                            impl_block: TypeImplBlockPath(
                                                                ItemPathId {
                                                                    data: ItemPathData::ImplBlock(
                                                                        ImplBlockPathData::TypeImplBlock(
                                                                            TypeImplBlockPathData {
                                                                                module_path: `core::slice`,
                                                                                ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                                                disambiguator: 0,
                                                                            },
                                                                        ),
                                                                    ),
                                                                },
                                                            ),
                                                            ident: `last`,
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