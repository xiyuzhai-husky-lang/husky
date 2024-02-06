[
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`core::slice::Slice`, `Extern`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Type(
            TypePath(`core::slice::CyclicSlice`, `Extern`),
        ),
    ),
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
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`(core::slice::Slice(0)::len`, `MethodFn`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`(core::slice::Slice(0)::swap`, `MethodFn`),
        ),
    ),
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
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`(core::slice::CyclicSlice(0)::ilen`, `MethodFn`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodFn`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`(core::slice::CyclicSlice(0)::first`, `MethodFn`),
        ),
    ),
    ItemPath::AssocItem(
        AssocItemPath::TypeItem(
            TypeItemPath(`(core::slice::CyclicSlice(0)::last`, `MethodFn`),
        ),
    ),
]