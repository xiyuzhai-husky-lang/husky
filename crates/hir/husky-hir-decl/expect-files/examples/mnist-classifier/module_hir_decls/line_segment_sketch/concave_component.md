[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::PropsStruct(
                PropsStructTypeHirDecl {
                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    fields: [
                        PropsStructFieldHirDecl {
                            ident: `line_segment_sketch`,
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
                            initialization: None,
                        },
                        PropsStructFieldHirDecl {
                            ident: `strokes`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                    template_arguments: [
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                                    template_arguments: [],
                                                                    always_copyable: false,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: false,
                                                },
                                            ),
                                        ),
                                    ],
                                    always_copyable: true,
                                },
                            ),
                            initialization: None,
                        },
                    ],
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `line_segment_sketch`,
                                        ),
                                        data: HirEagerRuntimeSvarData::FieldVariable,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `strokes`,
                                        ),
                                        data: HirEagerRuntimeSvarData::FieldVariable,
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
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::FunctionFn(
                FunctionMajorFnHirDecl {
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                        Fn,
                    )`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
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
                                    FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                        Fn,
                                    )`),
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
                                HirEagerPatternExpr::Ident {
                                    symbol_modifier: None,
                                    ident: `line_segment_sketch`,
                                },
                            ],
                        },
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `line_segment_sketch`,
                                        ),
                                        data: HirEagerRuntimeSvarData::ParenateParameter,
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
                    comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                        arena: Arena {
                            data: [],
                        },
                    },
                    runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                        arena: Arena {
                            data: [],
                        },
                        self_value_variable: None,
                    },
                },
            },
        ),
    ),
    HirDecl::AssocItem(
        AssocItemHirDecl::TraitForTypeItem(
            TraitForTypeItemHirDecl::MethodFn(
                TraitForTypeMethodFnHirDecl {
                    path: TraitForTypeItemPath(
                        ItemPathId(
                            Id {
                                value: 352,
                            },
                        ),
                    ),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter {
                        contract: Pure,
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 21,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::visual::Visual`, `Extern`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssocItem(
                                AssocItemPath::TraitForTypeItem(
                                    TraitForTypeItemPath(
                                        ItemPathId(
                                            Id {
                                                value: 352,
                                            },
                                        ),
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
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
                    ItemPathId(
                        Id {
                            value: 296,
                        },
                    ),
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
                                    ItemPathId(
                                        Id {
                                            value: 296,
                                        },
                                    ),
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
                    comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                        arena: Arena {
                            data: [],
                        },
                    },
                    runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                        arena: Arena {
                            data: [],
                        },
                        self_value_variable: None,
                    },
                },
            },
        ),
    ),
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MemoizedField(
                TypeMemoFieldHirDecl {
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
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
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
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MemoizedField(
                TypeMemoFieldHirDecl {
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
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
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
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MemoizedField(
                TypeMemoFieldHirDecl {
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
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
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
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MemoizedField(
                TypeMemoFieldHirDecl {
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
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
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
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MemoizedField(
                TypeMemoFieldHirDecl {
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
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
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
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MemoizedField(
                TypeMemoFieldHirDecl {
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
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
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
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`, `MethodRitchie(
                        Fn,
                    )`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter {
                        contract: Pure,
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 21,
                                },
                            ),
                        ),
                    },
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
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`, `MethodRitchie(
                                        Fn,
                                    )`),
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
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
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`, `MethodRitchie(
                        Fn,
                    )`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter {
                        contract: Pure,
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 21,
                                },
                            ),
                        ),
                    },
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
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`, `MethodRitchie(
                                        Fn,
                                    )`),
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
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
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`, `MethodRitchie(
                        Fn,
                    )`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter {
                        contract: Pure,
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 21,
                                },
                            ),
                        ),
                    },
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
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`, `MethodRitchie(
                                        Fn,
                                    )`),
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
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
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodRitchie(
                        Fn,
                    )`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter {
                        contract: Pure,
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 21,
                                },
                            ),
                        ),
                    },
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
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodRitchie(
                                        Fn,
                                    )`),
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
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
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`, `MethodRitchie(
                        Fn,
                    )`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter {
                        contract: Pure,
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 21,
                                },
                            ),
                        ),
                    },
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
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`, `MethodRitchie(
                                        Fn,
                                    )`),
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
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
    HirDecl::AssocItem(
        AssocItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`, `MethodRitchie(
                        Fn,
                    )`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_value_parameter: HirEagerSelfValueParameter {
                        contract: Pure,
                        self_ty: PathLeading(
                            HirTypePathLeading(
                                Id {
                                    value: 21,
                                },
                            ),
                        ),
                    },
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
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`, `MethodRitchie(
                                        Fn,
                                    )`),
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
                        comptime_symbol_region_data: HirEagerComptimeSvarRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeSvarRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
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