[
    Linkage {
        data: LinkageData::EnumU8ToJsonValue {
            ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
        },
    },
    Linkage {
        data: LinkageData::EnumU8ToJsonValue {
            ty_path: TypePath(`malamute::OneVsAllResult`, `Enum`),
        },
    },
    Linkage {
        data: LinkageData::AssocRitchie {
            path: AssocItemPath::TraitForTypeItem(
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
                                    item_kind: AssocRitchie(
                                        Fn,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::AssocRitchie {
            path: AssocItemPath::TraitForTypeItem(
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
                                    item_kind: AssocRitchie(
                                        Fn,
                                    ),
                                },
                            ),
                        ),
                    },
                ),
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::UnveilAssocFn {
            path: TraitForTypeItemPath(
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
                                item_kind: AssocRitchie(
                                    Fn,
                                ),
                            },
                        ),
                    ),
                },
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [],
                separator: Some(
                    0,
                ),
            },
        },
    },
    Linkage {
        data: LinkageData::TypeVariantConstructor {
            path: TypeVariantPath(
                ItemPathId {
                    data: ItemPathData::TypeVariant(
                        TypeVariantPathData {
                            parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                            ident: `Break`,
                            index: U8(
                                1,
                            ),
                        },
                    ),
                },
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
    Linkage {
        data: LinkageData::TypeVariantConstructor {
            path: TypeVariantPath(
                ItemPathId {
                    data: ItemPathData::TypeVariant(
                        TypeVariantPathData {
                            parent_ty_path: TypePath(`core::ops::ControlFlow`, `Enum`),
                            ident: `Continue`,
                            index: U8(
                                0,
                            ),
                        },
                    ),
                },
            ),
            instantiation: LinInstantiation {
                symbol_resolutions: [
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 0,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 1,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                    (
                        Type(
                            Type {
                                attrs: HirTemplateSvarAttrs {
                                    class: Comptime,
                                },
                                variance: None,
                                disambiguator: 1,
                            },
                        ),
                        Explicit(
                            Type(
                                PathLeading(
                                    LinTypePathLeading(
                                        Id {
                                            value: 2,
                                        },
                                    ),
                                ),
                            ),
                        ),
                    ),
                ],
                separator: None,
            },
        },
    },
]