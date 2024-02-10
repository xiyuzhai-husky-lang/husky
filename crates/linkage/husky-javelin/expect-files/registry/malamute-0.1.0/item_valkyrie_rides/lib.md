[
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`malamute::Class`, `Enum`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: HirTemplateVar::Type(
                                    HirTypeSvar::Type {
                                        attrs: HirTemplateVarAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                data: HirTemplateParameterData::Type {
                                    ident: `Label`,
                                    traits: [],
                                },
                            },
                        ],
                    ),
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`malamute::OneVsAll`, `Enum`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`malamute::OneVsAllResult`, `Enum`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`malamute::narrow_down`, `FunctionGn`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: HirTemplateVar::Type(
                                    HirTypeSvar::Type {
                                        attrs: HirTemplateVarAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                data: HirTemplateParameterData::Type {
                                    ident: `Label`,
                                    traits: [],
                                },
                            },
                            HirTemplateParameter {
                                symbol: HirTemplateVar::Const(
                                    HirConstSvar {
                                        ty: HirType::Svar(
                                            HirTypeSvar::Type {
                                                attrs: HirTemplateVarAttrs {
                                                    class: Comptime,
                                                },
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
                                        index: HirConstSymbolIndex::Other {
                                            attrs: HirTemplateVarAttrs {
                                                class: Runtime,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                data: HirTemplateParameterData::Constant {
                                    ident: `label`,
                                    ty: HirType::Svar(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateVarAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                },
                            },
                        ],
                    ),
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlock {
                    data: TraitForTypeImplBlockPathData {
                        module_path: `malamute`,
                        trai_path: TraitPath(`core::default::Default`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`malamute::OneVsAll`, `Enum`),
                        ),
                        disambiguator: 0,
                    },
                },
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
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
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlock {
                    data: TraitForTypeImplBlockPathData {
                        module_path: `malamute`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`malamute::Class`, `Enum`),
                        ),
                        disambiguator: 0,
                    },
                },
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [
                            HirTemplateParameter {
                                symbol: HirTemplateVar::Type(
                                    HirTypeSvar::Type {
                                        attrs: HirTemplateVarAttrs {
                                            class: Comptime,
                                        },
                                        variance: None,
                                        disambiguator: 0,
                                    },
                                ),
                                data: HirTemplateParameterData::Type {
                                    ident: `Label`,
                                    traits: [],
                                },
                            },
                            HirTemplateParameter {
                                symbol: HirTemplateVar::Const(
                                    HirConstSvar {
                                        ty: HirType::Svar(
                                            HirTypeSvar::Type {
                                                attrs: HirTemplateVarAttrs {
                                                    class: Comptime,
                                                },
                                                variance: None,
                                                disambiguator: 0,
                                            },
                                        ),
                                        index: HirConstSymbolIndex::Other {
                                            attrs: HirTemplateVarAttrs {
                                                class: Runtime,
                                            },
                                            disambiguator: 0,
                                        },
                                    },
                                ),
                                data: HirTemplateParameterData::Constant {
                                    ident: `label`,
                                    ty: HirType::Svar(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateVarAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                },
                            },
                        ],
                    ),
                ),
                rides: [],
            },
        ),
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
                                            module_path: `malamute`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`malamute::Class`, `Enum`),
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
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
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
                                            module_path: `malamute`,
                                            trai_path: TraitPath(`core::ops::Unveil`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`malamute::Class`, `Enum`),
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
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [
                    ValkyrieRide::PathLeading {
                        javelin_item_path: JavPath::TypeVariantConstructor(
                            TypeVariantPath(
                                ItemPathId {
                                    data: ItemPathData::TypeVariant(
                                        TypeVariantPathData {
                                            parent_ty_path: TypePath(`malamute::Class`, `Enum`),
                                            ident: `Known`,
                                            index: U8(
                                                0,
                                            ),
                                        },
                                    ),
                                },
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateVar::Type(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateVarAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::Svar(
                                                HirTypeSvar::Type {
                                                    attrs: HirTemplateVarAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: None,
                        },
                    },
                    ValkyrieRide::PathLeading {
                        javelin_item_path: JavPath::TypeVariantConstructor(
                            TypeVariantPath(
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
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateVar::Type(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateVarAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`malamute::Class`, `Enum`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::Svar(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateVarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVar::Type(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateVarAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 1,
                                        },
                                    ),
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: None,
                        },
                    },
                    ValkyrieRide::PathLeading {
                        javelin_item_path: JavPath::TypeVariantConstructor(
                            TypeVariantPath(
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
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateVar::Type(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateVarAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`malamute::Class`, `Enum`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::Svar(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateVarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVar::Type(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateVarAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 1,
                                        },
                                    ),
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: None,
                        },
                    },
                ],
            },
        ),
    ),
    (
        ItemPath::ImplBlock(
            ImplBlockPath::TraitForTypeImplBlock(
                TraitForTypeImplBlock {
                    data: TraitForTypeImplBlockPathData {
                        module_path: `malamute`,
                        trai_path: TraitPath(`core::ops::Unveil`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`malamute::OneVsAll`, `Enum`),
                        ),
                        disambiguator: 0,
                    },
                },
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
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
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [],
            },
        ),
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
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: Some(
                    HirTemplateParameters(
                        [],
                    ),
                ),
                rides: [
                    ValkyrieRide::PathLeading {
                        javelin_item_path: JavPath::TypeVariantConstructor(
                            TypeVariantPath(
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
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateVar::Type(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateVarAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVar::Type(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateVarAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 1,
                                        },
                                    ),
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: None,
                        },
                    },
                    ValkyrieRide::PathLeading {
                        javelin_item_path: JavPath::TypeVariantConstructor(
                            TypeVariantPath(
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
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateVar::Type(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateVarAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVar::Type(
                                        HirTypeSvar::Type {
                                            attrs: HirTemplateVarAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 1,
                                        },
                                    ),
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::basic::unit`, `Extern`),
                                                    template_arguments: [],
                                                    always_copyable: true,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: None,
                        },
                    },
                ],
            },
        ),
    ),
]