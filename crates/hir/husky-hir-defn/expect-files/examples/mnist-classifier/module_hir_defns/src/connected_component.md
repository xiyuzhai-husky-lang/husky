```rust
[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                    hir_decl: PropsStructHirDecl {
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
                                ItemPath(`mnist_classifier::connected_component::ConnectedComponentDistribution`),
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
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                    hir_decl: PropsStructHirDecl {
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
                                ItemPath(`mnist_classifier::connected_component::EffHoles`),
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
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Form(
            MajorFormHirDefn::Ritchie(
                MajorFunctionRitchieHirDefn {
                    path: FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
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
                                    value: 5,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                8,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 6,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                    hir_decl: PropsStructHirDecl {
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
                                ItemPath(`mnist_classifier::connected_component::ConnectedComponent`),
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
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Form(
            MajorFormHirDefn::Ritchie(
                MajorFunctionRitchieHirDefn {
                    path: FormPath(`mnist_classifier::connected_component::horizontal_extend`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
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
                                    value: 8,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                42,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 9,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Form(
            MajorFormHirDefn::Ritchie(
                MajorFunctionRitchieHirDefn {
                    path: FormPath(`mnist_classifier::connected_component::find_connected_components`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
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
                                    value: 10,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                104,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 11,
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
                            ItemPath(`mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)`),
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
                        `<mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)>::visualize`,
                        TraitItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TraitForTypeMethodRitchieHirDecl {
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
                                ItemPath(`<mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)>::visualize`),
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
                                    ItemPath(`<mnist_classifier::connected_component::ConnectedComponent as core::visual::Visualize(0)>::visualize`),
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
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
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
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `visualize`,
                                                path: AssocItemPath::TraitForTypeItem(
                                                    TraitForTypeItemPath(
                                                        `<mnist::BinaryImage28 as core::visual::Visualize(0)>::visualize`,
                                                        TraitItemKind::MethodRitchie(
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
    HirDefn::ImplBlock(
        ImplBlockHirDefn::Type(
            TypeImplBlockHirDefn {
                hir_decl: TypeImplBlockHirDecl {
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
                            ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)`),
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
                        `mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
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
                                ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`),
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
                                    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`),
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
                                            data: HirEagerExprData::FunctionRitchieCall {
                                                path: FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Ritchie(
                                                    Fn,
                                                )`),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 17,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        0,
                                                        PlaceToLeash,
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
                                            expr: 1,
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
                        `mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
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
                                ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`),
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
                            19,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::eff_holes`),
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
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `raw_contours`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`,
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
                                                self_argument: 1,
                                                self_contract: Leash,
                                                ident: `collect_leashes`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::vec::Vec(0)::collect_leashes`,
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
                                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                            data: HirEagerExprData::NewList {
                                                exprs: [],
                                                element_ty: HirType::PathLeading(
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
                                            is_always_copyable: false,
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
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Ritchie(
                                                            Fn,
                                                        )`),
                                                    ),
                                                ),
                                            ),
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
                                                self_argument: 4,
                                                self_contract: BorrowMut,
                                                ident: `pop_with_largest_opt_f32`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::vec::Vec(0)::pop_with_largest_opt_f32`,
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
                                                            ),
                                                        ),
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::SelfLifetime,
                                                            ),
                                                            HirTermSymbolicVariableResolution::SelfLifetime,
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: Ritchie(
                                                                HirRitchieType(
                                                                    Id {
                                                                        value: 1,
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
                                            is_always_copyable: false,
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
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Ritchie(
                                                            Fn,
                                                        )`),
                                                    ),
                                                ),
                                            ),
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
                                                self_argument: 8,
                                                self_contract: BorrowMut,
                                                ident: `pop_with_largest_opt_f32`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::vec::Vec(0)::pop_with_largest_opt_f32`,
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
                                                            ),
                                                        ),
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::SelfLifetime,
                                                            ),
                                                            HirTermSymbolicVariableResolution::SelfLifetime,
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: Ritchie(
                                                                HirRitchieType(
                                                                    Id {
                                                                        value: 1,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        9,
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
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 7,
                                                self_contract: BorrowMut,
                                                ident: `push`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::vec::Vec(0)::push`,
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
                                                            ),
                                                        ),
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::SelfLifetime,
                                                            ),
                                                            HirTermSymbolicVariableResolution::SelfLifetime,
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 7,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        10,
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
                                            is_always_copyable: false,
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
                                            data: HirEagerExprData::PrincipalEntityPath(
                                                PrincipalEntityPath::MajorItem(
                                                    MajorItemPath::Form(
                                                        FormPath(`mnist_classifier::connected_component::hole_tmpl`, `Ritchie(
                                                            Fn,
                                                        )`),
                                                    ),
                                                ),
                                            ),
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
                                                self_argument: 13,
                                                self_contract: BorrowMut,
                                                ident: `pop_with_largest_opt_f32`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::vec::Vec(0)::pop_with_largest_opt_f32`,
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
                                                            ),
                                                        ),
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::SelfLifetime,
                                                            ),
                                                            HirTermSymbolicVariableResolution::SelfLifetime,
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: Ritchie(
                                                                HirRitchieType(
                                                                    Id {
                                                                        value: 1,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        14,
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
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 12,
                                                self_contract: BorrowMut,
                                                ident: `push`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::vec::Vec(0)::push`,
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
                                                            ),
                                                        ),
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::SelfLifetime,
                                                            ),
                                                            HirTermSymbolicVariableResolution::SelfLifetime,
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                arguments: [
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 7,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        15,
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
                                            is_always_copyable: false,
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
                                            data: HirEagerExprData::TypeConstructorCall {
                                                path: TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
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
                                                                        value: 8,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        17,
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
                                                    0..6,
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
                                                pattern_idx: 0,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 2,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 1,
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 8,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            contract: Move,
                                            initial_value: 3,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        Eval {
                                            expr: 6,
                                            coercion: None,
                                            discarded: true,
                                        },
                                        Eval {
                                            expr: 11,
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
                                            expr: 16,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Return {
                                            result: 18,
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
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `raw_contours`,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `matches`,
                                            },
                                            contract: Move,
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
                                                    `raw_contours`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `matches`,
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
                        `mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
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
                                ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`),
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
                            19,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::max_hole_ilen`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
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
                                            data: HirEagerExprData::MemoizedField {
                                                self_argument: 1,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `raw_contours`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::connected_component::ConnectedComponent(0)::raw_contours`,
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
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
                                                2,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(2),
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
                                                self_argument: 4,
                                                self_contract: Pure,
                                                ident: `ilen`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::vec::Vec(0)::ilen`,
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
                                                                            ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
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
                                                2,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(2),
                                                    ),
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
                                                owner: 6,
                                                items: [
                                                    7,
                                                ],
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(2),
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
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `points`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        template_arguments: [
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                                        template_arguments: [],
                                                                        always_copyable: false,
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Leashed {
                                                    place_idx: Some(
                                                        PlaceIdx(2),
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
                                                self_argument: 9,
                                                self_contract: Pure,
                                                ident: `ilen`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::vec::Vec(0)::ilen`,
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
                                            data: HirEagerExprData::Binary {
                                                lopd: 11,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                ropd: 12,
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
                                            data: HirEagerExprData::Binary {
                                                lopd: 14,
                                                opr: Assign,
                                                ropd: 15,
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
                                            data: HirEagerExprData::As {
                                                opd: 17,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
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
                                        Eval {
                                            expr: 16,
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
                                                pattern_idx: 2,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 10,
                                            coercion: None,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    opd: 13,
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
                                            initial_value: 2,
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
                                                            3,
                                                        ),
                                                        kind: LowerOpen,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            5,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            stmts: ArenaIdxRange(
                                                1..3,
                                            ),
                                        },
                                        Return {
                                            result: 18,
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
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `max_hole_ilen`,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `raw_contours`,
                                            },
                                            contract: Pure,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `hole_ilen`,
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
                                                    `max_hole_ilen`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `raw_contours`,
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
                                                    `hole_ilen`,
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
                        `mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
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
                                ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`),
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
                            14,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::max_row_span`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    29,
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
                                                self_argument: 5,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
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
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
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
                                            data: HirEagerExprData::Index {
                                                owner: 6,
                                                items: [
                                                    7,
                                                ],
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
                                                self_argument: 8,
                                                self_contract: Pure,
                                                ident: `span`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::raw_bits::r32(0)::span`,
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
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 4,
                                                self_contract: Pure,
                                                ident: `max`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::num::i32(0)::max`,
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
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        9,
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
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 3,
                                                opr: Assign,
                                                ropd: 10,
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
                                            data: HirEagerExprData::As {
                                                opd: 12,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
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
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..4,
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
                                            expr: 11,
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
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 4,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            contract: Move,
                                            initial_value: 0,
                                            coercion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Compterm,
                                                    },
                                                ),
                                            ),
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
                                                            1,
                                                        ),
                                                        kind: LowerOpen,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            2,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            stmts: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                        Return {
                                            result: 13,
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
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `max_row`,
                                            },
                                            contract: Move,
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
                                                    `max_row`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LoopVariable,
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
                        `mnist_classifier::connected_component::ConnectedComponent(0)::row_span_sum`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
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
                                ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::row_span_sum`),
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
                            12,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::row_span_sum`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    29,
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
                                                self_argument: 4,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
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
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
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
                                            data: HirEagerExprData::Index {
                                                owner: 5,
                                                items: [
                                                    6,
                                                ],
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
                                                self_argument: 7,
                                                self_contract: Pure,
                                                ident: `span`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::raw_bits::r32(0)::span`,
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
                                                lopd: 3,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                ropd: 8,
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
                                            data: HirEagerExprData::As {
                                                opd: 10,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
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
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..4,
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
                                            expr: 9,
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
                                                            1,
                                                        ),
                                                        kind: LowerOpen,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            2,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            stmts: ArenaIdxRange(
                                                0..1,
                                            ),
                                        },
                                        Return {
                                            result: 11,
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
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `row_span_sum`,
                                            },
                                            contract: Move,
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
                                                    `row_span_sum`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LoopVariable,
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
                        `mnist_classifier::connected_component::ConnectedComponent(0)::distribution`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
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
                                ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::distribution`),
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
                            50,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::distribution`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    29,
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
                                                self_argument: 2,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
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
                                            data: HirEagerExprData::Index {
                                                owner: 3,
                                                items: [
                                                    4,
                                                ],
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
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
                                            data: HirEagerExprData::Binary {
                                                lopd: 6,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 7,
                                            },
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    29,
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
                                                self_argument: 10,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
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
                                            data: HirEagerExprData::Index {
                                                owner: 11,
                                                items: [
                                                    12,
                                                ],
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
                                            data: HirEagerExprData::Prefix {
                                                opr: NotInt,
                                                opd: 13,
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
                                                lopd: 15,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 16,
                                            },
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    2,
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
                                            data: HirEagerExprData::Binary {
                                                lopd: 18,
                                                opr: Closed(
                                                    Div,
                                                ),
                                                ropd: 19,
                                            },
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
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
                                            data: HirEagerExprData::Binary {
                                                lopd: 23,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 24,
                                            },
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
                                                self_argument: 27,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
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
                                            is_always_copyable: false,
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
                                                owner: 28,
                                                items: [
                                                    29,
                                                ],
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
                                                self_argument: 30,
                                                self_contract: Pure,
                                                ident: `co`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::raw_bits::r32(0)::co`,
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
                                                lopd: 26,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                ropd: 31,
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
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
                                            data: HirEagerExprData::Binary {
                                                lopd: 35,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 36,
                                            },
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
                                                contract: Some(
                                                    BorrowMut,
                                                ),
                                                quary: MutableOnStack {
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
                                                        BorrowMut,
                                                    ),
                                                ],
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
                                                self_argument: 39,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
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
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                8,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
                                                    place: Idx(
                                                        PlaceIdx(8),
                                                    ),
                                                },
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [
                                                    (
                                                        Idx(
                                                            PlaceIdx(8),
                                                        ),
                                                        Pure,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner: 40,
                                                items: [
                                                    41,
                                                ],
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
                                                self_argument: 42,
                                                self_contract: Pure,
                                                ident: `co`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::raw_bits::r32(0)::co`,
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
                                                lopd: 38,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                ropd: 43,
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
                                            data: HirEagerExprData::Variable(
                                                7,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: MutableOnStack {
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
                                            data: HirEagerExprData::TypeConstructorCall {
                                                path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
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
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        45,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: MutableOnStack {
                                                                    place: Idx(
                                                                        PlaceIdx(1),
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
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        46,
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
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        47,
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
                                                    Simple(
                                                        HirRitchieSimpleParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        48,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: MutableOnStack {
                                                                    place: Idx(
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
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    6..17,
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
                                        Break,
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    opd: 5,
                                                    conversion: IntToBool(
                                                        R32,
                                                    ),
                                                },
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Break,
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    opd: 14,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    2..3,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr: 32,
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
                                            expr: 44,
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
                                        Forext {
                                            particulars: HirEagerForExtParticulars {
                                                forext_loop_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 39,
                                                        },
                                                    ),
                                                ),
                                                bound_expr_hir_eager_expr_idx: 1,
                                                boundary_kind: UpperOpen,
                                            },
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 1,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 8,
                                            coercion: None,
                                        },
                                        Forext {
                                            particulars: HirEagerForExtParticulars {
                                                forext_loop_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 41,
                                                        },
                                                    ),
                                                ),
                                                bound_expr_hir_eager_expr_idx: 9,
                                                boundary_kind: UpperOpen,
                                            },
                                            stmts: ArenaIdxRange(
                                                3..4,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 2,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 17,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 3,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 20,
                                            coercion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 4,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 21,
                                            coercion: None,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 77,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            22,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            25,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            stmts: ArenaIdxRange(
                                                4..5,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 5,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 33,
                                            coercion: None,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 79,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            34,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            37,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    step: Constant(
                                                        -1,
                                                    ),
                                                },
                                            },
                                            stmts: ArenaIdxRange(
                                                5..6,
                                            ),
                                        },
                                        Return {
                                            result: 49,
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
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `row_start`,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `row_end`,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `height`,
                                            },
                                            contract: Pure,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `half_height`,
                                            },
                                            contract: Pure,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `upper_mass`,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `lower_mass`,
                                            },
                                            contract: Move,
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
                                                    `row_start`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `row_end`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `height`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `half_height`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `upper_mass`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `i1`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LoopVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `lower_mass`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `i2`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LoopVariable,
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
                        `mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
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
                                ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`),
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
                            4,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::upper_mass`),
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
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `distribution`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::connected_component::ConnectedComponent(0)::distribution`,
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
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 1,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `upper_mass`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                                        template_arguments: [],
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
                                            data: HirEagerExprData::As {
                                                opd: 2,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
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
                                            expr: 3,
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
                        `mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
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
                                ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`),
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
                            4,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::lower_mass`),
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
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `distribution`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::connected_component::ConnectedComponent(0)::distribution`,
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
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 1,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `lower_mass`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                                        template_arguments: [],
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
                                            data: HirEagerExprData::As {
                                                opd: 2,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
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
                                            expr: 3,
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
                        `mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TypeMethodRitchieHirDecl {
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
                                ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            23,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_span_sum`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
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
                                                1,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
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
                                            data: HirEagerExprData::Binary {
                                                lopd: 1,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 2,
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    29,
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
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
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
                                            data: HirEagerExprData::Variable(
                                                3,
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
                                            data: HirEagerExprData::Index {
                                                owner: 7,
                                                items: [
                                                    8,
                                                ],
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
                                            data: HirEagerExprData::Variable(
                                                3,
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
                                                3,
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
                                                1,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
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
                                                lopd: 11,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 12,
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
                                                self_argument: 15,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
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
                                            data: HirEagerExprData::Variable(
                                                4,
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
                                                owner: 16,
                                                items: [
                                                    17,
                                                ],
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
                                                self_argument: 18,
                                                self_contract: Pure,
                                                ident: `span`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::raw_bits::r32(0)::span`,
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
                                                lopd: 14,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                ropd: 19,
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
                                            data: HirEagerExprData::As {
                                                opd: 21,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
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
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    3..9,
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
                                        Break,
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    opd: 9,
                                                    conversion: IntToBool(
                                                        R32,
                                                    ),
                                                },
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr: 20,
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
                                        Assert {
                                            condition: Other {
                                                opd: 3,
                                                conversion: None,
                                            },
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 1,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 4,
                                            coercion: None,
                                        },
                                        Forext {
                                            particulars: HirEagerForExtParticulars {
                                                forext_loop_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                                bound_expr_hir_eager_expr_idx: 5,
                                                boundary_kind: UpperOpen,
                                            },
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 82,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            10,
                                                        ),
                                                        kind: LowerClosed,
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
                                                2..3,
                                            ),
                                        },
                                        Return {
                                            result: 22,
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
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `top_k_row_span_sum`,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `i`,
                                            },
                                            contract: Move,
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
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `top_k_row_span_sum`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `j`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LoopVariable,
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
                        `mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_right_mass_sum`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TypeMethodRitchieHirDecl {
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
                                ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_right_mass_sum`),
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            23,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::connected_component::ConnectedComponent(0)::top_k_row_right_mass_sum`),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
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
                                                1,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
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
                                            data: HirEagerExprData::Binary {
                                                lopd: 1,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 2,
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
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
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    29,
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
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
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
                                            data: HirEagerExprData::Variable(
                                                3,
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
                                            data: HirEagerExprData::Index {
                                                owner: 7,
                                                items: [
                                                    8,
                                                ],
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
                                            data: HirEagerExprData::Variable(
                                                3,
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
                                                3,
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
                                                1,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
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
                                                lopd: 11,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 12,
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
                                                self_argument: 15,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `mask`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist::BinaryImage28`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: false,
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
                                            data: HirEagerExprData::Variable(
                                                4,
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
                                                owner: 16,
                                                items: [
                                                    17,
                                                ],
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
                                                self_argument: 18,
                                                self_contract: Pure,
                                                ident: `right_mass`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::raw_bits::r32(0)::right_mass`,
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
                                                lopd: 14,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                ropd: 19,
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
                                            data: HirEagerExprData::As {
                                                opd: 21,
                                                ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        template_arguments: [],
                                                        always_copyable: true,
                                                    },
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
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    3..9,
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
                                        Break,
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    opd: 9,
                                                    conversion: IntToBool(
                                                        R32,
                                                    ),
                                                },
                                                stmts: ArenaIdxRange(
                                                    0..1,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr: 20,
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
                                        Assert {
                                            condition: Other {
                                                opd: 3,
                                                conversion: None,
                                            },
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_idx: 1,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 4,
                                            coercion: None,
                                        },
                                        Forext {
                                            particulars: HirEagerForExtParticulars {
                                                forext_loop_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                                bound_expr_hir_eager_expr_idx: 5,
                                                boundary_kind: UpperOpen,
                                            },
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 82,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            10,
                                                        ),
                                                        kind: LowerClosed,
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
                                                2..3,
                                            ),
                                        },
                                        Return {
                                            result: 22,
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
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `top_k_row_span_sum`,
                                            },
                                            contract: Move,
                                        },
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: Some(
                                                    Mut,
                                                ),
                                                ident: `i`,
                                            },
                                            contract: Move,
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
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `top_k_row_span_sum`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LetVariable,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `j`,
                                                ),
                                                data: HirEagerRuntimeVariableData::LoopVariable,
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