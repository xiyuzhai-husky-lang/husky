```rust
[
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: SubmoduleItemPath(`mnist_classifier::line_segment_sketch::concave_component),
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: SubmoduleItemPath(`mnist_classifier::line_segment_sketch::convex_component),
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: SubmoduleItemPath(`mnist_classifier::line_segment_sketch::convexity),
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: SubmoduleItemPath(`mnist_classifier::line_segment_sketch::line_segment),
            },
        },
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                    hir_decl: PropsStructHirDecl {
                        path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        fields: [
                            PropsStructFieldHirDecl {
                                ident: `points`,
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
                                            ),
                                        ],
                                        always_copyable: true,
                                    },
                                ),
                                initialization: None,
                            },
                            PropsStructFieldHirDecl {
                                ident: `start`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
                                ),
                                initialization: Some(
                                    Bind {
                                        value: 3,
                                    },
                                ),
                            },
                            PropsStructFieldHirDecl {
                                ident: `end`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
                                ),
                                initialization: Some(
                                    Bind {
                                        value: 7,
                                    },
                                ),
                            },
                        ],
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                            ),
                            self_value_ty: None,
                            expr_arena: Arena {
                                data: [
                                    HirEagerExprEntry {
                                        data: HirEagerExprData::RuntimeVariable(
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
                                        coersion: None,
                                    },
                                    HirEagerExprEntry {
                                        data: HirEagerExprData::MethodRitchieCall {
                                            self_argument: 0,
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
                                            indirections: HirIndirections {
                                                initial_place: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                                indirections: [
                                                    HirIndirection::Deleash,
                                                ],
                                                final_place: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            instantiation: HirInstantiation {
                                                path: ItemPath(`core::slice::CyclicSlice(0)::first`),
                                                context: HirTypeContext {
                                                    comptime_var_overrides: [],
                                                },
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
                                        coersion: None,
                                    },
                                    HirEagerExprEntry {
                                        data: HirEagerExprData::Unwrap {
                                            opd: 1,
                                        },
                                        contracted_quary: HirContractedQuary {
                                            contract: None,
                                            quary: Transient,
                                        },
                                        is_always_copyable: true,
                                        place_contract_site: HirPlaceContractSite {
                                            place_contracts: [],
                                        },
                                        coersion: None,
                                    },
                                    HirEagerExprEntry {
                                        data: HirEagerExprData::MethodRitchieCall {
                                            self_argument: 2,
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
                                            indirections: HirIndirections {
                                                initial_place: Transient,
                                                indirections: [
                                                    HirIndirection::Deleash,
                                                ],
                                                final_place: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            instantiation: HirInstantiation {
                                                path: ItemPath(`#derive _ as core::clone::Clone(0)`),
                                                context: HirTypeContext {
                                                    comptime_var_overrides: [],
                                                },
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
                                        coersion: Some(
                                            Trivial(
                                                TrivialHirEagerCoercion {
                                                    expectee_quary: Transient,
                                                },
                                            ),
                                        ),
                                    },
                                    HirEagerExprEntry {
                                        data: HirEagerExprData::RuntimeVariable(
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
                                        coersion: None,
                                    },
                                    HirEagerExprEntry {
                                        data: HirEagerExprData::MethodRitchieCall {
                                            self_argument: 4,
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
                                            indirections: HirIndirections {
                                                initial_place: StackPure {
                                                    place: Idx(
                                                        PlaceIdx(0),
                                                    ),
                                                },
                                                indirections: [
                                                    HirIndirection::Deleash,
                                                ],
                                                final_place: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            instantiation: HirInstantiation {
                                                path: ItemPath(`core::slice::CyclicSlice(0)::last`),
                                                context: HirTypeContext {
                                                    comptime_var_overrides: [],
                                                },
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
                                        coersion: None,
                                    },
                                    HirEagerExprEntry {
                                        data: HirEagerExprData::Unwrap {
                                            opd: 5,
                                        },
                                        contracted_quary: HirContractedQuary {
                                            contract: None,
                                            quary: Transient,
                                        },
                                        is_always_copyable: true,
                                        place_contract_site: HirPlaceContractSite {
                                            place_contracts: [],
                                        },
                                        coersion: None,
                                    },
                                    HirEagerExprEntry {
                                        data: HirEagerExprData::MethodRitchieCall {
                                            self_argument: 6,
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
                                            indirections: HirIndirections {
                                                initial_place: Transient,
                                                indirections: [
                                                    HirIndirection::Deleash,
                                                ],
                                                final_place: Leashed {
                                                    place_idx: None,
                                                },
                                            },
                                            instantiation: HirInstantiation {
                                                path: ItemPath(`#derive _ as core::clone::Clone(0)`),
                                                context: HirTypeContext {
                                                    comptime_var_overrides: [],
                                                },
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
                                        coersion: Some(
                                            Trivial(
                                                TrivialHirEagerCoercion {
                                                    expectee_quary: Transient,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_arena: Arena {
                                data: [],
                            },
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::Ident(
                                                `points`,
                                            ),
                                            data: HirEagerRuntimeVariableData::FieldVariable,
                                        },
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::Ident(
                                                `start`,
                                            ),
                                            data: HirEagerRuntimeVariableData::FieldVariable,
                                        },
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::Ident(
                                                `end`,
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
                    path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                    hir_decl: PropsStructHirDecl {
                        path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        fields: [
                            PropsStructFieldHirDecl {
                                ident: `contour`,
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
                                initialization: None,
                            },
                            PropsStructFieldHirDecl {
                                ident: `strokes`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
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
                                initialization: None,
                            },
                        ],
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                            ),
                            self_value_ty: None,
                            expr_arena: Arena {
                                data: [],
                            },
                            stmt_arena: Arena {
                                data: [],
                            },
                            pattern_arena: Arena {
                                data: [],
                            },
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::Ident(
                                                `contour`,
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
                    path: MajorFormPath(`mnist_classifier::line_segment_sketch::go_right`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
                        path: MajorFormPath(`mnist_classifier::line_segment_sketch::go_right`, `Ritchie(
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
                                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                template_arguments: [],
                                                always_copyable: false,
                                            },
                                        ),
                                    },
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 1,
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
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 239,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                49,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 240,
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
                    path: MajorFormPath(`mnist_classifier::line_segment_sketch::go_left`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
                        path: MajorFormPath(`mnist_classifier::line_segment_sketch::go_left`, `Ritchie(
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
                                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                template_arguments: [],
                                                always_copyable: false,
                                            },
                                        ),
                                    },
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 1,
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
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 241,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                49,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 242,
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
                    path: MajorFormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
                        path: MajorFormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Ritchie(
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
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 1,
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
                                        pattern_idx: 2,
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
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 243,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                110,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 244,
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
                    path: MajorFormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
                        path: MajorFormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Ritchie(
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
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 1,
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
                                        pattern_idx: 2,
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
                                        pattern_idx: 3,
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
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 245,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                119,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 246,
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
                    path: MajorFormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: MajorFunctionRitchieHirDecl {
                        path: MajorFormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Ritchie(
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
                                    HirEagerParenateParameter::Simple {
                                        pattern_idx: 1,
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
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::vec::Vec`, `Extern`),
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
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 247,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                184,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 248,
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
                    path: TraitForTypeImplBlockPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke as core::visual::Visualize(0)`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    trai: HirTrait {
                        trai_path: TraitPath(`core::visual::Visualize`),
                        template_arguments: [],
                    },
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke as core::visual::Visualize(0)`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
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
                        `<mnist_classifier::line_segment_sketch::LineSegmentStroke as core::visual::Visualize(0)>::visualize`,
                        TraitItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TraitForTypeMethodRitchieHirDecl {
                        path: TraitForTypeItemPath(
                            `<mnist_classifier::line_segment_sketch::LineSegmentStroke as core::visual::Visualize(0)>::visualize`,
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
                                        value: 29,
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
                                ItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke as core::visual::Visualize(0)>::visualize`),
                            ),
                            self_value_ty: Some(
                                HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
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
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
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
                                    ItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentStroke as core::visual::Visualize(0)>::visualize`),
                                ),
                                self_value_ty: Some(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                            template_arguments: [],
                                            always_copyable: false,
                                        },
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 0,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
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
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                },
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 2,
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
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                },
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::EmptyHtmlTag {
                                                function_ident: `LineSegment`,
                                                arguments: [
                                                    HirEagerHtmlArgumentExpr {
                                                        property_ident: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 122,
                                                                },
                                                            ),
                                                        ),
                                                        expr: 1,
                                                    },
                                                    HirEagerHtmlArgumentExpr {
                                                        property_ident: Ident(
                                                            Coword(
                                                                Id {
                                                                    value: 123,
                                                                },
                                                            ),
                                                        ),
                                                        expr: 3,
                                                    },
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
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
                                comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
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
                    path: TypeImplBlockPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
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
            TypeItemHirDefn::AssocRitchie(
                TypeAssocRitchieHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`,
                        TypeItemKind::AssocRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TypeAssocRitchieHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`,
                            TypeItemKind::AssocRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
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
                                HirEagerParenateParameter::Simple {
                                    pattern_idx: 1,
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
                                    pattern_idx: 2,
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
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`),
                            ),
                            self_value_ty: None,
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
                                            ident: `ct`,
                                        },
                                        contract: Pure,
                                    },
                                    HirEagerPatternEntry {
                                        data: HirEagerPatternData::Ident {
                                            symbol_modifier: None,
                                            ident: `from`,
                                        },
                                        contract: Pure,
                                    },
                                    HirEagerPatternEntry {
                                        data: HirEagerPatternData::Ident {
                                            symbol_modifier: None,
                                            ident: `to`,
                                        },
                                        contract: Pure,
                                    },
                                ],
                            },
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::SelfValue,
                                            data: HirEagerRuntimeVariableData::SelfValue,
                                        },
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::Ident(
                                                `ct`,
                                            ),
                                            data: HirEagerRuntimeVariableData::ParenateParameter,
                                        },
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::Ident(
                                                `from`,
                                            ),
                                            data: HirEagerRuntimeVariableData::ParenateParameter,
                                        },
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::Ident(
                                                `to`,
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
                            11,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::new`),
                                ),
                                self_value_ty: None,
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                2,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: StackPure {
                                                            place: Idx(
                                                                PlaceIdx(2),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 0,
                                                opr: Comparison(
                                                    Leq,
                                                ),
                                                ropd: 1,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Leash,
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
                                                        Leash,
                                                    ),
                                                ],
                                            },
                                            coersion: None,
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
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: StackPure {
                                                            place: Idx(
                                                                PlaceIdx(1),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                2,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: StackPure {
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
                                            coersion: None,
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Compterm,
                                                    },
                                                ),
                                            ),
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
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 4,
                                                self_contract: Leash,
                                                ident: `cyclic_slice_leashed`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `core::vec::Vec(0)::cyclic_slice_leashed`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::vec::Vec(0)::cyclic_slice_leashed`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
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
                                                        5,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: StackPure {
                                                                    place: Idx(
                                                                        PlaceIdx(1),
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
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        8,
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorCall {
                                                path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
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
                                                                        value: 50,
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
                                            is_always_copyable: false,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..2,
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Assert {
                                            condition: Other {
                                                opd: 2,
                                                conversion: None,
                                            },
                                        },
                                        Eval {
                                            expr: 10,
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
                                comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `ct`,
                                                ),
                                                data: HirEagerRuntimeVariableData::ParenateParameter,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `from`,
                                                ),
                                                data: HirEagerRuntimeVariableData::ParenateParameter,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `to`,
                                                ),
                                                data: HirEagerRuntimeVariableData::ParenateParameter,
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
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodRitchieHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
                        TypeItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TypeMethodRitchieHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`,
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
                                        value: 29,
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
                                ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`),
                            ),
                            self_value_ty: Some(
                                HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
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
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
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
                                    ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`),
                                ),
                                self_value_ty: Some(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                            template_arguments: [],
                                            always_copyable: false,
                                        },
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 0,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
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
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                },
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 2,
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
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                },
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: StackPure {
                                                            place: Idx(
                                                                PlaceIdx(0),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `to`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(
                                                        `mnist_classifier::geom2d::Point2d(0)::to`,
                                                        TypeItemKind::MethodRitchie(
                                                            RitchieItemKind::Fn,
                                                        ),
                                                    ),
                                                ),
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::geom2d::Point2d(0)::to`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
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
                                                        3,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: StackPure {
                                                                    place: Idx(
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
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
                                comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
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
        ImplBlockHirDefn::TraitForType(
            TraitForTypeImplBlockHirDefn {
                hir_decl: TraitForTypeImplBlockHirDecl {
                    path: TraitForTypeImplBlockPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch as core::visual::Visualize(0)`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    trai: HirTrait {
                        trai_path: TraitPath(`core::visual::Visualize`),
                        template_arguments: [],
                    },
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch as core::visual::Visualize(0)`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
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
                        `<mnist_classifier::line_segment_sketch::LineSegmentSketch as core::visual::Visualize(0)>::visualize`,
                        TraitItemKind::MethodRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TraitForTypeMethodRitchieHirDecl {
                        path: TraitForTypeItemPath(
                            `<mnist_classifier::line_segment_sketch::LineSegmentSketch as core::visual::Visualize(0)>::visualize`,
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
                                        value: 40,
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
                                ItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentSketch as core::visual::Visualize(0)>::visualize`),
                            ),
                            self_value_ty: Some(
                                HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
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
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
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
                                    ItemPath(`<mnist_classifier::line_segment_sketch::LineSegmentSketch as core::visual::Visualize(0)>::visualize`),
                                ),
                                self_value_ty: Some(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                            template_arguments: [],
                                            always_copyable: false,
                                        },
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 0,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
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
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                },
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
                                            coersion: None,
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
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`#derive _ as core::visual::Visualize(0)`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVariable::Type(
                                                                HirTypeTemplateVariable::SelfType,
                                                            ),
                                                            HirTermSymbolicVariableResolution::Explicit(
                                                                HirTemplateArgument::Type(
                                                                    HirType::PathLeading(
                                                                        HirTypePathLeading {
                                                                            ty_path: TypePath(`core::vec::Vec`, `Extern`),
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
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
                                comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
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
                    path: TypeImplBlockPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
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
                        `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::concave_components`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::concave_components`,
                            TypeItemKind::MemoizedField,
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
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::concave_components`),
                            ),
                            self_value_ty: Some(
                                HirType::PathLeading(
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
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
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
                                    ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::concave_components`),
                                ),
                                self_value_ty: Some(
                                    HirType::PathLeading(
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
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: StackPure {
                                                            place: Idx(
                                                                PlaceIdx(0),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::FunctionRitchieCall {
                                                path: MajorFormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Ritchie(
                                                    Fn,
                                                )`),
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
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
                                                                        value: 41,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        0,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: StackPure {
                                                                    place: Idx(
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
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
                                comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
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
                        `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`,
                        TypeItemKind::MemoizedField,
                    ),
                    hir_decl: TypeMemoFieldHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`,
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
                                ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`),
                            ),
                            self_value_ty: Some(
                                HirType::PathLeading(
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
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
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
                            52,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::bounding_box`),
                                ),
                                self_value_ty: Some(
                                    HirType::PathLeading(
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
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 0,
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
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
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
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::USize(
                                                    USizeLiteral {
                                                        value: 0,
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner: 1,
                                                items: [
                                                    2,
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 3,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
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
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
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
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                },
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
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
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                },
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
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
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                },
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
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
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(1),
                                                        ),
                                                    },
                                                },
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 13,
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
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
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
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 14,
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
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::vec::Vec(0)::ilen`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 16,
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
                                                ident: `strokes`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
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
                                                indirections: HirIndirections {
                                                    initial_place: StackPure {
                                                        place: Idx(
                                                            PlaceIdx(0),
                                                        ),
                                                    },
                                                    indirections: [
                                                        HirIndirection::Deleash,
                                                    ],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner: 17,
                                                items: [
                                                    18,
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 19,
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
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: None,
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: None,
                                                    },
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 23,
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
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(7),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(7),
                                                        ),
                                                    },
                                                },
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
                                            coersion: Some(
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
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 22,
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
                                                indirections: HirIndirections {
                                                    initial_place: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(2),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(2),
                                                        ),
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::num::f32(0)::min`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
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
                                                        24,
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 21,
                                                opr: Assign,
                                                ropd: 25,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 29,
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
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(7),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(7),
                                                        ),
                                                    },
                                                },
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
                                            coersion: Some(
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
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 28,
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
                                                indirections: HirIndirections {
                                                    initial_place: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(3),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(3),
                                                        ),
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::num::f32(0)::max`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
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
                                                        30,
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 27,
                                                opr: Assign,
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 35,
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
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(7),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(7),
                                                        ),
                                                    },
                                                },
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
                                            coersion: Some(
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
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 34,
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
                                                indirections: HirIndirections {
                                                    initial_place: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(4),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(4),
                                                        ),
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::num::f32(0)::min`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
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
                                                        36,
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 33,
                                                opr: Assign,
                                                ropd: 37,
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: None,
                                                quary: Transient,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: None,
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 41,
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
                                                indirections: HirIndirections {
                                                    initial_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(7),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: Leashed {
                                                        place_idx: Some(
                                                            PlaceIdx(7),
                                                        ),
                                                    },
                                                },
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
                                            coersion: Some(
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
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodRitchieCall {
                                                self_argument: 40,
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
                                                indirections: HirIndirections {
                                                    initial_place: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(5),
                                                        ),
                                                    },
                                                    indirections: [],
                                                    final_place: MutableOnStack {
                                                        place: Idx(
                                                            PlaceIdx(5),
                                                        ),
                                                    },
                                                },
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`core::num::f32(0)::max`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
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
                                                        42,
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 39,
                                                opr: Assign,
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: Some(
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
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: Some(
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
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorCall {
                                                path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::geom2d::ClosedRange`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
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
                                                        45,
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
                                                        46,
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: Some(
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
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: Some(
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
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorCall {
                                                path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::geom2d::ClosedRange`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
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
                                                        49,
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorCall {
                                                path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::geom2d::BoundingBox`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
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
                                                                        value: 43,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        47,
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
                                                                        value: 43,
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
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
                                            coersion: Some(
                                                Never,
                                            ),
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
                                            initial_value: 20,
                                            coercion: None,
                                        },
                                        Eval {
                                            expr: 26,
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
                                            expr: 38,
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
                                                        bound_expr: None,
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            15,
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
                                            result: 51,
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
                                comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
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
            TypeItemHirDefn::AssocRitchie(
                TypeAssocRitchieHirDefn {
                    path: TypeItemPath(
                        `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`,
                        TypeItemKind::AssocRitchie(
                            RitchieItemKind::Fn,
                        ),
                    ),
                    hir_decl: TypeAssocRitchieHirDecl {
                        path: TypeItemPath(
                            `mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`,
                            TypeItemKind::AssocRitchie(
                                RitchieItemKind::Fn,
                            ),
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
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
                                HirEagerParenateParameter::Simple {
                                    pattern_idx: 1,
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
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`),
                            ),
                            self_value_ty: None,
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
                                            ident: `ct`,
                                        },
                                        contract: Pure,
                                    },
                                    HirEagerPatternEntry {
                                        data: HirEagerPatternData::Ident {
                                            symbol_modifier: None,
                                            ident: `r`,
                                        },
                                        contract: Pure,
                                    },
                                ],
                            },
                            comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::SelfValue,
                                            data: HirEagerRuntimeVariableData::SelfValue,
                                        },
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::Ident(
                                                `ct`,
                                            ),
                                            data: HirEagerRuntimeVariableData::ParenateParameter,
                                        },
                                        HirEagerRuntimeVariableEntry {
                                            name: HirEagerRuntimeVariableName::Ident(
                                                `r`,
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
                            5,
                            HirEagerExprRegion {
                                region_path: RegionPath::ItemDefn(
                                    ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch(0)::new`),
                                ),
                                self_value_ty: None,
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
                                                0,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
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
                                                        Move,
                                                    ),
                                                ],
                                            },
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: StackPure {
                                                            place: Idx(
                                                                PlaceIdx(0),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: StackPure {
                                                            place: Idx(
                                                                PlaceIdx(0),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::RuntimeVariable(
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: StackPure {
                                                            place: Idx(
                                                                PlaceIdx(1),
                                                            ),
                                                        },
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::FunctionRitchieCall {
                                                path: MajorFormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Ritchie(
                                                    Fn,
                                                )`),
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::line_segment_sketch::find_line_segments`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
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
                                                                        value: 6,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        1,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: StackPure {
                                                                    place: Idx(
                                                                        PlaceIdx(0),
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
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        2,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: StackPure {
                                                                    place: Idx(
                                                                        PlaceIdx(1),
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::TypeConstructorCall {
                                                path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                instantiation: HirInstantiation {
                                                    path: ItemPath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`),
                                                    context: HirTypeContext {
                                                        comptime_var_overrides: [],
                                                    },
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
                                                                        value: 6,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        0,
                                                        Trivial(
                                                            TrivialHirEagerCoercion {
                                                                expectee_quary: StackPure {
                                                                    place: Idx(
                                                                        PlaceIdx(0),
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
                                                                        value: 46,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        3,
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
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
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoercion {
                                                        expectee_quary: Transient,
                                                    },
                                                ),
                                            ),
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
                                comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `ct`,
                                                ),
                                                data: HirEagerRuntimeVariableData::ParenateParameter,
                                            },
                                            HirEagerRuntimeVariableEntry {
                                                name: HirEagerRuntimeVariableName::Ident(
                                                    `r`,
                                                ),
                                                data: HirEagerRuntimeVariableData::ParenateParameter,
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
]
```