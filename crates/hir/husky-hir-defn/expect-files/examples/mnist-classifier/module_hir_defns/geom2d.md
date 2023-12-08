[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
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
                                                `x`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `y`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
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
                                                `x`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `y`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
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
                                                `x`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `y`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
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
                                                `min`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `max`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
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
                                                `xrange`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `yrange`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
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
                                                `xrange`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `yrange`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDefn::Type(
            TypeImplBlockHirDefn {
                hir_decl: TypeImplBlockHirDecl {
                    path: TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
                                        ItemPathId {
                                            data: ItemPathData::ImplBlock(
                                                ImplBlockPathData::TypeImplBlock(
                                                    TypeImplBlockPathData {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
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
            },
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::AssociatedFn(
                TypeAssociatedFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssociatedFunctionFn`),
                    hir_decl: TypeAssociatedFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssociatedFunctionFn`),
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
                                            ty_path: TypePath(`core::num::i32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Ordinary {
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssociatedFunctionFn`),
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
                                                    value: 251,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 272,
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
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `i`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `shift`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            10,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::from_i_shift28`, `AssociatedFunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::I32(
                                                    29,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 1,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 2,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::As {
                                                opd: 3,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::I32(
                                                    29,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 5,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 6,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::As {
                                                opd: 7,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorFnCall {
                                                path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        4,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        8,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 9,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `shift`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                        ],
                                    },
                                    self_value_variable: None,
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::vector`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::vector`, `MethodFn`),
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
                                        TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::vector`, `MethodFn`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            6,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::vector`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorFnCall {
                                                path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        2,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        4,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 5,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::to`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::to`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::to`, `MethodFn`),
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
                                                    value: 55,
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
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `other`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            12,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::to`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 2,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 4,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 6,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 8,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 7,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 9,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorFnCall {
                                                path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        5,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        10,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 11,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `other`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::norm`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::norm`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::norm`, `MethodFn`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            13,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::norm`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 2,
                                                opr: Closed(
                                                    Mul,
                                                ),
                                                ropd: 4,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 6,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 8,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 7,
                                                opr: Closed(
                                                    Mul,
                                                ),
                                                ropd: 9,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 5,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 10,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 11,
                                                self_contract: Pure,
                                                ident: `sqrt`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::sqrt`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 12,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::dist`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::dist`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::dist`, `MethodFn`),
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
                                                    value: 55,
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
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `other`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            5,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::dist`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `to`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::to`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 44,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        2,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 3,
                                                self_contract: Pure,
                                                ident: `norm`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 4,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `other`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDefn::Type(
            TypeImplBlockHirDefn {
                hir_decl: TypeImplBlockHirDecl {
                    path: TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                        ItemPathId {
                                            data: ItemPathData::ImplBlock(
                                                ImplBlockPathData::TypeImplBlock(
                                                    TypeImplBlockPathData {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
            },
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::point`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::point`, `MethodFn`),
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
                                        TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::point`, `MethodFn`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            6,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::point`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorFnCall {
                                                path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        2,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        4,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 5,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::to`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::to`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::to`, `MethodFn`),
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
                                                    value: 55,
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
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `other`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            12,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::to`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 2,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 4,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 6,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 8,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 7,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 9,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorFnCall {
                                                path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        5,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        10,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 11,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `other`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodFn`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            13,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 2,
                                                opr: Closed(
                                                    Mul,
                                                ),
                                                ropd: 4,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 6,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 8,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 7,
                                                opr: Closed(
                                                    Mul,
                                                ),
                                                ropd: 9,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 5,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 10,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 11,
                                                self_contract: Pure,
                                                ident: `sqrt`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::sqrt`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 12,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::dot`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::dot`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::dot`, `MethodFn`),
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
                                                    value: 55,
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
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `other`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            12,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::dot`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 2,
                                                opr: Closed(
                                                    Mul,
                                                ),
                                                ropd: 4,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 6,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 8,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 7,
                                                opr: Closed(
                                                    Mul,
                                                ),
                                                ropd: 9,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 5,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 10,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 11,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `other`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::cross`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::cross`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::cross`, `MethodFn`),
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
                                                    value: 55,
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
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `other`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            12,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::cross`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 2,
                                                opr: Closed(
                                                    Mul,
                                                ),
                                                ropd: 4,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 6,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 8,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 7,
                                                opr: Closed(
                                                    Mul,
                                                ),
                                                ropd: 9,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 5,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 10,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 11,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `other`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle`, `MethodFn`),
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
                                                    value: 351,
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
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `is_branch_cut_positive`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            28,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 3,
                                                self_contract: Pure,
                                                ident: `norm`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 2,
                                                opr: Closed(
                                                    Div,
                                                ),
                                                ropd: 4,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            1.0,
                                                        ),
                                                        text: "1.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 5,
                                                self_contract: Pure,
                                                ident: `min`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::min`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        6,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Const,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            1.0,
                                                        ),
                                                        text: "1.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 8,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 9,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            0.001,
                                                        ),
                                                        text: "0.001f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 10,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                ropd: 11,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            180.0,
                                                        ),
                                                        text: "180.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            180.0,
                                                        ),
                                                        text: "180.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 15,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 17,
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 18,
                                                self_contract: Pure,
                                                ident: `sgnx`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::sgnx`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::As {
                                                opd: 19,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 21,
                                                self_contract: Pure,
                                                ident: `acos`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::acos`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 20,
                                                opr: Closed(
                                                    Mul,
                                                ),
                                                ropd: 22,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            180.0,
                                                        ),
                                                        text: "180.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 23,
                                                opr: Closed(
                                                    Mul,
                                                ),
                                                ropd: 24,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            3.1415925,
                                                        ),
                                                        text: "3.1415926f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 25,
                                                opr: Closed(
                                                    Div,
                                                ),
                                                ropd: 26,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    5..7,
                                                ),
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 14,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Const,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 16,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Const,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 13,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                HirEagerElseBranch {
                                                    stmts: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                },
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 27,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 7,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 12,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    3..4,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                HirEagerElseBranch {
                                                    stmts: ArenaIdxRange(
                                                        4..5,
                                                    ),
                                                },
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 352,
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
                                                name: HirEagerRuntimeSymbolName::SelfValue,
                                                data: HirEagerRuntimeSymbolData::SelfValue,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `is_branch_cut_positive`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `cos_value`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`, `MethodFn`),
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
                                                    value: 55,
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
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `other`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            5,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `cross`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::cross`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 64,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        2,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 3,
                                                self_contract: Pure,
                                                ident: `sgnx`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::sgnx`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 4,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `other`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle_to`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle_to`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
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
                                HirEagerParenateParameter::Ordinary {
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle_to`, `MethodFn`),
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
                                                    value: 55,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 351,
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
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `other`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `is_branch_cut_positive`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            41,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::angle_to`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `norm`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            0.0,
                                                        ),
                                                        text: "0.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 3,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 4,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 6,
                                                self_contract: Pure,
                                                ident: `norm`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            0.0,
                                                        ),
                                                        text: "0.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 8,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 9,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 11,
                                                self_contract: Pure,
                                                ident: `dot`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::dot`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 64,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        12,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 14,
                                                opr: Closed(
                                                    Mul,
                                                ),
                                                ropd: 15,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 13,
                                                opr: Closed(
                                                    Div,
                                                ),
                                                ropd: 16,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            1.0,
                                                        ),
                                                        text: "1.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 17,
                                                self_contract: Pure,
                                                ident: `min`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::min`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        18,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Const,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            1.0,
                                                        ),
                                                        text: "1.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 20,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 21,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            0.001,
                                                        ),
                                                        text: "0.001f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 22,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                ropd: 23,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            180.0,
                                                        ),
                                                        text: "180.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            180.0,
                                                        ),
                                                        text: "180.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 27,
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 29,
                                                self_contract: Pure,
                                                ident: `rotation_direction_to`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 64,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        30,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            1,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::As {
                                                opd: 31,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 33,
                                                self_contract: Pure,
                                                ident: `acos`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::acos`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 32,
                                                opr: Closed(
                                                    Mul,
                                                ),
                                                ropd: 34,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                7,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            180.0,
                                                        ),
                                                        text: "180.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 36,
                                                opr: Closed(
                                                    Mul,
                                                ),
                                                ropd: 37,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                TermLiteral::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            3.1415925,
                                                        ),
                                                        text: "3.1415926f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 38,
                                                opr: Closed(
                                                    Div,
                                                ),
                                                ropd: 39,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    6..12,
                                                ),
                                            },
                                            ty_place: Const,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 26,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Const,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 28,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Const,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 25,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                HirEagerElseBranch {
                                                    stmts: ArenaIdxRange(
                                                        2..3,
                                                    ),
                                                },
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 35,
                                        },
                                        Eval {
                                            expr_idx: 40,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 2,
                                        },
                                        Assert {
                                            condition: Other {
                                                hir_eager_expr_idx: 5,
                                                conversion: None,
                                            },
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 7,
                                        },
                                        Assert {
                                            condition: Other {
                                                hir_eager_expr_idx: 10,
                                                conversion: None,
                                            },
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 19,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 24,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    3..4,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                HirEagerElseBranch {
                                                    stmts: ArenaIdxRange(
                                                        4..6,
                                                    ),
                                                },
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 355,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 356,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 352,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 357,
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
                                                name: HirEagerRuntimeSymbolName::SelfValue,
                                                data: HirEagerRuntimeSymbolData::SelfValue,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `other`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `is_branch_cut_positive`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `self_norm`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `other_norm`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `cos_value`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `arc_angle`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDefn::Type(
            TypeImplBlockHirDefn {
                hir_decl: TypeImplBlockHirDecl {
                    path: TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
                                        ItemPathId {
                                            data: ItemPathData::ImplBlock(
                                                ImplBlockPathData::TypeImplBlock(
                                                    TypeImplBlockPathData {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
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
            },
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_range`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_range`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_range`, `MethodFn`),
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
                                                    value: 55,
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
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `other`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            28,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_range`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `max`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: `min`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 2,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 4,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 6,
                                                ident: `max`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 8,
                                                ident: `min`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 7,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 9,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 11,
                                                ident: `min`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 13,
                                                ident: `min`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 12,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 14,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 15,
                                                opr: Closed(
                                                    Div,
                                                ),
                                                ropd: 16,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 18,
                                                ident: `max`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 20,
                                                ident: `min`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 19,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 21,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 22,
                                                opr: Closed(
                                                    Div,
                                                ),
                                                ropd: 23,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorFnCall {
                                                path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        25,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            2,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        26,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..6,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Assert {
                                            condition: Other {
                                                hir_eager_expr_idx: 5,
                                                conversion: None,
                                            },
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 10,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 17,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 24,
                                        },
                                        Eval {
                                            expr_idx: 27,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 123,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 359,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 360,
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
                                                name: HirEagerRuntimeSymbolName::SelfValue,
                                                data: HirEagerRuntimeSymbolData::SelfValue,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `other`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `span`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `rel_min`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `rel_max`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_point`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_point`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_point`, `MethodFn`),
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
                                                    value: 194,
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
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `v`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            12,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_point`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `max`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: `min`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 2,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 4,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 7,
                                                ident: `min`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 6,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 8,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 9,
                                                opr: Closed(
                                                    Div,
                                                ),
                                                ropd: 10,
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..3,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 5,
                                        },
                                        Eval {
                                            expr_idx: 11,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 123,
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
                                                name: HirEagerRuntimeSymbolName::SelfValue,
                                                data: HirEagerRuntimeSymbolData::SelfValue,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `v`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `span`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDefn::Type(
            TypeImplBlockHirDefn {
                hir_decl: TypeImplBlockHirDecl {
                    path: TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
                                        ItemPathId {
                                            data: ItemPathData::ImplBlock(
                                                ImplBlockPathData::TypeImplBlock(
                                                    TypeImplBlockPathData {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
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
            },
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`, `MethodFn`),
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
                                                    value: 55,
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
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `other`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            12,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `xrange`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: `xrange`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 2,
                                                self_contract: Pure,
                                                ident: `relative_range`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_range`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 63,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        4,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 6,
                                                ident: `yrange`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 8,
                                                ident: `yrange`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 7,
                                                self_contract: Pure,
                                                ident: `relative_range`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_range`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 63,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        9,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorFnCall {
                                                path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 63,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        5,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 63,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        10,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 11,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `other`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_point`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_point`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_point`, `MethodFn`),
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
                                                    value: 289,
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
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `point`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: Some(
                                    1,
                                ),
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            12,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::relative_point`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `xrange`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 3,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 2,
                                                self_contract: Pure,
                                                ident: `relative_point`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_point`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        4,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 6,
                                                ident: `yrange`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 8,
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 7,
                                                self_contract: Pure,
                                                ident: `relative_point`,
                                                path: AssociatedItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::ClosedRange(0)::relative_point`, `MethodFn`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        9,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorFnCall {
                                                path: TypePath(`mnist_classifier::geom2d::RelativePoint2d`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        5,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 15,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        10,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 11,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `point`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        1,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmin`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmin`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmin`, `MethodFn`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            4,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmin`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `xrange`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 2,
                                                ident: `min`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 3,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmax`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmax`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmax`, `MethodFn`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            4,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::xmax`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `xrange`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 2,
                                                ident: `max`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 3,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodFn`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            4,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymin`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `yrange`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 2,
                                                ident: `min`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 3,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodFn`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            4,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::BoundingBox(0)::ymax`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `yrange`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 2,
                                                ident: `max`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 3,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDefn::Type(
            TypeImplBlockHirDefn {
                hir_decl: TypeImplBlockHirDecl {
                    path: TypeImplBlockPath(
                        ItemPathId {
                            data: ItemPathData::ImplBlock(
                                ImplBlockPathData::TypeImplBlock(
                                    TypeImplBlockPathData {
                                        module_path: `mnist_classifier::geom2d`,
                                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
                                        ItemPathId {
                                            data: ItemPathData::ImplBlock(
                                                ImplBlockPathData::TypeImplBlock(
                                                    TypeImplBlockPathData {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
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
            },
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmin`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmin`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmin`, `MethodFn`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            4,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmin`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `xrange`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 2,
                                                ident: `min`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 3,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`, `MethodFn`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            4,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::xmax`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `xrange`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 2,
                                                ident: `max`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 3,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`, `MethodFn`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            4,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymin`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `yrange`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 2,
                                                ident: `min`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 3,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
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
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`, `MethodFn`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            4,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: `yrange`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: false,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 2,
                                                ident: `max`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_ty_always_copyable: true,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 3,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
]