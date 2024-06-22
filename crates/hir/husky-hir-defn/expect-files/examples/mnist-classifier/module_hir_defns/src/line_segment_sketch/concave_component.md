```rust
[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                    hir_decl: PropsStructHirDecl {
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
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`),
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
                                                `line_segment_sketch`,
                                            ),
                                            data: HirEagerRuntimeVariableData::FieldVariable,
                                        },
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::Ident(
                                                `strokes`,
                                            ),
                                            data: HirEagerRuntimeVariableData::FieldVariable,
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
        MajorItemHirDefn::Form(
            MajorFormHirDefn::Ritchie(
                MajorFunctionRitchieHirDefn {
                    path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
                        path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
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
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 195,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                58,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 196,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::ImplBlock(
        ImplBlockHirDefn::TraitForType(
            TraitForTypeImplBlockHirDefn {
                hir_decl: TraitForTypeImplBlockHirDecl {
                    path: TraitForTypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)`),
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
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)`),
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
            },
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::MethodFn(
                TraitForTypeMethodRitchieHirDefn {
                    path: TraitForTypeItemPath(
                        `<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)>::visualize`,
                        TraitItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TraitForTypeMethodRitchieHirDecl {
                        path: TraitForTypeItemPath(
                            `<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)>::visualize`,
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
                                        value: 23,
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
                                ItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)>::visualize`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            3,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent as core::visual::Visualize(0)>::visualize`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 0,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `visualize`,
                                                path: AssocItemPath::TraitForTypeItem(
                                                    TraitForTypeItemPath(
                                                        `<#derive _ as core::visual::Visualize(0)>::visualize`,
                                                        TraitItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::SelfType,
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 2,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                    path: TypeImplBlockPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)`),
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
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)`),
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
            },
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`,
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
                                ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            2,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MemoizedField {
                                                self_argument: 0,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `hausdorff_norm`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`,
                                                        TypeItemKind::MemoizedField,
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 1,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Leashed {
                                                            place_idx: Some(
                                                                PlaceIdx(0),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`,
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
                                ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            6,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MemoizedField {
                                                self_argument: 0,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `norm`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`,
                                                        TypeItemKind::MemoizedField,
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 2,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 3,
                                                self_contract: Pure,
                                                ident: `norm`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::geom2d::Vector2d(0)::norm`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 1,
                                                opr: Closed(
                                                    Div,
                                                ),
                                                ropd: 4,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 5,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`,
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
                                ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            32,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::F32(
                                                    F32Literal {
                                                        value: OrderedFloat(
                                                            0.0,
                                                        ),
                                                        text: "0.0f32",
                                                    },
                                                ),
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 1,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 2,
                                                self_contract: Pure,
                                                ident: `first`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::slice::CyclicSlice(0)::first`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unwrap {
                                                opd: 3,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 4,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
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
                                                        always_copyable: true,
                                                    },
                                                ),
                                                ident: `start`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 6,
                                                self_contract: Pure,
                                                ident: `line_segment`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(3),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 8,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 9,
                                                self_contract: Pure,
                                                ident: `norm`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::geom2d::Vector2d(0)::norm`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 11,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 12,
                                                self_contract: Pure,
                                                ident: `start`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::slice::CyclicSlice(0)::start`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 14,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 15,
                                                self_contract: Pure,
                                                ident: `end`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::slice::CyclicSlice(0)::end`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 17,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(5),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner: 18,
                                                items: [
                                                    19,
                                                ],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 20,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `end`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(3),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(6),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 22,
                                                self_contract: Pure,
                                                ident: `dist_to_point`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::dist_to_point`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 19,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        23,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Leashed {
                                                                    place_idx: Some(
                                                                        PlaceIdx(6),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                7,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(7),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(1),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 25,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 26,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(1),
                                                        ),
                                                        BorrowMut,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                7,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(7),
                                                        ),
                                                        Move,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 28,
                                                opr: Assign,
                                                ropd: 29,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(1),
                                                        ),
                                                        Move,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    4..10,
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 30,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 4,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 21,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 5,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 24,
                                            coercion: None,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    opd: 27,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 0,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 0,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 1,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 5,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 2,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 7,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 3,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 10,
                                            coercion: None,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            13,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            16,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            stmts: ArenaIdxRange(
                                                1..4,
                                            ),
                                        },
                                        Return {
                                            result: 31,
                                            coercion: Trivial(
                                                TrivialHirEagerCoercion {
                                                    expectee_quary: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                },
                                            ),
                                        },
                                    ],
                                },
                                pattern_arena: Arena {
                                    data: [
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `hausdorff_norm`,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `curve_start`,
                                            },
                                            contract: Pure,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `curve_ls`,
                                            },
                                            contract: Pure,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `dp_norm`,
                                            },
                                            contract: Pure,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `point`,
                                            },
                                            contract: Pure,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `point_dist`,
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
                                                    `hausdorff_norm`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `curve_start`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `curve_ls`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `dp_norm`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LoopVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `point`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `point_dist`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        0,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`,
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
                                ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            29,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::F32(
                                                    F32Literal {
                                                        value: OrderedFloat(
                                                            0.0,
                                                        ),
                                                        text: "0.0f32",
                                                    },
                                                ),
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 1,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 3,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 4,
                                                self_contract: Pure,
                                                ident: `start`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::slice::CyclicSlice(0)::start`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner: 2,
                                                items: [
                                                    5,
                                                ],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 6,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 8,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 9,
                                                self_contract: Pure,
                                                ident: `start`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::slice::CyclicSlice(0)::start`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 11,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 12,
                                                self_contract: Pure,
                                                ident: `end`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::slice::CyclicSlice(0)::end`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 14,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(3),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner: 15,
                                                items: [
                                                    16,
                                                ],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 17,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(1),
                                                        ),
                                                        BorrowMut,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(2),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(4),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::Bool(
                                                    true,
                                                ),
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Compterm,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 20,
                                                self_contract: Pure,
                                                ident: `angle_to`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::geom2d::Vector2d(0)::angle_to`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 35,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        21,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: ImmutableOnStack {
                                                                    place: Idx(
                                                                        PlaceIdx(4),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 36,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        22,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Compterm,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 19,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                ropd: 23,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(2),
                                                        ),
                                                        BorrowMut,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(4),
                                                        ),
                                                        Move,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 25,
                                                opr: Assign,
                                                ropd: 26,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(1),
                                                        ),
                                                        Move,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    3..7,
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 2,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 18,
                                            coercion: None,
                                        },
                                        Eval {
                                            expr: 24,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr: 27,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 0,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 0,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 1,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 7,
                                            coercion: None,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            10,
                                                        ),
                                                        kind: LowerOpen,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            13,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            stmts: ArenaIdxRange(
                                                0..3,
                                            ),
                                        },
                                        Return {
                                            result: 28,
                                            coercion: Trivial(
                                                TrivialHirEagerCoercion {
                                                    expectee_quary: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                },
                                            ),
                                        },
                                    ],
                                },
                                pattern_arena: Arena {
                                    data: [
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `angle_change`,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `dp0`,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `dp`,
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
                                                    `angle_change`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `dp0`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LoopVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `dp`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        0,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`,
                            TypeItemKind::MemoizedField,
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            55,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 0,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `first`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::slice::CyclicSlice(0)::first`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unwrap {
                                                opd: 2,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 3,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
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
                                                        always_copyable: true,
                                                    },
                                                ),
                                                ident: `start`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 5,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 7,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 9,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 11,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(1),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 13,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 14,
                                                self_contract: Pure,
                                                ident: `start`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::slice::CyclicSlice(0)::start`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 16,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 17,
                                                self_contract: Pure,
                                                ident: `end`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::slice::CyclicSlice(0)::end`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 19,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(6),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(6),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner: 20,
                                                items: [
                                                    21,
                                                ],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 22,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `end`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(2),
                                                        ),
                                                        BorrowMut,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(2),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                7,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 26,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 25,
                                                self_contract: Pure,
                                                ident: `min`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::num::f32(0)::min`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        27,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Leashed {
                                                                    place_idx: Some(
                                                                        PlaceIdx(7),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 24,
                                                opr: Assign,
                                                ropd: 28,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(3),
                                                        ),
                                                        BorrowMut,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(3),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                7,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 32,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `x`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 31,
                                                self_contract: Pure,
                                                ident: `max`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::num::f32(0)::max`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        33,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Leashed {
                                                                    place_idx: Some(
                                                                        PlaceIdx(7),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 30,
                                                opr: Assign,
                                                ropd: 34,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(4),
                                                        ),
                                                        BorrowMut,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(4),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                7,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 38,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 37,
                                                self_contract: Pure,
                                                ident: `min`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::num::f32(0)::min`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        39,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Leashed {
                                                                    place_idx: Some(
                                                                        PlaceIdx(7),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 36,
                                                opr: Assign,
                                                ropd: 40,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(5),
                                                        ),
                                                        BorrowMut,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(5),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                7,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 44,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `y`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(7),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 43,
                                                self_contract: Pure,
                                                ident: `max`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::num::f32(0)::max`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        45,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Leashed {
                                                                    place_idx: Some(
                                                                        PlaceIdx(7),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 42,
                                                opr: Assign,
                                                ropd: 46,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(2),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(2),
                                                        ),
                                                        Move,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(3),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(3),
                                                        ),
                                                        Move,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorCall {
                                                path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        48,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: MutableOnStack {
                                                                    place: Idx(
                                                                        PlaceIdx(2),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        49,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: MutableOnStack {
                                                                    place: Idx(
                                                                        PlaceIdx(3),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(4),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(4),
                                                        ),
                                                        Move,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(5),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(5),
                                                        ),
                                                        Move,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorCall {
                                                path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        51,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: MutableOnStack {
                                                                    place: Idx(
                                                                        PlaceIdx(4),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        52,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: MutableOnStack {
                                                                    place: Idx(
                                                                        PlaceIdx(5),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorCall {
                                                path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 41,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        50,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 41,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        53,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    5..12,
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 5,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 23,
                                            coercion: None,
                                        },
                                        Eval {
                                            expr: 29,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr: 35,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr: 41,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr: 47,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 0,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 4,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 1,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 6,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 2,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 8,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 3,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 10,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 4,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 12,
                                            coercion: None,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            15,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            18,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            stmts: ArenaIdxRange(
                                                0..5,
                                            ),
                                        },
                                        Return {
                                            result: 54,
                                            coercion: Trivial(
                                                TrivialHirEagerCoercion {
                                                    expectee_quary: Transient,
                                                },
                                            ),
                                        },
                                    ],
                                },
                                pattern_arena: Arena {
                                    data: [
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `start_point`,
                                            },
                                            contract: Pure,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `xmin`,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `xmax`,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `ymin`,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `ymax`,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `point`,
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
                                                    `start_point`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `xmin`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `xmax`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `ymin`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `ymax`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LoopVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `point`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                        ],
                                    },
                                    self_value_variable: Some(
                                        0,
                                    ),
                                },
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`,
                            TypeItemKind::MemoizedField,
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            6,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 0,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `line_segment_sketch`,
                                                field_ty: HirType::PathLeading(
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
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MemoizedField {
                                                self_argument: 1,
                                                self_ty: HirType::PathLeading(
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
                                                ident: `bounding_box`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`,
                                                        TypeItemKind::MemoizedField,
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MemoizedField {
                                                self_argument: 3,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `bounding_box`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`,
                                                        TypeItemKind::MemoizedField,
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 2,
                                                self_contract: Pure,
                                                ident: `relative_bounding_box`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::geom2d::BoundingBox(0)::relative_bounding_box`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 42,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        4,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Leashed {
                                                                    place_idx: Some(
                                                                        PlaceIdx(0),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 5,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodRitchieHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TypeMethodRitchieHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`,
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
                                        value: 23,
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
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            13,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 0,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `first`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::slice::CyclicSlice(0)::first`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unwrap {
                                                opd: 2,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 3,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
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
                                                        always_copyable: true,
                                                    },
                                                ),
                                                ident: `start`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 4,
                                                self_contract: Pure,
                                                ident: `clone`,
                                                path: AssocItemPath::TraitForTypeItem(
                                                    TraitForTypeItemPath(
                                                        `<#derive _ as core::clone::Clone(0)>::clone`,
                                                        TraitItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::SelfType,
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 6,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 7,
                                                self_contract: Pure,
                                                ident: `last`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::slice::CyclicSlice(0)::last`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unwrap {
                                                opd: 8,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 9,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
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
                                                        always_copyable: true,
                                                    },
                                                ),
                                                ident: `end`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 10,
                                                self_contract: Pure,
                                                ident: `clone`,
                                                path: AssocItemPath::TraitForTypeItem(
                                                    TraitForTypeItemPath(
                                                        `<#derive _ as core::clone::Clone(0)>::clone`,
                                                        TraitItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::SelfType,
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorCall {
                                                path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 19,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        5,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 19,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        11,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 12,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodRitchieHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TypeMethodRitchieHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`,
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
                                        value: 23,
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
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            6,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 0,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `first`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::slice::CyclicSlice(0)::first`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unwrap {
                                                opd: 2,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 3,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
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
                                                        always_copyable: true,
                                                    },
                                                ),
                                                ident: `start`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 4,
                                                self_contract: Pure,
                                                ident: `clone`,
                                                path: AssocItemPath::TraitForTypeItem(
                                                    TraitForTypeItemPath(
                                                        `<#derive _ as core::clone::Clone(0)>::clone`,
                                                        TraitItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::SelfType,
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 5,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodRitchieHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TypeMethodRitchieHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`,
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
                                        value: 23,
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
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            6,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 0,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `last`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::slice::CyclicSlice(0)::last`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unwrap {
                                                opd: 2,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 3,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
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
                                                        always_copyable: true,
                                                    },
                                                ),
                                                ident: `end`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 4,
                                                self_contract: Pure,
                                                ident: `clone`,
                                                path: AssocItemPath::TraitForTypeItem(
                                                    TraitForTypeItemPath(
                                                        `<#derive _ as core::clone::Clone(0)>::clone`,
                                                        TraitItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::SelfType,
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 5,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodRitchieHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TypeMethodRitchieHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`,
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
                                        value: 23,
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
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            3,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 0,
                                                self_contract: Pure,
                                                ident: `line_segment`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 2,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodRitchieHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TypeMethodRitchieHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`,
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
                                        value: 23,
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
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            5,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 0,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `first`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::slice::CyclicSlice(0)::first`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unwrap {
                                                opd: 2,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 3,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 4,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodRitchieHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TypeMethodRitchieHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`,
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
                                        value: 23,
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
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            5,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 0,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
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
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `last`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::slice::CyclicSlice(0)::last`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::Type {
                                                                    attrs: HirTemplateVariableAttrs {
                                                                        class: Mono,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
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
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Unwrap {
                                                opd: 2,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 3,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                arguments: [],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr: 4,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                    ],
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
                        ),
                    ),
                },
            ),
        ),
    ),
]
```