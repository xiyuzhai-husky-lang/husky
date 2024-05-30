```rust
[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::PropsStruct(
                PropsStructHirDecl {
                    path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    fields: [
                        PropsStructFieldHirDecl {
                            ident: `row_start`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
                            ),
                            initialization: None,
                        },
                        PropsStructFieldHirDecl {
                            ident: `row_end`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
                            ),
                            initialization: None,
                        },
                        PropsStructFieldHirDecl {
                            ident: `upper_mass`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
                            ),
                            initialization: None,
                        },
                        PropsStructFieldHirDecl {
                            ident: `lower_mass`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::num::i32`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: true,
                                },
                            ),
                            initialization: None,
                        },
                    ],
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::Ident(
                                            `row_start`,
                                        ),
                                        data: HirEagerRuntimeVariableData::FieldVariable,
                                    },
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::Ident(
                                            `row_end`,
                                        ),
                                        data: HirEagerRuntimeVariableData::FieldVariable,
                                    },
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::Ident(
                                            `upper_mass`,
                                        ),
                                        data: HirEagerRuntimeVariableData::FieldVariable,
                                    },
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::Ident(
                                            `lower_mass`,
                                        ),
                                        data: HirEagerRuntimeVariableData::FieldVariable,
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
                PropsStructHirDecl {
                    path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    fields: [
                        PropsStructFieldHirDecl {
                            ident: `matches`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                    template_arguments: [
                                        HirTemplateArgument::Type(
                                            HirType::PathLeading(
                                                HirTypePathLeading {
                                                    ty_path: TypePath(`core::option::Option`, `Enum`),
                                                    template_arguments: [
                                                        HirTemplateArgument::Type(
                                                            HirType::PathLeading(
                                                                HirTypePathLeading {
                                                                    ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                                                    template_arguments: [
                                                                        HirTemplateArgument::Type(
                                                                            HirType::PathLeading(
                                                                                HirTypePathLeading {
                                                                                    ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                                                    template_arguments: [],
                                                                                    always_copyable: false,
                                                                                },
                                                                            ),
                                                                        ),
                                                                    ],
                                                                    always_copyable: true,
                                                                },
                                                            ),
                                                        ),
                                                    ],
                                                    always_copyable: true,
                                                },
                                            ),
                                        ),
                                    ],
                                    always_copyable: false,
                                },
                            ),
                            initialization: None,
                        },
                    ],
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::Ident(
                                            `matches`,
                                        ),
                                        data: HirEagerRuntimeVariableData::FieldVariable,
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
        MajorItemHirDecl::Form(
            MajorFormHirDecl::Ritchie(
                MajorFunctionRitchieHirDecl {
                    path: FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Ritchie(
                        Fn,
                    )`),
                    ritchie_item_kind: RitchieItemKind::Fn,
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    parenate_parameters: HirParenateParameters::Eager(
                        HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Simple {
                                    pattern_idx: 0,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::option::Option`, `Enum`),
                            template_arguments: [
                                HirTemplateArgument::Type(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::f32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                ),
                            ],
                            always_copyable: true,
                        },
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 4,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TypeHirDecl::PropsStruct(
                PropsStructHirDecl {
                    path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    fields: [
                        PropsStructFieldHirDecl {
                            ident: `mask`,
                            ty: HirType::PathLeading(
                                HirTypePathLeading {
                                    ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                    template_arguments: [],
                                    always_copyable: false,
                                },
                            ),
                            initialization: None,
                        },
                    ],
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath::MajorItem(
                                MajorItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                ),
                            ),
                        ),
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::Ident(
                                            `mask`,
                                        ),
                                        data: HirEagerRuntimeVariableData::FieldVariable,
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
        MajorItemHirDecl::Form(
            MajorFormHirDecl::Ritchie(
                MajorFunctionRitchieHirDecl {
                    path: FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Ritchie(
                        Fn,
                    )`),
                    ritchie_item_kind: RitchieItemKind::Fn,
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    parenate_parameters: HirParenateParameters::Eager(
                        HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Simple {
                                    pattern_idx: 0,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                                HirEagerParenateParameter::Simple {
                                    pattern_idx: 1,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::raw_bits::r32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 6,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Form(
            MajorFormHirDecl::Ritchie(
                MajorFunctionRitchieHirDecl {
                    path: FormPath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
                        Fn,
                    )`),
                    ritchie_item_kind: RitchieItemKind::Fn,
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    parenate_parameters: HirParenateParameters::Eager(
                        HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Simple {
                                    pattern_idx: 0,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: false,
                                        },
                                    ),
                                },
                            ],
                        ),
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::vec::Vec`, `Extern`),
                            template_arguments: [
                                HirTemplateArgument::Type(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                            template_arguments: [],
                                            always_copyable: false,
                                        },
                                    ),
                                ),
                            ],
                            always_copyable: false,
                        },
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 7,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)`),
                template_parameters: HirTemplateParameters(
                    [],
                ),
                trai: HirTrait {
                    trai_path: TraitPath(`core::visual::Visualize`),
                    template_arguments: [],
                },
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        template_arguments: [],
                        always_copyable: false,
                    },
                ),
                hir_eager_expr_region: HirEagerExprRegion {
                    region_path: RegionPath::ItemDecl(
                        ItemPath::ImplBlock(
                            ImplBlockPath::TraitForTypeImplBlock(
                                TraitForTypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)`),
                            ),
                        ),
                    ),
                    expr_arena: Arena {
                        data: [],
                    },
                    stmt_arena: Arena {
                        data: [],
                    },
                    pattern_arena: Arena {
                        data: [],
                    },
                    comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                        arena: Arena {
                            data: [],
                        },
                    },
                    runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
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
                TraitForTypeMethodRitchieHirDecl {
                    path: TraitForTypeItemPath(
                        `<mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)>::visualize`,
                        TraitItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
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
                                    value: 13,
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
                        region_path: RegionPath::ItemDecl(
                            ItemPath::AssocItem(
                                AssocItemPath::TraitForTypeItem(
                                    TraitForTypeItemPath(
                                        `<mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)>::visualize`,
                                        TraitItemKind::MethodRitchie(
                                            RitchieItemKind::Fn,
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
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                0,
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
                path: TypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent(0)`),
                template_parameters: HirTemplateParameters(
                    [],
                ),
                self_ty: HirType::PathLeading(
                    HirTypePathLeading {
                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                        template_arguments: [],
                        always_copyable: false,
                    },
                ),
                hir_eager_expr_region: HirEagerExprRegion {
                    region_path: RegionPath::ItemDecl(
                        ItemPath::ImplBlock(
                            ImplBlockPath::TypeImplBlock(
                                TypeImplBlockPath(`mnist_classifier::connected_component::ConnectedComponent(0)`),
                            ),
                        ),
                    ),
                    expr_arena: Arena {
                        data: [],
                    },
                    stmt_arena: Arena {
                        data: [],
                    },
                    pattern_arena: Arena {
                        data: [],
                    },
                    comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                        arena: Arena {
                            data: [],
                        },
                    },
                    runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
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
                    path: TypeItemPath(
                        `mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`,
                        TypeItemKind::MemoizedField,
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::vec::Vec`, `Extern`),
                            template_arguments: [
                                HirTemplateArgument::Type(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                        region_path: RegionPath::ItemDecl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(
                                        `mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`,
                                        TypeItemKind::MemoizedField,
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
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                0,
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
                    path: TypeItemPath(
                        `mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`,
                        TypeItemKind::MemoizedField,
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(
                                        `mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`,
                                        TypeItemKind::MemoizedField,
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
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                0,
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
                    path: TypeItemPath(
                        `mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`,
                        TypeItemKind::MemoizedField,
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(
                                        `mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`,
                                        TypeItemKind::MemoizedField,
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
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                0,
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
                    path: TypeItemPath(
                        `mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`,
                        TypeItemKind::MemoizedField,
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(
                                        `mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`,
                                        TypeItemKind::MemoizedField,
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
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                0,
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
                    path: TypeItemPath(
                        `mnist_classifier::connected_component::ConnectedComponent(0)::row_span_sum`,
                        TypeItemKind::MemoizedField,
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(
                                        `mnist_classifier::connected_component::ConnectedComponent(0)::row_span_sum`,
                                        TypeItemKind::MemoizedField,
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
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                0,
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
                    path: TypeItemPath(
                        `mnist_classifier::connected_component::ConnectedComponent(0)::distribution`,
                        TypeItemKind::MemoizedField,
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(
                                        `mnist_classifier::connected_component::ConnectedComponent(0)::distribution`,
                                        TypeItemKind::MemoizedField,
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
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                0,
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
                    path: TypeItemPath(
                        `mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`,
                        TypeItemKind::MemoizedField,
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(
                                        `mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`,
                                        TypeItemKind::MemoizedField,
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
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                0,
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
                    path: TypeItemPath(
                        `mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`,
                        TypeItemKind::MemoizedField,
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(
                                        `mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`,
                                        TypeItemKind::MemoizedField,
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
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                0,
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
                TypeMethodRitchieHirDecl {
                    path: TypeItemPath(
                        `mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
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
                                    value: 13,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
                                pattern_idx: 0,
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
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(
                                        `mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`,
                                        TypeItemKind::MethodRitchie(
                                            RitchieItemKind::Fn,
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
                        pattern_arena: Arena {
                            data: [
                                HirEagerPatternEntry {
                                    data: HirEagerPatternData::Ident {
                                        symbol_modifier: None,
                                        ident: `k`,
                                    },
                                    contract: Pure,
                                },
                            ],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
                                    },
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::Ident(
                                            `k`,
                                        ),
                                        data: HirEagerRuntimeVariableData::ParenateParameter,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                0,
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
                TypeMethodRitchieHirDecl {
                    path: TypeItemPath(
                        `mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_right_mass_sum`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
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
                                    value: 13,
                                },
                            ),
                        ),
                    },
                    parenate_parameters: HirEagerParenateParameters(
                        [
                            HirEagerParenateParameter::Simple {
                                pattern_idx: 0,
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
                            ty_path: TypePath(`core::num::f32`, `Extern`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath::AssocItem(
                                AssocItemPath::TypeItem(
                                    TypeItemPath(
                                        `mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_right_mass_sum`,
                                        TypeItemKind::MethodRitchie(
                                            RitchieItemKind::Fn,
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
                        pattern_arena: Arena {
                            data: [
                                HirEagerPatternEntry {
                                    data: HirEagerPatternData::Ident {
                                        symbol_modifier: None,
                                        ident: `k`,
                                    },
                                    contract: Pure,
                                },
                            ],
                        },
                        comptime_symbol_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_symbol_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::SelfValue,
                                        data: HirEagerRuntimeVariableData::SelfValue,
                                    },
                                    HirEagerRuntimeVariableEntry {
                                        name: HirEagerRuntimeVariableName::Ident(
                                            `k`,
                                        ),
                                        data: HirEagerRuntimeVariableData::ParenateParameter,
                                    },
                                ],
                            },
                            self_value_variable: Some(
                                0,
                            ),
                        },
                    },
                },
            ),
        ),
    ),
]
```