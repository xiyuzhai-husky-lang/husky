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
        MajorItemPath::Type(
            TypePath(`mnist::BinaryGrid28`, `Struct`),
        ),
    ),
    ItemPath::MajorItem(
        MajorItemPath::Fugitive(
            FugitivePath(`mnist::input`, `Val`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TraitForTypeImplBlock(
            TraitForTypeImplBlock {
                data: TraitForTypeImplBlockPathData {
                    module_path: `mnist`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist::BinaryImage28`, `Struct`),
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
                                        module_path: `mnist`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist::BinaryImage28`, `Struct`),
                                        ),
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
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TypeImplBlock(
            TypeImplBlockPath(
                ItemPathId {
                    data: ItemPathData::ImplBlock(
                        ImplBlockPathData::TypeImplBlock(
                            TypeImplBlockPathData {
                                module_path: `mnist`,
                                ty_path: TypePath(`mnist::BinaryImage28`, `Struct`),
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
            TypeItemPath(
                ItemPathId {
                    data: ItemPathData::AssociatedItem(
                        AssociatedItemPathData::TypeItem(
                            TypeItemPathData {
                                impl_block: TypeImplBlockPath(
                                    ItemPathId {
                                        data: ItemPathData::ImplBlock(
                                            ImplBlockPathData::TypeImplBlock(
                                                TypeImplBlockPathData {
                                                    module_path: `mnist`,
                                                    ty_path: TypePath(`mnist::BinaryImage28`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                                ident: `new_zeros`,
                                item_kind: AssociatedFunctionFn,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TraitForTypeImplBlock(
            TraitForTypeImplBlock {
                data: TraitForTypeImplBlockPathData {
                    module_path: `mnist`,
                    trai_path: TraitPath(`core::ops::IntIndex`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist::BinaryImage28`, `Struct`),
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
                                        module_path: `mnist`,
                                        trai_path: TraitPath(`core::ops::IntIndex`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist::BinaryImage28`, `Struct`),
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
        ImplBlockPath::TraitForTypeImplBlock(
            TraitForTypeImplBlock {
                data: TraitForTypeImplBlockPathData {
                    module_path: `mnist`,
                    trai_path: TraitPath(`core::visual::Visualize`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist::BinaryGrid28`, `Struct`),
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
                                        module_path: `mnist`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist::BinaryGrid28`, `Struct`),
                                        ),
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
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TypeImplBlock(
            TypeImplBlockPath(
                ItemPathId {
                    data: ItemPathData::ImplBlock(
                        ImplBlockPathData::TypeImplBlock(
                            TypeImplBlockPathData {
                                module_path: `mnist`,
                                ty_path: TypePath(`mnist::BinaryGrid28`, `Struct`),
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
            TypeItemPath(
                ItemPathId {
                    data: ItemPathData::AssociatedItem(
                        AssociatedItemPathData::TypeItem(
                            TypeItemPathData {
                                impl_block: TypeImplBlockPath(
                                    ItemPathId {
                                        data: ItemPathData::ImplBlock(
                                            ImplBlockPathData::TypeImplBlock(
                                                TypeImplBlockPathData {
                                                    module_path: `mnist`,
                                                    ty_path: TypePath(`mnist::BinaryGrid28`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                                ident: `new_zeros`,
                                item_kind: AssociatedFunctionFn,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TraitForTypeImplBlock(
            TraitForTypeImplBlock {
                data: TraitForTypeImplBlockPathData {
                    module_path: `mnist`,
                    trai_path: TraitPath(`core::ops::IntIndex`),
                    ty_sketch: TypeSketch::Path(
                        TypePath(`mnist::BinaryGrid28`, `Struct`),
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
                                        module_path: `mnist`,
                                        trai_path: TraitPath(`core::ops::IntIndex`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist::BinaryGrid28`, `Struct`),
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
]