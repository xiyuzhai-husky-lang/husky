[
    AmazonJavelin(
        Javelin {
            data: JavelinData::PathLeading {
                path: JavPath::TypeConstructor(
                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                path: JavPath::TypeConstructor(
                    TypePath(`malamute::OneVsAllResult`, `Enum`),
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
                path: JavPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssocItem(
                                AssocItemPathData::TraitForTypeItem(
                                    TraitForTypeItemPathData {
                                        impl_block: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `malamute`,
                                                trai_path: TraitPath(`core::default::Default`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                        ident: `default`,
                                        item_kind: AssocFunctionFn,
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
                path: JavPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssocItem(
                                AssocItemPathData::TraitForTypeItem(
                                    TraitForTypeItemPathData {
                                        impl_block: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `malamute`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
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
                path: JavPath::TraitForTypeItem(
                    TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssocItem(
                                AssocItemPathData::TraitForTypeItem(
                                    TraitForTypeItemPathData {
                                        impl_block: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `malamute`,
                                                trai_path: TraitPath(`core::ops::Unveil`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`malamute::OneVsAll`, `Enum`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                        ident: `unveil`,
                                        item_kind: AssocFunctionFn,
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