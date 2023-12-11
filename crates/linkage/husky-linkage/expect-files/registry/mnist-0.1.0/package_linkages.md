[
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist::BinaryImage28`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::TypeConstructor(
            TypePath(`mnist::BinaryGrid28`, `Struct`),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
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
    },
    Linkage {
        data: LinkageData::AssociatedFunctionFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist::BinaryImage28(0)::new_zeros`, `AssociatedFunctionFn`),
            ),
        ),
    },
    Linkage {
        data: LinkageData::MethodFn(
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
    },
    Linkage {
        data: LinkageData::AssociatedFunctionFn(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist::BinaryGrid28(0)::new_zeros`, `AssociatedFunctionFn`),
            ),
        ),
    },
]