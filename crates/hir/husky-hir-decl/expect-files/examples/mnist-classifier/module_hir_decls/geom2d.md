[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::PropsStruct(
                PropsStructTypeHirDecl {
                    path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    fields: [
                        PropsStructFieldHirDecl {
                            ident: `x`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
                            ),
                            initialization: None,
                        },
                        PropsStructFieldHirDecl {
                            ident: `y`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                    template_arguments: [],
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
                                    TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                            `x`,
                                        ),
                                        data: HirEagerRuntimeSvarData::FieldVariable,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `y`,
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
        MajorItemHirDecl::Type(
            TypeHirDecl::PropsStruct(
                PropsStructTypeHirDecl {
                    path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    fields: [
                        PropsStructFieldHirDecl {
                            ident: `x`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
                            ),
                            initialization: None,
                        },
                        PropsStructFieldHirDecl {
                            ident: `y`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                    template_arguments: [],
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
                                    TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
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
                                            `x`,
                                        ),
                                        data: HirEagerRuntimeSvarData::FieldVariable,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `y`,
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
        MajorItemHirDecl::Type(
            TypeHirDecl::PropsStruct(
                PropsStructTypeHirDecl {
                    path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    fields: [
                        PropsStructFieldHirDecl {
                            ident: `x`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
                            ),
                            initialization: None,
                        },
                        PropsStructFieldHirDecl {
                            ident: `y`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                    template_arguments: [],
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
                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                            `x`,
                                        ),
                                        data: HirEagerRuntimeSvarData::FieldVariable,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `y`,
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
        MajorItemHirDecl::Type(
            TypeHirDecl::PropsStruct(
                PropsStructTypeHirDecl {
                    path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    fields: [
                        PropsStructFieldHirDecl {
                            ident: `min`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
                            ),
                            initialization: None,
                        },
                        PropsStructFieldHirDecl {
                            ident: `max`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::f32`, `Extern`),
                                    template_arguments: [],
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
                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                            `min`,
                                        ),
                                        data: HirEagerRuntimeSvarData::FieldVariable,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `max`,
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
        MajorItemHirDecl::Type(
            TypeHirDecl::PropsStruct(
                PropsStructTypeHirDecl {
                    path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    fields: [
                        PropsStructFieldHirDecl {
                            ident: `xrange`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    template_arguments: [],
                                    always_copyable: false,
                                },
                            ),
                            initialization: None,
                        },
                        PropsStructFieldHirDecl {
                            ident: `yrange`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    template_arguments: [],
                                    always_copyable: false,
                                },
                            ),
                            initialization: None,
                        },
                    ],
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                            `xrange`,
                                        ),
                                        data: HirEagerRuntimeSvarData::FieldVariable,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `yrange`,
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
        MajorItemHirDecl::Type(
            TypeHirDecl::PropsStruct(
                PropsStructTypeHirDecl {
                    path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    fields: [
                        PropsStructFieldHirDecl {
                            ident: `xrange`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    template_arguments: [],
                                    always_copyable: false,
                                },
                            ),
                            initialization: None,
                        },
                        PropsStructFieldHirDecl {
                            ident: `yrange`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                    template_arguments: [],
                                    always_copyable: false,
                                },
                            ),
                            initialization: None,
                        },
                    ],
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                            `xrange`,
                                        ),
                                        data: HirEagerRuntimeSvarData::FieldVariable,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `yrange`,
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
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath(
                    ItemPathId(
                        Id {
                            value: 286,
                        },
                    ),
                ),
                template_parameters: HirTemplateParameters(
                    [],
                ),
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                            value: 286,
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
            TypeItemHirDecl::AssocFn(
                TypeAssocFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssocRitchie(
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
                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                        template_arguments: [],
                                        always_copyable: true,
                                    },
                                ),
                            },
                            HirEagerParenateParameter::Simple {
                                pattern_expr_idx: 2,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                        template_arguments: [],
                                        always_copyable: true,
                                    },
                                ),
                            },
                        ],
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
                                    TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssocRitchie(
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
                                    ident: `i`,
                                },
                                HirEagerPatternExpr::Ident {
                                    symbol_modifier: None,
                                    ident: `shift`,
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
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `i`,
                                        ),
                                        data: HirEagerRuntimeSvarData::ParenateParameter,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `shift`,
                                        ),
                                        data: HirEagerRuntimeSvarData::ParenateParameter,
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::vector`, `MethodRitchie(
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
                                    value: 30,
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
                                    TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::vector`, `MethodRitchie(
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::to`, `MethodRitchie(
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
                                    value: 30,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
                                pattern_expr_idx: 1,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
                                ),
                            },
                        ],
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
                                    TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::to`, `MethodRitchie(
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
                                    ident: `other`,
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
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `other`,
                                        ),
                                        data: HirEagerRuntimeSvarData::ParenateParameter,
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::norm`, `MethodRitchie(
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
                                    value: 30,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::norm`, `MethodRitchie(
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::dist`, `MethodRitchie(
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
                                    value: 30,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
                                pattern_expr_idx: 1,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
                                ),
                            },
                        ],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::dist`, `MethodRitchie(
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
                                    ident: `other`,
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
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `other`,
                                        ),
                                        data: HirEagerRuntimeSvarData::ParenateParameter,
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
                            value: 287,
                        },
                    ),
                ),
                template_parameters: HirTemplateParameters(
                    [],
                ),
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                            value: 287,
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
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::point`, `MethodRitchie(
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
                                    value: 31,
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
                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::point`, `MethodRitchie(
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::to`, `MethodRitchie(
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
                                    value: 31,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
                                pattern_expr_idx: 1,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
                                ),
                            },
                        ],
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
                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::to`, `MethodRitchie(
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
                                    ident: `other`,
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
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `other`,
                                        ),
                                        data: HirEagerRuntimeSvarData::ParenateParameter,
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodRitchie(
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
                                    value: 31,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodRitchie(
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::dot`, `MethodRitchie(
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
                                    value: 31,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
                                pattern_expr_idx: 1,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
                                ),
                            },
                        ],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::dot`, `MethodRitchie(
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
                                    ident: `other`,
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
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `other`,
                                        ),
                                        data: HirEagerRuntimeSvarData::ParenateParameter,
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::cross`, `MethodRitchie(
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
                                    value: 31,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
                                pattern_expr_idx: 1,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
                                ),
                            },
                        ],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::cross`, `MethodRitchie(
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
                                    ident: `other`,
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
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `other`,
                                        ),
                                        data: HirEagerRuntimeSvarData::ParenateParameter,
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle`, `MethodRitchie(
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
                                    value: 31,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
                                pattern_expr_idx: 1,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::basic::bool`, `Extern`),
                                        template_arguments: [],
                                        always_copyable: true,
                                    },
                                ),
                            },
                        ],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle`, `MethodRitchie(
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
                                    ident: `is_branch_cut_positive`,
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
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `is_branch_cut_positive`,
                                        ),
                                        data: HirEagerRuntimeSvarData::ParenateParameter,
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`, `MethodRitchie(
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
                                    value: 31,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
                                pattern_expr_idx: 1,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
                                ),
                            },
                        ],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::i32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`, `MethodRitchie(
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
                                    ident: `other`,
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
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `other`,
                                        ),
                                        data: HirEagerRuntimeSvarData::ParenateParameter,
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle_to`, `MethodRitchie(
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
                                    value: 31,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
                                pattern_expr_idx: 1,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
                                ),
                            },
                            HirEagerParenateParameter::Simple {
                                pattern_expr_idx: 2,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::basic::bool`, `Extern`),
                                        template_arguments: [],
                                        always_copyable: true,
                                    },
                                ),
                            },
                        ],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle_to`, `MethodRitchie(
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
                                    ident: `other`,
                                },
                                HirEagerPatternExpr::Ident {
                                    symbol_modifier: None,
                                    ident: `is_branch_cut_positive`,
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
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `other`,
                                        ),
                                        data: HirEagerRuntimeSvarData::ParenateParameter,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `is_branch_cut_positive`,
                                        ),
                                        data: HirEagerRuntimeSvarData::ParenateParameter,
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
                            value: 288,
                        },
                    ),
                ),
                template_parameters: HirTemplateParameters(
                    [],
                ),
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                            value: 288,
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
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_range`, `MethodRitchie(
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
                                    value: 29,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
                                pattern_expr_idx: 1,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
                                ),
                            },
                        ],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_range`, `MethodRitchie(
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
                                    ident: `other`,
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
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `other`,
                                        ),
                                        data: HirEagerRuntimeSvarData::ParenateParameter,
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_point`, `MethodRitchie(
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
                                    value: 29,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
                                pattern_expr_idx: 1,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                        template_arguments: [],
                                        always_copyable: true,
                                    },
                                ),
                            },
                        ],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_point`, `MethodRitchie(
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
                                    ident: `v`,
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
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `v`,
                                        ),
                                        data: HirEagerRuntimeSvarData::ParenateParameter,
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
                            value: 289,
                        },
                    ),
                ),
                template_parameters: HirTemplateParameters(
                    [],
                ),
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                            value: 289,
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
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`, `MethodRitchie(
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
                                    value: 33,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
                                pattern_expr_idx: 1,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
                                ),
                            },
                        ],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`, `MethodRitchie(
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
                                    ident: `other`,
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
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `other`,
                                        ),
                                        data: HirEagerRuntimeSvarData::ParenateParameter,
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_point`, `MethodRitchie(
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
                                    value: 33,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
                                pattern_expr_idx: 1,
                                contract: Pure,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
                                ),
                            },
                        ],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_point`, `MethodRitchie(
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
                                    ident: `point`,
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
                                        name: HirEagerRuntimeSvarName::SelfValue,
                                        data: HirEagerRuntimeSvarData::SelfValue,
                                    },
                                    HirEagerRuntimeSvarEntry {
                                        name: HirEagerRuntimeSvarName::Ident(
                                            `point`,
                                        ),
                                        data: HirEagerRuntimeSvarData::ParenateParameter,
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmin`, `MethodRitchie(
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
                                    value: 33,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmin`, `MethodRitchie(
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmax`, `MethodRitchie(
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
                                    value: 33,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmax`, `MethodRitchie(
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodRitchie(
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
                                    value: 33,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodRitchie(
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodRitchie(
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
                                    value: 33,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodRitchie(
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
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
                path: TypeImplBlockPath(
                    ItemPathId(
                        Id {
                            value: 290,
                        },
                    ),
                ),
                template_parameters: HirTemplateParameters(
                    [],
                ),
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                            value: 290,
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
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
                    path: TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmin`, `MethodRitchie(
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
                                    value: 34,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmin`, `MethodRitchie(
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`, `MethodRitchie(
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
                                    value: 34,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`, `MethodRitchie(
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`, `MethodRitchie(
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
                                    value: 34,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`, `MethodRitchie(
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
                    path: TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`, `MethodRitchie(
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
                                    value: 34,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [],
                    ),
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
                                    TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`, `MethodRitchie(
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