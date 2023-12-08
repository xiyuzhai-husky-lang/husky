[
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructHirDefn {
                    path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
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
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                `line_segment_sketch`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `strokes`,
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
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnHirDefn {
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
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
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
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
                                                    value: 211,
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
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `line_segment_sketch`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                    ],
                                },
                                self_value_variable: None,
                            },
                        },
                    },
                    eager_body_with_hir_eager_expr_region: Some(
                        (
                            62,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: NewList {
                                                items: [],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 2,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 3,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 134,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 236,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                I32(
                                                    0,
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Prefix {
                                                opr: Minus,
                                                opd_hir_expr_idx: 8,
                                            },
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 7,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 9,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Fugitive(
                                                        FugitivePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 306,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: FunctionFnCall {
                                                path: FugitivePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 306,
                                                        },
                                                    ),
                                                ),
                                                function_hir_eager_expr_idx: 11,
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
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
                                                        12,
                                                        Deref(
                                                            Leash,
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 6,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        13,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
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
                                        },
                                        HirEagerExprEntry {
                                            data: Prefix {
                                                opr: Not,
                                                opd_hir_expr_idx: 14,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 10,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 15,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Suffix {
                                                opd_hir_expr_idx: 17,
                                                opr: Decr,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                6,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 21,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 22,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 20,
                                                opr: Comparison(
                                                    Less,
                                                ),
                                                ropd: 23,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                3,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 26,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 27,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 25,
                                                opr: Comparison(
                                                    Leq,
                                                ),
                                                ropd: 28,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Fugitive(
                                                        FugitivePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 306,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: FunctionFnCall {
                                                path: FugitivePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 306,
                                                        },
                                                    ),
                                                ),
                                                function_hir_eager_expr_idx: 30,
                                                instantiation: HirInstantiation {
                                                    symbol_map: [],
                                                    separator: None,
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
                                                        31,
                                                        Deref(
                                                            Leash,
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 6,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        32,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            4,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Prefix {
                                                opr: Not,
                                                opd_hir_expr_idx: 33,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 29,
                                                opr: ShortCircuitLogic(
                                                    And,
                                                ),
                                                ropd: 34,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Suffix {
                                                opd_hir_expr_idx: 36,
                                                opr: Incr,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 39,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 40,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 38,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 41,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Type(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 303,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: StackPure {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 46,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 47,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 141,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 242,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 6,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        48,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            3,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 6,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        49,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            4,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: TypeConstructorFnCall {
                                                path: TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 303,
                                                        },
                                                    ),
                                                ),
                                                function_hir_eager_expr_idx: 44,
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
                                                                        value: 64,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        45,
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
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 66,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        50,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 43,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 135,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 237,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 47,
                                                                            },
                                                                        ),
                                                                    ),
                                                                ),
                                                            ),
                                                        ),
                                                        (
                                                            Type(
                                                                SelfLifetime,
                                                            ),
                                                            SelfLifetime,
                                                        ),
                                                    ],
                                                    separator: Some(
                                                        1,
                                                    ),
                                                },
                                                item_groups: [
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Move,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 47,
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
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 53,
                                                opr: Assign,
                                                ropd: 54,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                I32(
                                                    1,
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 57,
                                                opr: Closed(
                                                    Add,
                                                ),
                                                ropd: 58,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 56,
                                                opr: Assign,
                                                ropd: 59,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    8..16,
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 18,
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
                                            expr_idx: 37,
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
                                            expr_idx: 52,
                                            coersion: Some(
                                                Trivial(
                                                    TrivialHirEagerCoersion {
                                                        expectee_place: Transient,
                                                    },
                                                ),
                                            ),
                                            discarded: false,
                                        },
                                        While {
                                            condition: Other {
                                                hir_eager_expr_idx: 35,
                                                conversion: None,
                                            },
                                            stmts: ArenaIdxRange(
                                                2..3,
                                            ),
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 42,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    3..4,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 55,
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
                                            expr_idx: 60,
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
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 48,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            initial_value: 1,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 4,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 5,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 6,
                                        },
                                        While {
                                            condition: Other {
                                                hir_eager_expr_idx: 16,
                                                conversion: None,
                                            },
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            initial_value: 19,
                                        },
                                        While {
                                            condition: Other {
                                                hir_eager_expr_idx: 24,
                                                conversion: None,
                                            },
                                            stmts: ArenaIdxRange(
                                                4..8,
                                            ),
                                        },
                                        Return {
                                            result: 61,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 368,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 23,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 142,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 143,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 412,
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
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `line_segment_sketch`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `concave_components`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `L`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `start`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `end`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ccv_start`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
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
    HirDefn::ImplBlock(
        ImplBlockHirDefn::TraitForType(
            TraitForTypeImplBlockHirDefn {
                hir_decl: TraitForTypeImplBlockHirDecl {
                    path: TraitForTypeImplBlock {
                        data: TraitForTypeImplBlockPathData {
                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                    },
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
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TraitForTypeImplBlock(
                                    TraitForTypeImplBlock {
                                        data: TraitForTypeImplBlockPathData {
                                            module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                            trai_path: TraitPath(`core::visual::Visualize`),
                                            ty_sketch: TypeSketch::Path(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
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
        AssociatedItemHirDefn::TraitForTypeItem(
            TraitForTypeItemHirDefn::MethodFn(
                TraitForTypeMethodFnHirDefn {
                    path: TraitForTypeItemPath(
                        ItemPathId {
                            data: ItemPathData::AssociatedItem(
                                AssociatedItemPathData::TraitForTypeItem(
                                    TraitForTypeItemPathData {
                                        impl_block: TraitForTypeImplBlock {
                                            data: TraitForTypeImplBlockPathData {
                                                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                                disambiguator: 0,
                                            },
                                        },
                                        ident: `visualize`,
                                        item_kind: MethodFn,
                                    },
                                ),
                            ),
                        },
                    ),
                    hir_decl: TraitForTypeMethodFnHirDecl {
                        path: TraitForTypeItemPath(
                            ItemPathId {
                                data: ItemPathData::AssociatedItem(
                                    AssociatedItemPathData::TraitForTypeItem(
                                        TraitForTypeItemPathData {
                                            impl_block: TraitForTypeImplBlock {
                                                data: TraitForTypeImplBlockPathData {
                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                            },
                                            ident: `visualize`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            },
                        ),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
                        parenate_parameters: HirEagerParenateParameters(
                            [],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::visual::Html`, `Extern`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath(
                                            ItemPathId {
                                                data: ItemPathData::AssociatedItem(
                                                    AssociatedItemPathData::TraitForTypeItem(
                                                        TraitForTypeItemPathData {
                                                            impl_block: TraitForTypeImplBlock {
                                                                data: TraitForTypeImplBlockPathData {
                                                                    module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                                    ty_sketch: TypeSketch::Path(
                                                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                    ),
                                                                    disambiguator: 0,
                                                                },
                                                            },
                                                            ident: `visualize`,
                                                            item_kind: MethodFn,
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
                                        AssociatedItemPath::TraitForTypeItem(
                                            TraitForTypeItemPath(
                                                ItemPathId {
                                                    data: ItemPathData::AssociatedItem(
                                                        AssociatedItemPathData::TraitForTypeItem(
                                                            TraitForTypeItemPathData {
                                                                impl_block: TraitForTypeImplBlock {
                                                                    data: TraitForTypeImplBlockPathData {
                                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                                        ty_sketch: TypeSketch::Path(
                                                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                                        ),
                                                                        disambiguator: 0,
                                                                    },
                                                                },
                                                                ident: `visualize`,
                                                                item_kind: MethodFn,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 2,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 152,
                                                        },
                                                    ),
                                                ),
                                                path: TraitForTypeItem(
                                                    TraitForTypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 245,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                SelfType,
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 65,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
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
                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                                        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`, `MemoizedField`),
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`, `MemoizedField`),
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
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`, `MemoizedField`),
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
                            3,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::norm`, `MemoizedField`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MemoizedField {
                                                owner_hir_expr_idx: 1,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 403,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 451,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 2,
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
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`, `MemoizedField`),
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`, `MemoizedField`),
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
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`, `MemoizedField`),
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
                            7,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::rel_norm`, `MemoizedField`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MemoizedField {
                                                owner_hir_expr_idx: 1,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 345,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 449,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 3,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 293,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 458,
                                                            },
                                                        ),
                                                    ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 4,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 345,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 473,
                                                            },
                                                        ),
                                                    ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 2,
                                                opr: Closed(
                                                    Div,
                                                ),
                                                ropd: 5,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 6,
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
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`, `MemoizedField`),
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`, `MemoizedField`),
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
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`, `MemoizedField`),
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
                            33,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::hausdorff_norm`, `MemoizedField`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Literal(
                                                F32(
                                                    TermF32Literal(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 2,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 3,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 137,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 232,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Suffix {
                                                opd_hir_expr_idx: 4,
                                                opr: Unwrap,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 5,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 142,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 7,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 215,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 455,
                                                            },
                                                        ),
                                                    ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 9,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 293,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 496,
                                                            },
                                                        ),
                                                    ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 10,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 345,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 473,
                                                            },
                                                        ),
                                                    ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 12,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 13,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 142,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 230,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 15,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 16,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 143,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 231,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 18,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                6,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Index {
                                                owner_hir_expr_idx: 19,
                                                items: [
                                                    20,
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 21,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 143,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                7,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 23,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 408,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 497,
                                                            },
                                                        ),
                                                    ),
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
                                                        24,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            6,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                8,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        7,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 26,
                                                opr: Comparison(
                                                    Greater,
                                                ),
                                                ropd: 27,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                8,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        7,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 29,
                                                opr: Assign,
                                                ropd: 30,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    5..11,
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
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
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            initial_value: 22,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 6,
                                                ty: None,
                                            },
                                            initial_value: 25,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: Other {
                                                    hir_eager_expr_idx: 28,
                                                    conversion: None,
                                                },
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 1,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 6,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 8,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 11,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 251,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            14,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            17,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                2..5,
                                            ),
                                        },
                                        Return {
                                            result: 32,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 403,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 405,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 406,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 385,
                                                    },
                                                ),
                                            ),
                                        },
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
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 407,
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
                                                    `hausdorff_norm`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `curve_start`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `curve_ls`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dp_norm`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LoopVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `point`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `point_dist`,
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
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`, `MemoizedField`),
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`, `MemoizedField`),
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
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`, `MemoizedField`),
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
                            30,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::angle_change`, `MemoizedField`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Literal(
                                                F32(
                                                    TermF32Literal(
                                                        Id {
                                                            value: 2,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 2,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 4,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 5,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 142,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 230,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Index {
                                                owner_hir_expr_idx: 3,
                                                items: [
                                                    6,
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 7,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 293,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 470,
                                                            },
                                                        ),
                                                    ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 9,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 10,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 142,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 230,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 12,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 13,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 143,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 231,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 15,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Index {
                                                owner_hir_expr_idx: 16,
                                                items: [
                                                    17,
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 18,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 293,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 470,
                                                            },
                                                        ),
                                                    ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                3,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Literal(
                                                Bool(
                                                    true,
                                                ),
                                            ),
                                            ty_place: Const,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 21,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 354,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 478,
                                                            },
                                                        ),
                                                    ),
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
                                                        22,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: ImmutableStackOwned {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            4,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                    Regular(
                                                        HirRitchieRegularParameter {
                                                            contract: Pure,
                                                            ty: PathLeading(
                                                                HirTypePathLeading(
                                                                    Id {
                                                                        value: 54,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        23,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Const,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 20,
                                                opr: AssignClosed(
                                                    Add,
                                                ),
                                                ropd: 24,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                3,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 26,
                                                opr: Assign,
                                                ropd: 27,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    4..8,
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 19,
                                        },
                                        Eval {
                                            expr_idx: 25,
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
                                            expr_idx: 28,
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
                                            initial_value: 1,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 8,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 251,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            11,
                                                        ),
                                                        kind: LowerOpen,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            14,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                1..4,
                                            ),
                                        },
                                        Return {
                                            result: 29,
                                        },
                                    ],
                                },
                                pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 342,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 390,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 380,
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
                                                    `angle_change`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dp0`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LoopVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dp`,
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
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`, `MemoizedField`),
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`, `MemoizedField`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                template_arguments: [],
                                always_copyable: false,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`, `MemoizedField`),
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
                            59,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::bounding_box`, `MemoizedField`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 2,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 137,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 232,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Suffix {
                                                opd_hir_expr_idx: 3,
                                                opr: Unwrap,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 4,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 142,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 6,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 266,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 8,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 266,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 10,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 267,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                2,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 12,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 267,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 14,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 15,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 142,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 230,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 17,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 18,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 143,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 231,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 20,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                7,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        6,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Index {
                                                owner_hir_expr_idx: 21,
                                                items: [
                                                    22,
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 23,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 143,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                3,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                3,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                8,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        7,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 27,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 266,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 26,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 59,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 168,
                                                            },
                                                        ),
                                                    ),
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
                                                        28,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 25,
                                                opr: Assign,
                                                ropd: 29,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                8,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        7,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 33,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 266,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 32,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 54,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 167,
                                                            },
                                                        ),
                                                    ),
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
                                                        34,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 31,
                                                opr: Assign,
                                                ropd: 35,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                8,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        7,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 39,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 267,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 38,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 59,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 168,
                                                            },
                                                        ),
                                                    ),
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
                                                        40,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 37,
                                                opr: Assign,
                                                ropd: 41,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                8,
                                            ),
                                            ty_place: ImmutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        7,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 45,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 267,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 44,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 54,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 167,
                                                            },
                                                        ),
                                                    ),
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
                                                        46,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Binary {
                                                lopd: 43,
                                                opr: Assign,
                                                ropd: 47,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Type(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 290,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Type(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 289,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                3,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        2,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                4,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        3,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: TypeConstructorFnCall {
                                                path: TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 289,
                                                        },
                                                    ),
                                                ),
                                                function_hir_eager_expr_idx: 50,
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
                                                        51,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
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
                                                        52,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
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
                                        },
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Type(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 289,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                5,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        4,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                6,
                                            ),
                                            ty_place: MutableStackOwned {
                                                location: StackLocationIdx(
                                                    ShiftedU32(
                                                        5,
                                                    ),
                                                ),
                                            },
                                        },
                                        HirEagerExprEntry {
                                            data: TypeConstructorFnCall {
                                                path: TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 289,
                                                        },
                                                    ),
                                                ),
                                                function_hir_eager_expr_idx: 54,
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
                                                        55,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            4,
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
                                                        56,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: MutableStackOwned {
                                                                    location: StackLocationIdx(
                                                                        ShiftedU32(
                                                                            5,
                                                                        ),
                                                                    ),
                                                                },
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: TypeConstructorFnCall {
                                                path: TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 290,
                                                        },
                                                    ),
                                                ),
                                                function_hir_eager_expr_idx: 49,
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
                                                                        value: 58,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        53,
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
                                                                        value: 58,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        57,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    6..13,
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 6,
                                                ty: None,
                                            },
                                            initial_value: 24,
                                        },
                                        Eval {
                                            expr_idx: 30,
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
                                            expr_idx: 36,
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
                                            expr_idx: 42,
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
                                            expr_idx: 48,
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
                                            initial_value: 5,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 7,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 9,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 11,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            initial_value: 13,
                                        },
                                        ForBetween {
                                            particulars: HirEagerForBetweenParticulars {
                                                frame_var_ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 251,
                                                        },
                                                    ),
                                                ),
                                                range: HirEagerForBetweenRange {
                                                    initial_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            16,
                                                        ),
                                                        kind: LowerClosed,
                                                    },
                                                    final_boundary: HirEagerForBetweenLoopBoundary {
                                                        bound_expr: Some(
                                                            19,
                                                        ),
                                                        kind: UpperOpen,
                                                    },
                                                    step: Constant(
                                                        1,
                                                    ),
                                                },
                                            },
                                            block: ArenaIdxRange(
                                                1..6,
                                            ),
                                        },
                                        Return {
                                            result: 58,
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
                                                        value: 284,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 285,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 286,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 287,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 288,
                                                    },
                                                ),
                                            ),
                                        },
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
                                                    `start_point`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `xmin`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `xmax`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ymin`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ymax`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `i`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LoopVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `point`,
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
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`, `MemoizedField`),
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`, `MemoizedField`),
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
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`, `MemoizedField`),
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
                            7,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`, `MemoizedField`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 211,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MemoizedField {
                                                owner_hir_expr_idx: 2,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 282,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 480,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MemoizedField {
                                                owner_hir_expr_idx: 4,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 282,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 453,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 3,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 291,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 482,
                                                            },
                                                        ),
                                                    ),
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
                                                                        value: 60,
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
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 6,
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
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`, `MethodFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        self_value_parameter: HirEagerSelfValueParameter,
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
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`, `MethodFn`),
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
                            15,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::line_segment`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: PrincipalEntityPath(
                                                MajorItem(
                                                    Type(
                                                        TypePath(
                                                            ItemPathId(
                                                                Id {
                                                                    value: 307,
                                                                },
                                                            ),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 2,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 3,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 137,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 232,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Suffix {
                                                opd_hir_expr_idx: 4,
                                                opr: Unwrap,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 5,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 142,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 6,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 5,
                                                        },
                                                    ),
                                                ),
                                                path: TraitForTypeItem(
                                                    TraitForTypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 136,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                SelfType,
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 44,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 8,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 9,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 138,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 233,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Suffix {
                                                opd_hir_expr_idx: 10,
                                                opr: Unwrap,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 11,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 143,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 12,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 5,
                                                        },
                                                    ),
                                                ),
                                                path: TraitForTypeItem(
                                                    TraitForTypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 136,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                SelfType,
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 44,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: TypeConstructorFnCall {
                                                path: TypePath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 307,
                                                        },
                                                    ),
                                                ),
                                                function_hir_eager_expr_idx: 1,
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
                                                                        value: 44,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        7,
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
                                                                        value: 44,
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                        13,
                                                        Trivial(
                                                            TrivialHirEagerCoersion {
                                                                expectee_place: Transient,
                                                            },
                                                        ),
                                                    ),
                                                ],
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
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
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`, `MethodFn`),
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
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`, `MethodFn`),
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
                            7,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 2,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 137,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 232,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Suffix {
                                                opd_hir_expr_idx: 3,
                                                opr: Unwrap,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 4,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 142,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 5,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 5,
                                                        },
                                                    ),
                                                ),
                                                path: TraitForTypeItem(
                                                    TraitForTypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 136,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                SelfType,
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 44,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 6,
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
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`, `MethodFn`),
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
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`, `MethodFn`),
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
                            7,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 2,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 138,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 233,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Suffix {
                                                opd_hir_expr_idx: 3,
                                                opr: Unwrap,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 4,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 143,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 5,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 5,
                                                        },
                                                    ),
                                                ),
                                                path: TraitForTypeItem(
                                                    TraitForTypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 136,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                SelfType,
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 44,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 6,
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
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
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
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
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
                                            TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::displacement`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 1,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 215,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 455,
                                                            },
                                                        ),
                                                    ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 2,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 293,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 496,
                                                            },
                                                        ),
                                                    ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
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
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`, `MethodFn`),
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
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`, `MethodFn`),
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
                                            TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::start_tangent`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 2,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 137,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 232,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Suffix {
                                                opd_hir_expr_idx: 3,
                                                opr: Unwrap,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 4,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 293,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 470,
                                                            },
                                                        ),
                                                    ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
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
                    path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`, `MethodFn`),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`, `MethodFn`),
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
                                        TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`, `MethodFn`),
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
                                            TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::end_tangent`, `MethodFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprEntry {
                                            data: Variable(
                                                1,
                                            ),
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: PropsStructField {
                                                owner_hir_expr_idx: 1,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 367,
                                                        },
                                                    ),
                                                ),
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 2,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 138,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 233,
                                                            },
                                                        ),
                                                    ),
                                                ),
                                                instantiation: HirInstantiation {
                                                    symbol_map: [
                                                        (
                                                            Type(
                                                                Type {
                                                                    attrs: HirTemplateSymbolAttrs {
                                                                        class: Comptime,
                                                                    },
                                                                    variance: None,
                                                                    disambiguator: 0,
                                                                },
                                                            ),
                                                            Explicit(
                                                                Type(
                                                                    PathLeading(
                                                                        HirTypePathLeading(
                                                                            Id {
                                                                                value: 53,
                                                                            },
                                                                        ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Suffix {
                                                opd_hir_expr_idx: 3,
                                                opr: Unwrap,
                                            },
                                            ty_place: Transient,
                                        },
                                        HirEagerExprEntry {
                                            data: MethodFnCall {
                                                self_argument: 4,
                                                ident: Ident(
                                                    Coword(
                                                        Id {
                                                            value: 293,
                                                        },
                                                    ),
                                                ),
                                                path: TypeItem(
                                                    TypeItemPath(
                                                        ItemPathId(
                                                            Id {
                                                                value: 470,
                                                            },
                                                        ),
                                                    ),
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
                                        },
                                        HirEagerExprEntry {
                                            data: Block {
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            ty_place: Transient,
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
]