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
                                    },
                                ),
                            },
                            PropsStructFieldHirDecl {
                                ident: `end`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                ),
                            },
                        ],
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
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
                                                `start`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `end`,
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
                                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                            template_arguments: [],
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
                                                        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
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
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`, `MethodFn`),
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
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`, `MethodFn`),
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
                                            TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 1,
                                            ident: `start`,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 3,
                                            ident: `end`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 2,
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
                                                    4,
                                                    Trivial(
                                                        TrivialHirEagerCoersion {
                                                            expectee_place: Transient,
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 5,
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
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::dist_to_point`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::dist_to_point`, `MethodFn`),
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
                                        },
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::f32`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::dist_to_point`, `MethodFn`),
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
                                                    value: 422,
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
                                                `pt`,
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
                            32,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::dist_to_point`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 1,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::line_segment_sketch::line_segment::LineSegment(0)::displacement`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 3,
                                            ident: `start`,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 4,
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
                                                    5,
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
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 7,
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
                                                                    value: 59,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    8,
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
                                            ],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::F32(
                                                NotNan(
                                                    0.0,
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 9,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 10,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 12,
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
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 14,
                                            ident: `end`,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 15,
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
                                                    16,
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
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 18,
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
                                                                    value: 59,
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    19,
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
                                        HirEagerExprData::Literal(
                                            TermLiteral::F32(
                                                NotNan(
                                                    0.0,
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 20,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 21,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 23,
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
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 25,
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
                                                                    value: 59,
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
                                                                        2,
                                                                    ),
                                                                ),
                                                            },
                                                        },
                                                    ),
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 27,
                                            ident: `abs`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::num::f32(0)::abs`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 29,
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
                                        HirEagerExprData::Binary {
                                            lopd: 28,
                                            opr: Closed(
                                                Div,
                                            ),
                                            ropd: 30,
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                6..9,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 13,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 24,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 31,
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 17,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other(
                                                    22,
                                                ),
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
                                            initial_value: 2,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 6,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other(
                                                    11,
                                                ),
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
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 423,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 424,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 425,
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
                                                    `pt`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ab`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ap`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `bp`,
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
]