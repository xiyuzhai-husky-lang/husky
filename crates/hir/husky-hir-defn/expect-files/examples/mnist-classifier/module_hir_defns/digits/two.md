```rust
[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Ki(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::two::two_match`, `Val`),
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
                    hir_expr_body_and_region: Some(
                        (
                            Eager(
                                6,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 109,
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
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Ritchie(
                            Fn,
                        )`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Simple {
                                    pattern_expr_idx: 0,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
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
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
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
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Ritchie(
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
                            pattern_arena: Arena {
                                data: [
                                    HirEagerPatternEntry {
                                        data: HirEagerPatternData::Ident {
                                            symbol_modifier: None,
                                            ident: `cc`,
                                        },
                                        contract: Pure,
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
                                                `cc`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            8,
                            HirEagerExprRegion {
                                region_path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::two::left_cc_pattern`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
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
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 0,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::displacement`, `MethodRitchie(
                                                        Fn,
                                                    )`),
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
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
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
                                                self_argument: 2,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
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
                                                quary: Const,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 3,
                                                opr: Comparison(
                                                    Less,
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
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: ImmutableOnStack {
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
                                                        Move,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 6,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: ImmutableOnStack {
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
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..3,
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: ImmutableOnStack {
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
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 0,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 1,
                                            coersion: None,
                                        },
                                        Require {
                                            condition: Other {
                                                opd: 5,
                                                conversion: None,
                                            },
                                        },
                                        Eval {
                                            expr: 7,
                                            coersion: Some(
                                                WrapInSome,
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_arena: Arena {
                                    data: [
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `dp`,
                                            },
                                            contract: Pure,
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
                                                    `cc`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `dp`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Ritchie(
                            Fn,
                        )`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Simple {
                                    pattern_expr_idx: 0,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
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
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
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
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Ritchie(
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
                            pattern_arena: Arena {
                                data: [
                                    HirEagerPatternEntry {
                                        data: HirEagerPatternData::Ident {
                                            symbol_modifier: None,
                                            ident: `cc`,
                                        },
                                        contract: Pure,
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
                                                `cc`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            8,
                            HirEagerExprRegion {
                                region_path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::two::right_cc_pattern`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
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
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 0,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::displacement`, `MethodRitchie(
                                                        Fn,
                                                    )`),
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
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
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
                                                self_argument: 2,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
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
                                                quary: Const,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 3,
                                                opr: Comparison(
                                                    Greater,
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
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: ImmutableOnStack {
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
                                                        Move,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 6,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: ImmutableOnStack {
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
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..3,
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: ImmutableOnStack {
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
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 0,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 1,
                                            coersion: None,
                                        },
                                        Require {
                                            condition: Other {
                                                opd: 5,
                                                conversion: None,
                                            },
                                        },
                                        Eval {
                                            expr: 7,
                                            coersion: Some(
                                                WrapInSome,
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_arena: Arena {
                                    data: [
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `dp`,
                                            },
                                            contract: Pure,
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
                                                    `cc`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `dp`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Ritchie(
                            Fn,
                        )`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Simple {
                                    pattern_expr_idx: 0,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
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
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
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
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Ritchie(
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
                            pattern_arena: Arena {
                                data: [
                                    HirEagerPatternEntry {
                                        data: HirEagerPatternData::Ident {
                                            symbol_modifier: None,
                                            ident: `cc`,
                                        },
                                        contract: Pure,
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
                                                `cc`,
                                            ),
                                            data: HirEagerRuntimeSvarData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            8,
                            HirEagerExprRegion {
                                region_path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::two::down_cc_pattern`, `Ritchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
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
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 0,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`<mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)>::displacement`, `MethodRitchie(
                                                        Fn,
                                                    )`),
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
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
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
                                                self_argument: 2,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                contract: Some(
                                                    Pure,
                                                ),
                                                quary: ImmutableOnStack {
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
                                                quary: Const,
                                            },
                                            is_always_copyable: true,
                                            place_contract_site: HirPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 3,
                                                opr: Comparison(
                                                    Greater,
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
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: ImmutableOnStack {
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
                                                        Move,
                                                    ),
                                                ],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                self_argument: 6,
                                                self_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
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
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: ImmutableOnStack {
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
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    0..3,
                                                ),
                                            },
                                            contracted_quary: HirContractedQuary {
                                                contract: Some(
                                                    Move,
                                                ),
                                                quary: ImmutableOnStack {
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
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 0,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 1,
                                            coersion: None,
                                        },
                                        Require {
                                            condition: Other {
                                                opd: 5,
                                                conversion: None,
                                            },
                                        },
                                        Eval {
                                            expr: 7,
                                            coersion: Some(
                                                WrapInSome,
                                            ),
                                            discarded: false,
                                        },
                                    ],
                                },
                                pattern_arena: Arena {
                                    data: [
                                        HirEagerPatternEntry {
                                            data: HirEagerPatternData::Ident {
                                                symbol_modifier: None,
                                                ident: `dp`,
                                            },
                                            contract: Pure,
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
                                                    `cc`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `dp`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Ki(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::two::is_two`, `Val`),
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
                    hir_expr_body_and_region: Some(
                        (
                            Eager(
                                101,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 117,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
]
```