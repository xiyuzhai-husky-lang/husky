[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::slice::Slice`, `Extern`),
            ),
        ),
        None,
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`core::slice::CyclicSlice`, `Extern`),
            ),
        ),
        None,
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(
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
            ),
        ),
        None,
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`(core::slice::Slice(0)::len`, `MethodRitchie(
                    Fn,
                )`),
            ),
        ),
        None,
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`(core::slice::Slice(0)::swap`, `MethodRitchie(
                    Fn,
                )`),
            ),
        ),
        None,
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlock {
                    data: TraitForTypeImplBlockPathData {
                        module_path: `core::slice`,
                        trai_path: TraitPath(`core::ops::IntIndex`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`core::slice::CyclicSlice`, `Extern`),
                        ),
                        disambiguator: 0,
                    },
                },
            ),
        ),
        None,
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId {
                        data: ItemPathData::AssocItem(
                            AssocItemPathData::TraitForTypeItem(
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
                                    item_kind: AssocType,
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
        ItemPath::ImplBlock(
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(
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
            ),
        ),
        None,
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`(core::slice::CyclicSlice(0)::ilen`, `MethodRitchie(
                    Fn,
                )`),
            ),
        ),
        None,
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodRitchie(
                    Fn,
                )`),
            ),
        ),
        None,
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodRitchie(
                    Fn,
                )`),
            ),
        ),
        None,
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`(core::slice::CyclicSlice(0)::first`, `MethodRitchie(
                    Fn,
                )`),
            ),
        ),
        None,
    ),
    (
        ItemPath::AssocItem(
            AssocItemPath::TypeItem(
                TypeItemPath(`(core::slice::CyclicSlice(0)::last`, `MethodRitchie(
                    Fn,
                )`),
            ),
        ),
        None,
    ),
]