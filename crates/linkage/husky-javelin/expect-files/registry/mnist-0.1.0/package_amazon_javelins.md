[
    AmazonJavelin(
        Javelin {
            data: JavelinData::PathLeading {
                path: JavelinPath::TypeConstructor(
                    TypePath(`mnist::MnistLabel`, `Enum`),
                ),
                instantiation: JavelinInstantiation {
                    symbol_resolutions: [],
                    separator: None,
                },
            },
        },
    ),
    AmazonJavelin(
        Javelin {
            data: JavelinData::PathLeading {
                path: JavelinPath::Fugitive(
                    FugitivePath(`mnist::input`, `Val`),
                ),
                instantiation: JavelinInstantiation {
                    symbol_resolutions: [],
                    separator: None,
                },
            },
        },
    ),
    AmazonJavelin(
        Javelin {
            data: JavelinData::PathLeading {
                path: JavelinPath::TypeItem(
                    TypeItemPath(`(mnist::BinaryImage28(0)::new_zeros`, `AssociatedFunctionFn`),
                ),
                instantiation: JavelinInstantiation {
                    symbol_resolutions: [],
                    separator: Some(
                        0,
                    ),
                },
            },
        },
    ),
    AmazonJavelin(
        Javelin {
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
                                                trai_path: TraitPath(`core::ops::IntIndex`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist::BinaryImage28`, `Extern`),
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
                instantiation: JavelinInstantiation {
                    symbol_resolutions: [],
                    separator: Some(
                        0,
                    ),
                },
            },
        },
    ),
    AmazonJavelin(
        Javelin {
            data: JavelinData::PathLeading {
                path: JavelinPath::TypeItem(
                    TypeItemPath(`(mnist::BinaryGrid28(0)::new_zeros`, `AssociatedFunctionFn`),
                ),
                instantiation: JavelinInstantiation {
                    symbol_resolutions: [],
                    separator: Some(
                        0,
                    ),
                },
            },
        },
    ),
    AmazonJavelin(
        Javelin {
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
                                                trai_path: TraitPath(`core::ops::IntIndex`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist::BinaryGrid28`, `Extern`),
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
                instantiation: JavelinInstantiation {
                    symbol_resolutions: [],
                    separator: Some(
                        0,
                    ),
                },
            },
        },
    ),
]