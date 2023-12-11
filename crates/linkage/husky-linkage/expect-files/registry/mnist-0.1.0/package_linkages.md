[
    Linkage {
        javelin: Javelin {
            data: JavelinData::PathLeading {
                path: JavelinPath::TypeConstructor(
                    TypePath(`mnist::BinaryImage28`, `Struct`),
                ),
                instantiation: JavelinInstantiation {
                    symbol_resolutions: [],
                    separator: None,
                },
            },
        },
        data: LinkageData::TypeConstructor(
            TypePath(`mnist::BinaryImage28`, `Struct`),
        ),
    },
    Linkage {
        javelin: Javelin {
            data: JavelinData::PathLeading {
                path: JavelinPath::TypeConstructor(
                    TypePath(`mnist::BinaryGrid28`, `Struct`),
                ),
                instantiation: JavelinInstantiation {
                    symbol_resolutions: [],
                    separator: None,
                },
            },
        },
        data: LinkageData::TypeConstructor(
            TypePath(`mnist::BinaryGrid28`, `Struct`),
        ),
    },
    Linkage {
        javelin: Javelin {
            data: JavelinData::PathLeading {
                path: JavelinPath::TraitForTypeItem(
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
                instantiation: JavelinInstantiation {
                    symbol_resolutions: [],
                    separator: None,
                },
            },
        },
        data: LinkageData::TraitForTypeMethodFn(
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
    },
    Linkage {
        javelin: Javelin {
            data: JavelinData::PathLeading {
                path: JavelinPath::TypeItem(
                    TypeItemPath(`(mnist::BinaryImage28(0)::new_zeros`, `AssociatedFunctionFn`),
                ),
                instantiation: JavelinInstantiation {
                    symbol_resolutions: [],
                    separator: None,
                },
            },
        },
        data: LinkageData::TypeAssociatedFunctionFn(
            TypeItemPath(`(mnist::BinaryImage28(0)::new_zeros`, `AssociatedFunctionFn`),
        ),
    },
    Linkage {
        javelin: Javelin {
            data: JavelinData::PathLeading {
                path: JavelinPath::TraitForTypeItem(
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
                instantiation: JavelinInstantiation {
                    symbol_resolutions: [],
                    separator: None,
                },
            },
        },
        data: LinkageData::TraitForTypeMethodFn(
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
    },
    Linkage {
        javelin: Javelin {
            data: JavelinData::PathLeading {
                path: JavelinPath::TypeItem(
                    TypeItemPath(`(mnist::BinaryGrid28(0)::new_zeros`, `AssociatedFunctionFn`),
                ),
                instantiation: JavelinInstantiation {
                    symbol_resolutions: [],
                    separator: None,
                },
            },
        },
        data: LinkageData::TypeAssociatedFunctionFn(
            TypeItemPath(`(mnist::BinaryGrid28(0)::new_zeros`, `AssociatedFunctionFn`),
        ),
    },
]