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
    ItemPath::AssociatedItem(
        AssociatedItemPath::TypeItem(
            TypeItemPath(`(core::slice::Slice(0)::len`, `MethodFn`),
        ),
    ),
    ItemPath::AssociatedItem(
        AssociatedItemPath::TypeItem(
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
    ItemPath::AssociatedItem(
        AssociatedItemPath::TraitForTypeItem(
            TraitForTypeItemPath(
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
    ItemPath::AssociatedItem(
        AssociatedItemPath::TypeItem(
            TypeItemPath(`(core::slice::CyclicSlice(0)::ilen`, `MethodFn`),
        ),
    ),
    ItemPath::AssociatedItem(
        AssociatedItemPath::TypeItem(
            TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
        ),
    ),
    ItemPath::AssociatedItem(
        AssociatedItemPath::TypeItem(
            TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodFn`),
        ),
    ),
    ItemPath::AssociatedItem(
        AssociatedItemPath::TypeItem(
            TypeItemPath(`(core::slice::CyclicSlice(0)::first`, `MethodFn`),
        ),
    ),
    ItemPath::AssociatedItem(
        AssociatedItemPath::TypeItem(
            TypeItemPath(`(core::slice::CyclicSlice(0)::last`, `MethodFn`),
        ),
    ),
]