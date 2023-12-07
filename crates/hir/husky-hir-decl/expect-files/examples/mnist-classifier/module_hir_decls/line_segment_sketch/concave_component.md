[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            PropsStruct(
                PropsStructTypeHirDecl(
                    Id {
                        value: 11,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::FunctionFn(
                FunctionFnFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Ordinary {
                                pattern_expr_idx: 1,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            HirTemplateArgument::Type(
                                                HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            ),
                                        ],
                                        always_copyable: true,
                                    },
                                ),
                            },
                        ],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::vec::Vec`, `Extern`),
                            template_arguments: [
                                HirTemplateArgument::Type(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            template_arguments: [],
                                            always_copyable: false,
                                        },
                                    ),
                                ),
                            ],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::MajorItem(
                                MajorItemPath::Fugitive(
                                    FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [
                                Ident {
                                    symbol_modifier: None,
                                    ident: Ident(
                                        Coword(
                                            Id {
                                                value: 211,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::Ident(
                                            `line_segment_sketch`,
                                        ),
                                        data: HirEagerRuntimeSymbolData::ParenateParameter,
                                    },
                                ],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlock {
                    data: TraitForTypeImplBlockPathData {
                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        ),
                        disambiguator: 0,
                    },
                },
                template_parameters: HirTemplateParameters(
                    [],
                ),
                trai: HirTrait {
                    trai_path: TraitPath(`core::visual::Visualize`),
                    template_arguments: [],
                },
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        template_arguments: [],
                        always_copyable: false,
                    },
                ),
                hir_eager_expr_region: HirEagerExprRegion {
                    path: RegionPath::Decl(
                        ItemPath::ImplBlock(
                            ImplBlockPath::TraitForTypeImplBlock(
                                TraitForTypeImplBlock {
                                    data: TraitForTypeImplBlockPathData {
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                        ),
                    ),
                    expr_arena: Arena {
                        data: [],
                    },
                    stmt_arena: Arena {
                        data: [],
                    },
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                        arena: Arena {
                            data: [],
                        },
                    },
                    runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                        arena: Arena {
                            data: [],
                        },
                        self_value_variable: None,
                    },
                },
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TraitForTypeItem(
            TraitForTypeItemHirDecl::MethodFn(
                TraitForTypeMethodFnHirDecl {
                    path: TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssociatedItem(
                                AssociatedItemPathData::TraitForTypeItem(
                                    TraitForTypeItemPathData {
                                        impl_block: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter,
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::visual::Html`, `Extern`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssociatedItem(
                                AssociatedItemPath::TraitForTypeItem(
                                    TraitForTypeItemPath(
                                        ItemPathId {
                                            data: ItemPathData::AssociatedItem(
                                                AssociatedItemPathData::TraitForTypeItem(
                                                    TraitForTypeItemPathData {
                                                        impl_block: TraitForTypeImplBlock {
                                                            data: TraitForTypeImplBlockPathData {
                                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                                ty_sketch: TypeSketch::Path(
                                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::SelfValue,
                                        data: HirEagerRuntimeSymbolData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                1,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath(
                    ItemPathId {
                        data: ItemPathData::ImplBlock(
                            ImplBlockPathData::TypeImplBlock(
                                TypeImplBlockPathData {
                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                    disambiguator: 0,
                                },
                            ),
                        ),
                    },
                ),
                template_parameters: HirTemplateParameters(
                    [],
                ),
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                        template_arguments: [],
                        always_copyable: false,
                    },
                ),
                hir_eager_expr_region: HirEagerExprRegion {
                    path: RegionPath::Decl(
                        ItemPath::ImplBlock(
                            ImplBlockPath::TypeImplBlock(
                                TypeImplBlockPath(
                                    ItemPathId {
                                        data: ItemPathData::ImplBlock(
                                            ImplBlockPathData::TypeImplBlock(
                                                TypeImplBlockPathData {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                    ),
                    expr_arena: Arena {
                        data: [],
                    },
                    stmt_arena: Arena {
                        data: [],
                    },
                    pattern_expr_arena: Arena {
                        data: [],
                    },
                    comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                        arena: Arena {
                            data: [],
                        },
                    },
                    runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                        arena: Arena {
                            data: [],
                        },
                        self_value_variable: None,
                    },
                },
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MemoizedField(
                TypeMemoizedFieldHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`, `MemoizedField`),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssociatedItem(
                                AssociatedItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`, `MemoizedField`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::SelfValue,
                                        data: HirEagerRuntimeSymbolData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                1,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MemoizedField(
                TypeMemoizedFieldHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`, `MemoizedField`),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssociatedItem(
                                AssociatedItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`, `MemoizedField`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::SelfValue,
                                        data: HirEagerRuntimeSymbolData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                1,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MemoizedField(
                TypeMemoizedFieldHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`, `MemoizedField`),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssociatedItem(
                                AssociatedItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`, `MemoizedField`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::SelfValue,
                                        data: HirEagerRuntimeSymbolData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                1,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MemoizedField(
                TypeMemoizedFieldHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`, `MemoizedField`),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssociatedItem(
                                AssociatedItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`, `MemoizedField`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::SelfValue,
                                        data: HirEagerRuntimeSymbolData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                1,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MemoizedField(
                TypeMemoizedFieldHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`, `MemoizedField`),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssociatedItem(
                                AssociatedItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`, `MemoizedField`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::SelfValue,
                                        data: HirEagerRuntimeSymbolData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                1,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MemoizedField(
                TypeMemoizedFieldHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`, `MemoizedField`),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssociatedItem(
                                AssociatedItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`, `MemoizedField`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::SelfValue,
                                        data: HirEagerRuntimeSymbolData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                1,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`, `MethodFn`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter,
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssociatedItem(
                                AssociatedItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`, `MethodFn`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::SelfValue,
                                        data: HirEagerRuntimeSymbolData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                1,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`, `MethodFn`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter,
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssociatedItem(
                                AssociatedItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`, `MethodFn`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::SelfValue,
                                        data: HirEagerRuntimeSymbolData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                1,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`, `MethodFn`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter,
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssociatedItem(
                                AssociatedItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`, `MethodFn`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::SelfValue,
                                        data: HirEagerRuntimeSymbolData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                1,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter,
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssociatedItem(
                                AssociatedItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::SelfValue,
                                        data: HirEagerRuntimeSymbolData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                1,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`, `MethodFn`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter,
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssociatedItem(
                                AssociatedItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`, `MethodFn`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::SelfValue,
                                        data: HirEagerRuntimeSymbolData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                1,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`, `MethodFn`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter,
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssociatedItem(
                                AssociatedItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`, `MethodFn`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_expr_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSymbolEntry {
                                        name: HirEagerRuntimeSymbolName::SelfValue,
                                        data: HirEagerRuntimeSymbolData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                1,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
]