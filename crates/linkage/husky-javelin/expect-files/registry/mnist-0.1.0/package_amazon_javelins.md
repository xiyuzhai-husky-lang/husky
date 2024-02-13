[
    AmazonJavelin(
        Javelin {
            data: JavelinData::PathLeading {
                path: JavPath::TypeConstructor(
                    TypePath(`mnist::MnistLabel`, `Enum`),
                ),
                instantiation: JavInstantiation {
                    symbol_resolutions: [],
                    separator: None,
                },
            },
        },
    ),
    AmazonJavelin(
        Javelin {
            data: JavelinData::PathLeading {
                path: JavPath::Fugitive(
                    FugitivePath(`mnist::input`, `Val`),
                ),
                instantiation: JavInstantiation {
                    symbol_resolutions: [],
                    separator: None,
                },
            },
        },
    ),
    AmazonJavelin(
        Javelin {
            data: JavelinData::PathLeading {
                path: JavPath::TypeItem(
                    TypeItemPath(`(mnist::BinaryImage28(0)::new_zeros`, `AssocRitchie(
                        Fn,
                    )`),
                ),
                instantiation: JavInstantiation {
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
                path: JavPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssocItem(
                                AssocItemPathData::TraitForTypeItem(
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
                                        item_kind: AssocType,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
                instantiation: JavInstantiation {
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
                path: JavPath::TypeItem(
                    TypeItemPath(`(mnist::BinaryGrid28(0)::new_zeros`, `AssocRitchie(
                        Fn,
                    )`),
                ),
                instantiation: JavInstantiation {
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
                path: JavPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssocItem(
                                AssocItemPathData::TraitForTypeItem(
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
                                        item_kind: AssocType,
                                    },
                                ),
                            ),
                        },
                    ),
                ),
                instantiation: JavInstantiation {
                    symbol_resolutions: [],
                    separator: Some(
                        0,
                    ),
                },
            },
        },
    ),
]