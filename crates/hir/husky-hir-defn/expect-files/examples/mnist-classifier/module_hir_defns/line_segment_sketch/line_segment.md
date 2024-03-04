[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        fields: [
                            PropsStructFieldHirDecl {
                                ident: `start`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                        always_copyable: false,
                                    },
                                ),
                                initialization: None,
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
                                initialization: None,
                            },
                        ],
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                                                `start`,
                                            ),
                                            data: HirEagerRuntimeSvarData::FieldVariable,
                                        },
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `end`,
                                            ),
                                            data: HirEagerRuntimeSvarData::FieldVariable,
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
                        ItemPathId(
                            Id {
                                value: 298,
                            },
                        ),
                    ),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TypeImplBlock(
                                    TypeImplBlockPath(
                                        ItemPathId(
                                            Id {
                                                value: 298,
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
            },
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`, `MethodRitchie(
                        Fn,
                    )`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`, `MethodRitchie(
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
                                        value: 44,
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
                            region_path: RegionPath::Decl(
                                ItemPath::AssocItem(
                                    AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`, `MethodRitchie(
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            6,
                            HirEagerExprRegion {
                                region_path: RegionPath::Defn(
                                    ItemPath::AssocItem(
                                        AssocItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`, `MethodRitchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                                owner_hir_expr_idx: 1,
                                                ident: `start`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                                owner_hir_expr_idx: 3,
                                                ident: `end`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                                self_argument: 2,
                                                self_contract: Pure,
                                                ident: `to`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::to`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
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
                                                        4,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    place: Idx(
                                                                        PlaceIdx(0),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
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
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::AssocItem(
        AssocItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::dist_to_point`, `MethodRitchie(
                        Fn,
                    )`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::dist_to_point`, `MethodRitchie(
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
                                        value: 44,
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
                            region_path: RegionPath::Decl(
                                ItemPath::AssocItem(
                                    AssocItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::dist_to_point`, `MethodRitchie(
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
                                        ident: `pt`,
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
                                                `pt`,
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
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            32,
                            HirEagerExprRegion {
                                region_path: RegionPath::Defn(
                                    ItemPath::AssocItem(
                                        AssocItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::dist_to_point`, `MethodRitchie(
                                                Fn,
                                            )`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                                self_argument: 1,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`, `MethodRitchie(
                                                        Fn,
                                                    )`),
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
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                                owner_hir_expr_idx: 3,
                                                ident: `start`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 4,
                                                self_contract: Pure,
                                                ident: `to`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::to`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
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
                                                        5,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    place: Idx(
                                                                        PlaceIdx(1),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                            ty_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 7,
                                                self_contract: Pure,
                                                ident: `dot`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::dot`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 34,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        8,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: ImmutableOnStack {
                                                                    place: Idx(
                                                                        PlaceIdx(2),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            0.0,
                                                        ),
                                                        text: "0.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 9,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                ropd: 10,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 12,
                                                self_contract: Pure,
                                                ident: `norm`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodRitchie(
                                                        Fn,
                                                    )`),
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
                                            is_always_copyable: true,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                                owner_hir_expr_idx: 14,
                                                ident: `end`,
                                                field_ty: HirType::PathLeading(
                                                    HirTypePathLeading {
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        template_arguments: [],
                                                        always_copyable: false,
                                                    },
                                                ),
                                            },
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(0),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 15,
                                                self_contract: Pure,
                                                ident: `to`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Point2d(0)::to`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
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
                                                        16,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: StackPure {
                                                                    place: Idx(
                                                                        PlaceIdx(1),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                            ty_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(3),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 18,
                                                self_contract: Pure,
                                                ident: `dot`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::dot`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 34,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        19,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: ImmutableOnStack {
                                                                    place: Idx(
                                                                        PlaceIdx(3),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            0.0,
                                                        ),
                                                        text: "0.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 20,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 21,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(3),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 23,
                                                self_contract: Pure,
                                                ident: `norm`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodRitchie(
                                                        Fn,
                                                    )`),
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
                                            is_always_copyable: true,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                            ty_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(2),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 25,
                                                self_contract: Pure,
                                                ident: `cross`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::cross`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: Some(
                                                        0,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieSimpleParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 34,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        26,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: ImmutableOnStack {
                                                                    place: Idx(
                                                                        PlaceIdx(2),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 27,
                                                self_contract: Pure,
                                                ident: `abs`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::abs`, `MethodRitchie(
                                                        Fn,
                                                    )`),
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
                                            is_always_copyable: true,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableOnStack {
                                                place: Idx(
                                                    PlaceIdx(1),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contract_site: HirEagerPlaceContractSite {
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
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 29,
                                                self_contract: Pure,
                                                ident: `norm`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::norm`, `MethodRitchie(
                                                        Fn,
                                                    )`),
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
                                            is_always_copyable: true,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 28,
                                                opr: Closed(
                                                    Div,
                                                ),
                                                ropd: 30,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    6..9,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contract_site: HirEagerPlaceContractSite {
                                                place_contracts: [],
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 13,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 24,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 31,
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
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 17,
                                            coersion: None,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 22,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    2..3,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                HirEagerElseBranch {
                                                    stmts: ArenaIdxRange(
                                                        3..4,
                                                    ),
                                                },
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 2,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 6,
                                            coersion: None,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 11,
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
                                                        4..6,
                                                    ),
                                                },
                                            ),
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `ab`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `ap`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `bp`,
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
                                                    `pt`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `ab`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `ap`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `bp`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
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