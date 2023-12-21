[
    (
        ItemPath::Submodule(
            Room32,
            SubmoduleItemPath(
                ItemPathId {
                    data: ItemPathData::SubmoduleItem(
                        SubmoduleItemPathData {
                            submodule_path: SubmodulePath(
                                `mnist_classifier::line_segment_sketch::concave_component`,
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
                                `mnist_classifier::line_segment_sketch::convex_component`,
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
                                `mnist_classifier::line_segment_sketch::convexity`,
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
                                `mnist_classifier::line_segment_sketch::line_segment`,
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
            MajorItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
                    ValkyrieRide {
                        javelin_item_path: JavelinPath::TypeItem(
                            TypeItemPath(`(core::slice::CyclicSlice(0)::first`, `MethodFn`),
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
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide {
                        javelin_item_path: JavelinPath::TraitForTypeItem(
                            TraitForTypeItemPath(
                                ItemPathId {
                                    data: ItemPathData::AssociatedItem(
                                        AssociatedItemPathData::TraitForTypeItem(
                                            TraitForTypeItemPathData {
                                                impl_block: TraitForTypeImplBlock {
                                                    data: TraitForTypeImplBlockPathData {
                                                        module_path: `core::clone`,
                                                        trai_path: TraitPath(`core::clone::Clone`),
                                                        ty_sketch: TypeSketch::DeriveAny,
                                                        disambiguator: 0,
                                                    },
                                                },
                                                ident: `clone`,
                                                item_kind: MethodFn,
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
                                        HirTypeSymbol::SelfType,
                                    ),
                                    HirTermSymbolResolution::Explicit(
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide {
                        javelin_item_path: JavelinPath::TypeItem(
                            TypeItemPath(`(core::slice::CyclicSlice(0)::last`, `MethodFn`),
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
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                ],
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Type(
                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
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
                FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
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
                FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `FunctionFn`),
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
                    ValkyrieRide {
                        javelin_item_path: JavelinPath::TypeItem(
                            TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
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
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                ],
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `FunctionFn`),
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
                    ValkyrieRide {
                        javelin_item_path: JavelinPath::TypeItem(
                            TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
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
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                ],
            },
        ),
    ),
    (
        ItemPath::MajorItem(
            MajorItemPath::Fugitive(
                FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `FunctionFn`),
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
                    ValkyrieRide {
                        javelin_item_path: JavelinPath::TypeItem(
                            TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
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
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide {
                        javelin_item_path: JavelinPath::TypeItem(
                            TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide {
                        javelin_item_path: JavelinPath::TypeItem(
                            TypeItemPath(`(core::vec::Vec(0)::last`, `MethodFn`),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateSymbol::Type(
                                        HirTypeSymbol::SelfPlace,
                                    ),
                                    HirTermSymbolResolution::SelfPlace(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide {
                        javelin_item_path: JavelinPath::TypeItem(
                            TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodFn`),
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
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide {
                        javelin_item_path: JavelinPath::TypeItem(
                            TypeItemPath(`(core::vec::Vec(0)::pop`, `MethodFn`),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateSymbol::Type(
                                        HirTypeSymbol::SelfLifetime,
                                    ),
                                    HirTermSymbolResolution::SelfLifetime,
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide {
                        javelin_item_path: JavelinPath::TypeItem(
                            TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodFn`),
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
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide {
                        javelin_item_path: JavelinPath::TypeItem(
                            TypeItemPath(`(core::vec::Vec(0)::push`, `MethodFn`),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateSymbol::Type(
                                        HirTypeSymbol::SelfLifetime,
                                    ),
                                    HirTermSymbolResolution::SelfLifetime,
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                    ValkyrieRide {
                        javelin_item_path: JavelinPath::TypeItem(
                            TypeItemPath(`(core::vec::Vec(0)::first`, `MethodFn`),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                (
                                    HirTemplateSymbol::Type(
                                        HirTypeSymbol::SelfPlace,
                                    ),
                                    HirTermSymbolResolution::SelfPlace(
                                        MutableStackOwned {
                                            location: StackLocationIdx(
                                                ShiftedU32(
                                                    1,
                                                ),
                                            ),
                                        },
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
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
                        module_path: `mnist_classifier::line_segment_sketch`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
        ItemPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId {
                        data: ItemPathData::AssociatedItem(
                            AssociatedItemPathData::TraitForTypeItem(
                                TraitForTypeItemPathData {
                                    impl_block: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist_classifier::line_segment_sketch`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
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
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(
                    ItemPathId {
                        data: ItemPathData::ImplBlock(
                            ImplBlockPathData::TypeImplBlock(
                                TypeImplBlockPathData {
                                    module_path: `mnist_classifier::line_segment_sketch`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    disambiguator: 0,
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
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`, `AssociatedFunctionFn`),
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
                    ValkyrieRide {
                        javelin_item_path: JavelinPath::TypeItem(
                            TypeItemPath(`(core::vec::Vec(0)::cyclic_slice_leashed`, `MethodFn`),
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
                                                    ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                ],
            },
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`, `MethodFn`),
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
                        module_path: `mnist_classifier::line_segment_sketch`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
        ItemPath::AssociatedItem(
            AssociatedItemPath::TraitForTypeItem(
                TraitForTypeItemPath(
                    ItemPathId {
                        data: ItemPathData::AssociatedItem(
                            AssociatedItemPathData::TraitForTypeItem(
                                TraitForTypeItemPathData {
                                    impl_block: TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist_classifier::line_segment_sketch`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
            ImplBlockPath::TypeImplBlock(
                TypeImplBlockPath(
                    ItemPathId {
                        data: ItemPathData::ImplBlock(
                            ImplBlockPathData::TypeImplBlock(
                                TypeImplBlockPathData {
                                    module_path: `mnist_classifier::line_segment_sketch`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    disambiguator: 0,
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
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::concave_components`, `MemoizedField`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [],
            },
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`, `MemoizedField`),
            ),
        ),
        Some(
            ValkyrieRides {
                hir_template_parameters: None,
                rides: [
                    ValkyrieRide {
                        javelin_item_path: JavelinPath::TypeItem(
                            TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodFn`),
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
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                    template_arguments: [],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                            ],
                            separator: Some(
                                1,
                            ),
                        },
                    },
                ],
            },
        ),
    ),
    (
        ItemPath::AssociatedItem(
            AssociatedItemPath::TypeItem(
                TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`, `AssociatedFunctionFn`),
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
]