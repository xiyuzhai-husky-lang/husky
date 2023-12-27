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
                        javelin_item_path: JavelinPath::TraitForTypeItem(
                            TraitForTypeItemPath(
                                ItemPathId {
                                    data: ItemPathData::AssociatedItem(
                                        AssociatedItemPathData::TraitForTypeItem(
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
                                                item_kind: AssociatedFunctionFn,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateSymbol::Type(
                                        HirTypeSymbol::Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
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
                                    HirTemplateSymbol::Const(
                                        HirConstSymbol {
                                            ty: HirType::Symbol(
                                                HirTypeSymbol::Type {
                                                    attrs: HirTemplateSymbolAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                            index: Other {
                                                attrs: HirTemplateSymbolAttrs {
                                                    class: Runtime,
                                                },
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 390,
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
                        javelin_item_path: JavelinPath::TraitForTypeItem(
                            TraitForTypeItemPath(
                                ItemPathId {
                                    data: ItemPathData::AssociatedItem(
                                        AssociatedItemPathData::TraitForTypeItem(
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
                                                item_kind: AssociatedFunctionFn,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateSymbol::Type(
                                        HirTypeSymbol::Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
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
                                    HirTemplateSymbol::Const(
                                        HirConstSymbol {
                                            ty: HirType::Symbol(
                                                HirTypeSymbol::Type {
                                                    attrs: HirTemplateSymbolAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                            index: Other {
                                                attrs: HirTemplateSymbolAttrs {
                                                    class: Runtime,
                                                },
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 395,
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
                        javelin_item_path: JavelinPath::TraitForTypeItem(
                            TraitForTypeItemPath(
                                ItemPathId {
                                    data: ItemPathData::AssociatedItem(
                                        AssociatedItemPathData::TraitForTypeItem(
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
                                                item_kind: AssociatedFunctionFn,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateSymbol::Type(
                                        HirTypeSymbol::Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
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
                                    HirTemplateSymbol::Const(
                                        HirConstSymbol {
                                            ty: HirType::Symbol(
                                                HirTypeSymbol::Type {
                                                    attrs: HirTemplateSymbolAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                            index: Other {
                                                attrs: HirTemplateSymbolAttrs {
                                                    class: Runtime,
                                                },
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 389,
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
                        javelin_item_path: JavelinPath::TraitForTypeItem(
                            TraitForTypeItemPath(
                                ItemPathId {
                                    data: ItemPathData::AssociatedItem(
                                        AssociatedItemPathData::TraitForTypeItem(
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
                                                item_kind: AssociatedFunctionFn,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateSymbol::Type(
                                        HirTypeSymbol::Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
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
                                    HirTemplateSymbol::Const(
                                        HirConstSymbol {
                                            ty: HirType::Symbol(
                                                HirTypeSymbol::Type {
                                                    attrs: HirTemplateSymbolAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                            index: Other {
                                                attrs: HirTemplateSymbolAttrs {
                                                    class: Runtime,
                                                },
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 396,
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
                        javelin_item_path: JavelinPath::TraitForTypeItem(
                            TraitForTypeItemPath(
                                ItemPathId {
                                    data: ItemPathData::AssociatedItem(
                                        AssociatedItemPathData::TraitForTypeItem(
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
                                                item_kind: AssociatedFunctionFn,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateSymbol::Type(
                                        HirTypeSymbol::Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
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
                                    HirTemplateSymbol::Const(
                                        HirConstSymbol {
                                            ty: HirType::Symbol(
                                                HirTypeSymbol::Type {
                                                    attrs: HirTemplateSymbolAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                            index: Other {
                                                attrs: HirTemplateSymbolAttrs {
                                                    class: Runtime,
                                                },
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 397,
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
                        javelin_item_path: JavelinPath::TraitForTypeItem(
                            TraitForTypeItemPath(
                                ItemPathId {
                                    data: ItemPathData::AssociatedItem(
                                        AssociatedItemPathData::TraitForTypeItem(
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
                                                item_kind: AssociatedFunctionFn,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateSymbol::Type(
                                        HirTypeSymbol::Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
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
                                    HirTemplateSymbol::Const(
                                        HirConstSymbol {
                                            ty: HirType::Symbol(
                                                HirTypeSymbol::Type {
                                                    attrs: HirTemplateSymbolAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                            index: Other {
                                                attrs: HirTemplateSymbolAttrs {
                                                    class: Runtime,
                                                },
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 392,
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
                        javelin_item_path: JavelinPath::TraitForTypeItem(
                            TraitForTypeItemPath(
                                ItemPathId {
                                    data: ItemPathData::AssociatedItem(
                                        AssociatedItemPathData::TraitForTypeItem(
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
                                                item_kind: AssociatedFunctionFn,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateSymbol::Type(
                                        HirTypeSymbol::Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
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
                                    HirTemplateSymbol::Const(
                                        HirConstSymbol {
                                            ty: HirType::Symbol(
                                                HirTypeSymbol::Type {
                                                    attrs: HirTemplateSymbolAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                            index: Other {
                                                attrs: HirTemplateSymbolAttrs {
                                                    class: Runtime,
                                                },
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 398,
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
                        javelin_item_path: JavelinPath::TraitForTypeItem(
                            TraitForTypeItemPath(
                                ItemPathId {
                                    data: ItemPathData::AssociatedItem(
                                        AssociatedItemPathData::TraitForTypeItem(
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
                                                item_kind: AssociatedFunctionFn,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateSymbol::Type(
                                        HirTypeSymbol::Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
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
                                    HirTemplateSymbol::Const(
                                        HirConstSymbol {
                                            ty: HirType::Symbol(
                                                HirTypeSymbol::Type {
                                                    attrs: HirTemplateSymbolAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                            index: Other {
                                                attrs: HirTemplateSymbolAttrs {
                                                    class: Runtime,
                                                },
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 394,
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
                        javelin_item_path: JavelinPath::TraitForTypeItem(
                            TraitForTypeItemPath(
                                ItemPathId {
                                    data: ItemPathData::AssociatedItem(
                                        AssociatedItemPathData::TraitForTypeItem(
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
                                                item_kind: AssociatedFunctionFn,
                                            },
                                        ),
                                    ),
                                },
                            ),
                        ),
                        hir_instantiation: HirInstantiation {
                            symbol_map: [
                                (
                                    HirTemplateSymbol::Type(
                                        HirTypeSymbol::Type {
                                            attrs: HirTemplateSymbolAttrs {
                                                class: Comptime,
                                            },
                                            variance: None,
                                            disambiguator: 0,
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
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
                                    HirTemplateSymbol::Const(
                                        HirConstSymbol {
                                            ty: HirType::Symbol(
                                                HirTypeSymbol::Type {
                                                    attrs: HirTemplateSymbolAttrs {
                                                        class: Comptime,
                                                    },
                                                    variance: None,
                                                    disambiguator: 0,
                                                },
                                            ),
                                            index: Other {
                                                attrs: HirTemplateSymbolAttrs {
                                                    class: Runtime,
                                                },
                                                disambiguator: 0,
                                            },
                                        },
                                    ),
                                    HirTermSymbolResolution::Explicit(
                                        HirTemplateArgument::Constant(
                                            TypeVariant(
                                                TypeVariantPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 391,
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