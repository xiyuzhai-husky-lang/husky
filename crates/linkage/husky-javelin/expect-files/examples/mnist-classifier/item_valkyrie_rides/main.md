[
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(
                ItemPathId {
                    data: ItemPathData::SubmoduleItem(
                        SubmoduleItemPathData {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::connected_component`,
                            ),
                        },
                    ),
                },
            ),
        ),
        None,
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(
                ItemPathId {
                    data: ItemPathData::SubmoduleItem(
                        SubmoduleItemPathData {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::raw_contour`,
                            ),
                        },
                    ),
                },
            ),
        ),
        None,
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(
                ItemPathId {
                    data: ItemPathData::SubmoduleItem(
                        SubmoduleItemPathData {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::geom2d`,
                            ),
                        },
                    ),
                },
            ),
        ),
        None,
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(
                ItemPathId {
                    data: ItemPathData::SubmoduleItem(
                        SubmoduleItemPathData {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::line_segment_sketch`,
                            ),
                        },
                    ),
                },
            ),
        ),
        None,
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(
                ItemPathId {
                    data: ItemPathData::SubmoduleItem(
                        SubmoduleItemPathData {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::fermi`,
                            ),
                        },
                    ),
                },
            ),
        ),
        None,
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(
                ItemPathId {
                    data: ItemPathData::SubmoduleItem(
                        SubmoduleItemPathData {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::digits`,
                            ),
                        },
                    ),
                },
            ),
        ),
        None,
    ),
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(
                ItemPathId {
                    data: ItemPathData::SubmoduleItem(
                        SubmoduleItemPathData {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::major`,
                            ),
                        },
                    ),
                },
            ),
        ),
        None,
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::main`, `Val`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [
                    ValkyrieRide::PathLeading {
                        javelin_item_path: JavPath::TraitForTypeItem(
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
                                                    ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVar::Const(
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
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 264,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                2,
                            ),
                        },
                    },
                    ValkyrieRide::PathLeading {
                        javelin_item_path: JavPath::TraitForTypeItem(
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
                                                    ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVar::Const(
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
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 269,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                2,
                            ),
                        },
                    },
                    ValkyrieRide::PathLeading {
                        javelin_item_path: JavPath::TraitForTypeItem(
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
                                                    ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVar::Const(
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
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 263,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                2,
                            ),
                        },
                    },
                    ValkyrieRide::PathLeading {
                        javelin_item_path: JavPath::TraitForTypeItem(
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
                                                    ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVar::Const(
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
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 270,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                2,
                            ),
                        },
                    },
                    ValkyrieRide::PathLeading {
                        javelin_item_path: JavPath::TraitForTypeItem(
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
                                                    ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVar::Const(
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
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 271,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                2,
                            ),
                        },
                    },
                    ValkyrieRide::PathLeading {
                        javelin_item_path: JavPath::TraitForTypeItem(
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
                                                    ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVar::Const(
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
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 266,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                2,
                            ),
                        },
                    },
                    ValkyrieRide::PathLeading {
                        javelin_item_path: JavPath::TraitForTypeItem(
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
                                                    ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVar::Const(
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
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 272,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                2,
                            ),
                        },
                    },
                    ValkyrieRide::PathLeading {
                        javelin_item_path: JavPath::TraitForTypeItem(
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
                                                    ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVar::Const(
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
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 268,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                2,
                            ),
                        },
                    },
                    ValkyrieRide::PathLeading {
                        javelin_item_path: JavPath::TraitForTypeItem(
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
                                                    ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateVar::Const(
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
                                    HirTermSvarResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 265,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                2,
                            ),
                        },
                    },
                ],
            },
        ),
    ),
]