[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Ritchie(
                        Fn,
                    )`),
                    hir_decl: FunctionMajorFnHirDecl {
                        path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Ritchie(
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
                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
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
                                ty_path: TypePath(`core::basic::bool`, `Extern`),
                                template_arguments: [],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Ritchie(
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
                                    HirEagerPatternExpr::Ident {
                                        symbol_modifier: None,
                                        ident: `index`,
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
                                        HirEagerRuntimeSvarEntry {
                                            name: HirEagerRuntimeSvarName::Ident(
                                                `index`,
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
                            86,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Ritchie(
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
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 1,
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
                                            },
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 2,
                                                self_contract: Pure,
                                                ident: `ilen`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::vec::Vec(0)::ilen`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 4,
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
                                            },
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 6,
                                                opr: Closed(
                                                    RemEuclid,
                                                ),
                                                ropd: 7,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 5,
                                                items: [
                                                    8,
                                                ],
                                            },
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 9,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`, `MethodRitchie(
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
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 11,
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
                                            },
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 13,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 14,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 15,
                                                opr: Closed(
                                                    RemEuclid,
                                                ),
                                                ropd: 16,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 12,
                                                items: [
                                                    17,
                                                ],
                                            },
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 18,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::line_segment_sketch::LineSegmentStroke(0)::displacement`, `MethodRitchie(
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
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                5,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            3,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 20,
                                                self_contract: Pure,
                                                ident: `rotation_direction_to`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::geom2d::Vector2d(0)::rotation_direction_to`, `MethodRitchie(
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
                                                        21,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    place: Idx(
                                                                        PlaceIdx(
                                                                            ShiftedU32(
                                                                                2,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            4,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 23,
                                                opr: Comparison(
                                                    Eq,
                                                ),
                                                ropd: 24,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            999999.0,
                                                        ),
                                                        text: "999999.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 26,
                                            },
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 28,
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
                                            },
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 30,
                                                opr: Closed(
                                                    Sub,
                                                ),
                                                ropd: 31,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 32,
                                                opr: Closed(
                                                    RemEuclid,
                                                ),
                                                ropd: 33,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 29,
                                                items: [
                                                    34,
                                                ],
                                            },
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 35,
                                                ident: `points`,
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
                                            },
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                8,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            6,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 37,
                                                self_contract: Pure,
                                                ident: `start`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                8,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            6,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 39,
                                                self_contract: Pure,
                                                ident: `end`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 41,
                                                ident: `contour`,
                                                field_ty: HirType::PathLeading(
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
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                8,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            6,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 43,
                                                self_contract: Pure,
                                                ident: `start`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                9,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            7,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 42,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::displacement`, `MethodRitchie(
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
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        44,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
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
                                                        45,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    place: Idx(
                                                                        PlaceIdx(
                                                                            ShiftedU32(
                                                                                7,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                7,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            5,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                7,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            5,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                10,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            8,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 49,
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
                                                        50,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    place: Idx(
                                                                        PlaceIdx(
                                                                            ShiftedU32(
                                                                                8,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 48,
                                                self_contract: Pure,
                                                ident: `max`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::max`, `MethodRitchie(
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
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        51,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 47,
                                                opr: Assign,
                                                ropd: 52,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::F32(
                                                    TermF32Literal {
                                                        value: OrderedFloat(
                                                            999999.0,
                                                        ),
                                                        text: "999999.0f32",
                                                    },
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 54,
                                            },
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 56,
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
                                            },
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                2,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 58,
                                                opr: Closed(
                                                    RemEuclid,
                                                ),
                                                ropd: 59,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Index {
                                                owner_hir_expr_idx: 57,
                                                items: [
                                                    60,
                                                ],
                                            },
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 61,
                                                ident: `points`,
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
                                            },
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                12,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            10,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 63,
                                                self_contract: Pure,
                                                ident: `start`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                12,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            10,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 65,
                                                self_contract: Pure,
                                                ident: `end`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::slice::CyclicSlice(0)::end`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::PropsStructField {
                                                owner_hir_expr_idx: 67,
                                                ident: `contour`,
                                                field_ty: HirType::PathLeading(
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
                                            ty_place: StackPure {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            1,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                8,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            6,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 69,
                                                self_contract: Pure,
                                                ident: `start`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::slice::CyclicSlice(0)::start`, `MethodRitchie(
                                                        Fn,
                                                    )`),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            HirTemplateVar::Type(
                                                                HirTypeSvar::Type {
                                                                    attrs: HirTemplateSvarAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            HirTermSvarResolution::Explicit(
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
                                                item_groups: [],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                13,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            11,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 68,
                                                self_contract: Pure,
                                                ident: `displacement`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(mnist_classifier::raw_contour::RawContour(0)::displacement`, `MethodRitchie(
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
                                                                        value: 4,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        70,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
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
                                                        71,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    place: Idx(
                                                                        PlaceIdx(
                                                                            ShiftedU32(
                                                                                11,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            9,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            9,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            2,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                14,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            12,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: false,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 75,
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
                                                        76,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    place: Idx(
                                                                        PlaceIdx(
                                                                            ShiftedU32(
                                                                                12,
                                                                            ),
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::MethodFnCall {
                                                self_argument: 74,
                                                self_contract: Pure,
                                                ident: `max`,
                                                path: AssocItemPath::TypeItem(
                                                    TypeItemPath(`(core::num::f32(0)::max`, `MethodRitchie(
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
                                                                        value: 9,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        77,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 73,
                                                opr: Assign,
                                                ropd: 78,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                11,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            9,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                7,
                                            ),
                                            ty_place: MutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            5,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 80,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                ropd: 81,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Variable(
                                                6,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                place: Idx(
                                                    PlaceIdx(
                                                        ShiftedU32(
                                                            4,
                                                        ),
                                                    ),
                                                ),
                                            },
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Literal(
                                                Literal::I32(
                                                    0,
                                                ),
                                            ),
                                            ty_place: Const,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Binary {
                                                lopd: 83,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 84,
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: HirEagerExprData::Block {
                                                stmts: ArenaIdxRange(
                                                    13..18,
                                                ),
                                            },
                                            ty_place: Transient,
                                            is_always_copyable: true,
                                            place_contracts: HirEagerPlaceContractSite {
                                                data: [],
                                            },
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 7,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 46,
                                            coersion: None,
                                        },
                                        Eval {
                                            expr_idx: 53,
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
                                                pattern_expr_idx: 10,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 72,
                                            coersion: None,
                                        },
                                        Eval {
                                            expr_idx: 79,
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
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 27,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 6,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 36,
                                            coersion: None,
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
                                                            38,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            40,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                1..3,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 8,
                                                ty: None,
                                            },
                                            contract: Move,
                                            initial_value: 55,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 9,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 62,
                                            coersion: None,
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
                                                            64,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            66,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                3..5,
                                            ),
                                        },
                                        Return {
                                            result: 82,
                                            coersion: Trivial(
                                                TrivialHirEagerCoersion {
                                                    expectee_place: Transient,
                                                },
                                            ),
                                        },
                                        Return {
                                            result: 85,
                                            coersion: Trivial(
                                                TrivialHirEagerCoersion {
                                                    expectee_place: Transient,
                                                },
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 3,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 10,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 19,
                                            coersion: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            contract: Pure,
                                            initial_value: 22,
                                            coersion: None,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 25,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    5..12,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                HirEagerElseBranch {
                                                    stmts: ArenaIdxRange(
                                                        12..13,
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
                                            ident: `L`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `current_displacement`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `previous_displacement`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `is_rotation_counterclockwise_result`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `previous_raw_cross`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `previous_interval`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `displacement`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: `current_raw_cross`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `current_interval`,
                                        },
                                        HirEagerPatternExpr::Ident {
                                            symbol_modifier: None,
                                            ident: `displacement`,
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
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `index`,
                                                ),
                                                data: HirEagerRuntimeSvarData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `L`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `current_displacement`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `previous_displacement`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `is_rotation_counterclockwise_result`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `previous_raw_cross`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `previous_interval`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `i1`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LoopVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `displacement`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `current_raw_cross`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `current_interval`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LetVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `i2`,
                                                ),
                                                data: HirEagerRuntimeSvarData::LoopVariable,
                                            },
                                            HirEagerRuntimeSvarEntry {
                                                name: HirEagerRuntimeSvarName::Ident(
                                                    `displacement`,
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
]