[
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: SubmodulePath(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: SubmodulePath(
                    `mnist_classifier::line_segment_sketch::convex_component`,
                ),
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: SubmodulePath(
                    `mnist_classifier::line_segment_sketch::convexity`,
                ),
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: SubmodulePath(
                    `mnist_classifier::line_segment_sketch::line_segment`,
                ),
            },
        },
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructTypeHirDefn {
                    path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        fields: [
                            PropsFieldHirDecl {
                                ident: `points`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            Type(
                                                PathLeading(
                                                    HirTypePathLeading(
                                                        Id {
                                                            value: 65,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ],
                                    },
                                ),
                            },
                            PropsFieldHirDecl {
                                ident: `start`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                        template_arguments: [],
                                    },
                                ),
                            },
                            PropsFieldHirDecl {
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
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `points`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::FieldVariable,
                                        },
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Type(
            TypeHirDefn::PropsStruct(
                PropsStructTypeHirDefn {
                    path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                    hir_decl: PropsStructTypeHirDecl {
                        path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        fields: [
                            PropsFieldHirDecl {
                                ident: `contour`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                        template_arguments: [
                                            Type(
                                                PathLeading(
                                                    HirTypePathLeading(
                                                        Id {
                                                            value: 31,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ],
                                    },
                                ),
                            },
                            PropsFieldHirDecl {
                                ident: `strokes`,
                                ty: HirType::PathLeading(
                                    HirTypePathLeading {
                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                        template_arguments: [
                                            Type(
                                                PathLeading(
                                                    HirTypePathLeading(
                                                        Id {
                                                            value: 60,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        ],
                                    },
                                ),
                            },
                        ],
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `contour`,
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
                FunctionFnFugitiveHirDefn {
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                Ordinary {
                                    pattern_expr_idx: 1,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 53,
                                            },
                                        ),
                                    ),
                                },
                                Ordinary {
                                    pattern_expr_idx: 2,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 381,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 378,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `u`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `r`,
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
                            51,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 1,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 3,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 2,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            ropd: 4,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 6,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 8,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 7,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            ropd: 9,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 5,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 10,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 11,
                                            ident: `sqrt`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::num`,
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `sqrt`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 13,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 14,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 16,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            ropd: 17,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 19,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            ropd: 20,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 22,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            ropd: 23,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 21,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 24,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 25,
                                            ident: `sqrt`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::num`,
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `sqrt`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 18,
                                            opr: Closed(
                                                Div,
                                            ),
                                            ropd: 26,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 29,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 28,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            ropd: 30,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 31,
                                            opr: Closed(
                                                Div,
                                            ),
                                            ropd: 32,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 34,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 36,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 35,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            ropd: 37,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 38,
                                            opr: Closed(
                                                Div,
                                            ),
                                            ropd: 39,
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 42,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 43,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 44,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 46,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 47,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 48,
                                        },
                                        HirEagerExprData::TypeConstructorFnCall {
                                            path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            function_hir_eager_expr_idx: 41,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    45,
                                                ),
                                                Regular(
                                                    49,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..7,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 12,
                                        },
                                        Assert {
                                            condition: HirEagerCondition(
                                                15,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 27,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 33,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 40,
                                        },
                                        Eval {
                                            expr_idx: 50,
                                            discarded: false,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
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
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 382,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 383,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 384,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `u`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `r`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `L`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dr`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dx`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dy`,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnFugitiveHirDefn {
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                Ordinary {
                                    pattern_expr_idx: 1,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 53,
                                            },
                                        ),
                                    ),
                                },
                                Ordinary {
                                    pattern_expr_idx: 2,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 381,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 378,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `u`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `r`,
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
                            51,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 1,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 3,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 2,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            ropd: 4,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 6,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 8,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 7,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            ropd: 9,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 5,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 10,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 11,
                                            ident: `sqrt`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::num`,
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `sqrt`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 13,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 14,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 16,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            ropd: 17,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 19,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            ropd: 20,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 22,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            ropd: 23,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 21,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 24,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 25,
                                            ident: `sqrt`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::num`,
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `sqrt`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 18,
                                            opr: Closed(
                                                Div,
                                            ),
                                            ropd: 26,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 28,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 30,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 29,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            ropd: 31,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 32,
                                            opr: Closed(
                                                Div,
                                            ),
                                            ropd: 33,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 36,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 35,
                                            opr: Closed(
                                                Mul,
                                            ),
                                            ropd: 37,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 38,
                                            opr: Closed(
                                                Div,
                                            ),
                                            ropd: 39,
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 42,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 43,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 44,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 46,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 47,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 48,
                                        },
                                        HirEagerExprData::TypeConstructorFnCall {
                                            path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                            function_hir_eager_expr_idx: 41,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    45,
                                                ),
                                                Regular(
                                                    49,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..7,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: None,
                                            },
                                            initial_value: 12,
                                        },
                                        Assert {
                                            condition: HirEagerCondition(
                                                15,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 2,
                                                ty: None,
                                            },
                                            initial_value: 27,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 34,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 40,
                                        },
                                        Eval {
                                            expr_idx: 50,
                                            discarded: false,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
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
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 382,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 383,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 384,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `u`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `r`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `L`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dr`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dx`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dy`,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnFugitiveHirDefn {
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                Ordinary {
                                    pattern_expr_idx: 1,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                },
                                Ordinary {
                                    pattern_expr_idx: 2,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                },
                                Ordinary {
                                    pattern_expr_idx: 3,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
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
                                                    value: 150,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 378,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `ct`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `start`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `r`,
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
                            115,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 4,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 5,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 2,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::raw_contour`,
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `displacement`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    3,
                                                ),
                                                Regular(
                                                    6,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 8,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 9,
                                            ident: `ilen`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `ilen`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 11,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 12,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            7,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 14,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            ropd: 15,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 17,
                                            ident: `norm`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `norm`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 18,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 19,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 16,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 20,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 22,
                                            opr: Incr,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 27,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 28,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 25,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::raw_contour`,
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `displacement`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    26,
                                                ),
                                                Regular(
                                                    29,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 24,
                                            opr: Assign,
                                            ropd: 30,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 32,
                                            ident: `norm`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `norm`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 33,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 34,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 37,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    38,
                                                ),
                                                Regular(
                                                    39,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 41,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    42,
                                                ),
                                                Regular(
                                                    43,
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
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            7,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 46,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            ropd: 47,
                                        },
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 49,
                                            ident: `rotation_direction_to`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `rotation_direction_to`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    50,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 51,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            ropd: 52,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 48,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 53,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            9,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 55,
                                            ident: `rotation_direction_to`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `rotation_direction_to`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    56,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 57,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            ropd: 58,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 54,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 59,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 61,
                                            ident: `norm`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `norm`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Variable(
                                            10,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 64,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 65,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 63,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 66,
                                        },
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Variable(
                                            10,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 68,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 69,
                                        },
                                        HirEagerExprData::Variable(
                                            10,
                                        ),
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 71,
                                            opr: Assign,
                                            ropd: 72,
                                        },
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 74,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 75,
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 77,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    78,
                                                ),
                                                Regular(
                                                    79,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 81,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    82,
                                                ),
                                                Regular(
                                                    83,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 85,
                                            ident: `rotation_direction_to`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `rotation_direction_to`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    86,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 87,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 88,
                                        },
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 90,
                                            opr: Assign,
                                            ropd: 91,
                                        },
                                        HirEagerExprData::Variable(
                                            13,
                                        ),
                                        HirEagerExprData::Variable(
                                            9,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 93,
                                            ident: `rotation_direction_to`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `rotation_direction_to`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    94,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 95,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 96,
                                        },
                                        HirEagerExprData::Variable(
                                            9,
                                        ),
                                        HirEagerExprData::Variable(
                                            13,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 98,
                                            opr: Assign,
                                            ropd: 99,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 101,
                                            opr: Incr,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 106,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 107,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 104,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::raw_contour`,
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `displacement`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    105,
                                                ),
                                                Regular(
                                                    108,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 103,
                                            opr: Assign,
                                            ropd: 109,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 111,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 112,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                17..29,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 23,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 31,
                                            discarded: false,
                                        },
                                        Return {
                                            result: 36,
                                        },
                                        Break,
                                        Eval {
                                            expr_idx: 73,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 92,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 100,
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 9,
                                                ty: None,
                                            },
                                            initial_value: 80,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 10,
                                                ty: None,
                                            },
                                            initial_value: 84,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    89,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    6..7,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    97,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    7..8,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 8,
                                                ty: None,
                                            },
                                            initial_value: 62,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    67,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    4..5,
                                                ),
                                            },
                                            elif_branches: [
                                                HirEagerElifBranch {
                                                    condition: HirEagerCondition(
                                                        70,
                                                    ),
                                                    stmts: ArenaIdxRange(
                                                        5..6,
                                                    ),
                                                },
                                            ],
                                            else_branch: None,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    76,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    8..12,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 102,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 110,
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
                                            initial_value: 7,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 10,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 13,
                                        },
                                        While {
                                            condition: HirEagerCondition(
                                                21,
                                            ),
                                            stmts: ArenaIdxRange(
                                                1..3,
                                            ),
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    35,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    3..4,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            initial_value: 40,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 6,
                                                ty: None,
                                            },
                                            initial_value: 44,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 7,
                                                ty: None,
                                            },
                                            initial_value: 45,
                                        },
                                        While {
                                            condition: HirEagerCondition(
                                                60,
                                            ),
                                            stmts: ArenaIdxRange(
                                                12..17,
                                            ),
                                        },
                                        Assert {
                                            condition: HirEagerCondition(
                                                113,
                                            ),
                                        },
                                        Return {
                                            result: 114,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 151,
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
                                                        value: 387,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 304,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 388,
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
                                                        value: 389,
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
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 391,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 392,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 393,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 394,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ct`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `start`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `r`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `end`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dp`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `N`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `max_end`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `right_bound`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `left_bound`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `r_max`,
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
                                                    `dp_right`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dp_left`,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnFugitiveHirDefn {
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                Ordinary {
                                    pattern_expr_idx: 1,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                },
                                Ordinary {
                                    pattern_expr_idx: 2,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                },
                                Ordinary {
                                    pattern_expr_idx: 3,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                },
                                Ordinary {
                                    pattern_expr_idx: 4,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::num::i32`, `Extern`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
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
                                                    value: 396,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 151,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 378,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `ct`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `start0`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `end`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `r`,
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
                            124,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 4,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 5,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 2,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::raw_contour`,
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `displacement`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    3,
                                                ),
                                                Regular(
                                                    6,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 9,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 10,
                                            ident: `ilen`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `ilen`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 8,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 11,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            7,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 13,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            ropd: 14,
                                        },
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 16,
                                            ident: `norm`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `norm`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 17,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 18,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 15,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 19,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 21,
                                            opr: Decr,
                                        },
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 26,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 27,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 24,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::raw_contour`,
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `displacement`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    25,
                                                ),
                                                Regular(
                                                    28,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 23,
                                            opr: Assign,
                                            ropd: 29,
                                        },
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 31,
                                            ident: `norm`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `norm`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 32,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 33,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 35,
                                            ident: `min`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::num`,
                                                        ty_path: TypePath(`core::num::i32`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `min`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    36,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 38,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    39,
                                                ),
                                                Regular(
                                                    40,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 42,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    43,
                                                ),
                                                Regular(
                                                    44,
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
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            7,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 47,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            ropd: 48,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 52,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 53,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 50,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::raw_contour`,
                                                        ty_path: TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `displacement`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    51,
                                                ),
                                                Regular(
                                                    54,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 56,
                                            ident: `norm`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `norm`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::Variable(
                                            10,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 59,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 60,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 58,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 61,
                                        },
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::Variable(
                                            10,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 63,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 64,
                                        },
                                        HirEagerExprData::Variable(
                                            10,
                                        ),
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 66,
                                            opr: Assign,
                                            ropd: 67,
                                        },
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 69,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 70,
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::go_right`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 72,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    73,
                                                ),
                                                Regular(
                                                    74,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::go_left`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 76,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    77,
                                                ),
                                                Regular(
                                                    78,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Variable(
                                            13,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 80,
                                            ident: `rotation_direction_to`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `rotation_direction_to`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    81,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 82,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 83,
                                        },
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Variable(
                                            13,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 85,
                                            opr: Assign,
                                            ropd: 86,
                                        },
                                        HirEagerExprData::Variable(
                                            14,
                                        ),
                                        HirEagerExprData::Variable(
                                            9,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 88,
                                            ident: `rotation_direction_to`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `rotation_direction_to`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    89,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 90,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 91,
                                        },
                                        HirEagerExprData::Variable(
                                            9,
                                        ),
                                        HirEagerExprData::Variable(
                                            14,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 93,
                                            opr: Assign,
                                            ropd: 94,
                                        },
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Variable(
                                            9,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 96,
                                            ident: `rotation_direction_to`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `rotation_direction_to`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    97,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 98,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            ropd: 99,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 101,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            ropd: 102,
                                        },
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 104,
                                            ident: `rotation_direction_to`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `rotation_direction_to`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    105,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 106,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            ropd: 107,
                                        },
                                        HirEagerExprData::Variable(
                                            11,
                                        ),
                                        HirEagerExprData::Variable(
                                            9,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 109,
                                            ident: `rotation_direction_to`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `rotation_direction_to`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    110,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 111,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            ropd: 112,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 108,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 113,
                                        },
                                        HirEagerExprData::Prefix {
                                            opr: Not,
                                            opd_hir_expr_idx: 114,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 103,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 115,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 117,
                                            opr: Decr,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 119,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            ropd: 120,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                23..33,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 22,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 30,
                                            discarded: false,
                                        },
                                        Return {
                                            result: 37,
                                        },
                                        Break,
                                        Eval {
                                            expr_idx: 68,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 87,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 95,
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 9,
                                                ty: None,
                                            },
                                            initial_value: 75,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 10,
                                                ty: None,
                                            },
                                            initial_value: 79,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    84,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    6..7,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    92,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    7..8,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Break,
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    116,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    12..13,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 118,
                                            discarded: false,
                                        },
                                        Break,
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 7,
                                                ty: None,
                                            },
                                            initial_value: 55,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 8,
                                                ty: None,
                                            },
                                            initial_value: 57,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    62,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    4..5,
                                                ),
                                            },
                                            elif_branches: [
                                                HirEagerElifBranch {
                                                    condition: HirEagerCondition(
                                                        65,
                                                    ),
                                                    stmts: ArenaIdxRange(
                                                        5..6,
                                                    ),
                                                },
                                            ],
                                            else_branch: None,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    71,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    8..12,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    100,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    13..15,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                HirEagerElseBranch {
                                                    stmts: ArenaIdxRange(
                                                        15..16,
                                                    ),
                                                },
                                            ),
                                        },
                                        Return {
                                            result: 122,
                                        },
                                        Return {
                                            result: 123,
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
                                            initial_value: 7,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 12,
                                        },
                                        While {
                                            condition: HirEagerCondition(
                                                20,
                                            ),
                                            stmts: ArenaIdxRange(
                                                1..3,
                                            ),
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    34,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    3..4,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 41,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            initial_value: 45,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 6,
                                                ty: None,
                                            },
                                            initial_value: 46,
                                        },
                                        While {
                                            condition: HirEagerCondition(
                                                49,
                                            ),
                                            stmts: ArenaIdxRange(
                                                16..21,
                                            ),
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    121,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    21..22,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                HirEagerElseBranch {
                                                    stmts: ArenaIdxRange(
                                                        22..23,
                                                    ),
                                                },
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 150,
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
                                                        value: 397,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 398,
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
                                                        value: 389,
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
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 391,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 387,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 392,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 393,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 394,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ct`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `start0`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `end`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `r`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `start`,
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
                                                    `min_start`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `right_bound`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `left_bound`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `r_max`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dp`,
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
                                                    `dp_right`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dp_left`,
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
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::FunctionFn(
                FunctionFnFugitiveHirDefn {
                    path: FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                Ordinary {
                                    pattern_expr_idx: 1,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                },
                                Ordinary {
                                    pattern_expr_idx: 2,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                template_arguments: [
                                    Type(
                                        PathLeading(
                                            HirTypePathLeading(
                                                Id {
                                                    value: 60,
                                                },
                                            ),
                                        ),
                                    ),
                                ],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `FunctionFn`),
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
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
                                                    value: 378,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `ct`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `r`,
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
                            192,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::NewList {
                                            items: [],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 4,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 5,
                                            ident: `ilen`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `ilen`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 7,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            ropd: 8,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_end`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 11,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    12,
                                                ),
                                                Regular(
                                                    13,
                                                ),
                                                Regular(
                                                    14,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 10,
                                            opr: Assign,
                                            ropd: 15,
                                        },
                                        HirEagerExprData::AssociatedFn {
                                            associated_item_path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `new`,
                                                    item_kind: AssociatedFunctionFn,
                                                },
                                            ),
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::AssociatedFunctionFnCall {
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `new`,
                                                    item_kind: AssociatedFunctionFn,
                                                },
                                            ),
                                            function_hir_eager_expr_idx: 17,
                                            parent_template_arguments: None,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    18,
                                                ),
                                                Regular(
                                                    19,
                                                ),
                                                Regular(
                                                    20,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::Bool(
                                                true,
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 23,
                                            ident: `ilen`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `ilen`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 24,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 25,
                                        },
                                        HirEagerExprData::Variable(
                                            7,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 27,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `displacement`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 29,
                                            ident: `last`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `last`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 30,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 31,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `displacement`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            9,
                                        ),
                                        HirEagerExprData::Variable(
                                            10,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 33,
                                            ident: `cross`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `cross`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    34,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 35,
                                            ident: `abs`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::num`,
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `abs`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::F32(
                                                NotNan(
                                                    0.01,
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 36,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 37,
                                        },
                                        HirEagerExprData::Variable(
                                            9,
                                        ),
                                        HirEagerExprData::Variable(
                                            10,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 39,
                                            ident: `dot`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `dot`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    40,
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
                                            lopd: 41,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 42,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 38,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 43,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 45,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 46,
                                            ident: `ilen`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `ilen`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 48,
                                            ident: `last`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `last`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 49,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::AssociatedFn {
                                            associated_item_path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `new`,
                                                    item_kind: AssociatedFunctionFn,
                                                },
                                            ),
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 53,
                                            ident: `last`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `last`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 54,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 55,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 56,
                                            ident: `start`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::slice`,
                                                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `start`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::AssociatedFunctionFnCall {
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `new`,
                                                    item_kind: AssociatedFunctionFn,
                                                },
                                            ),
                                            function_hir_eager_expr_idx: 51,
                                            parent_template_arguments: None,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    52,
                                                ),
                                                Regular(
                                                    57,
                                                ),
                                                Regular(
                                                    58,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 50,
                                            opr: Assign,
                                            ropd: 59,
                                        },
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::Bool(
                                                false,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 61,
                                            opr: Assign,
                                            ropd: 62,
                                        },
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::extend_start`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 66,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    67,
                                                ),
                                                Regular(
                                                    68,
                                                ),
                                                Regular(
                                                    69,
                                                ),
                                                Regular(
                                                    70,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 65,
                                            opr: Assign,
                                            ropd: 71,
                                        },
                                        HirEagerExprData::AssociatedFn {
                                            associated_item_path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `new`,
                                                    item_kind: AssociatedFunctionFn,
                                                },
                                            ),
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::AssociatedFunctionFnCall {
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `new`,
                                                    item_kind: AssociatedFunctionFn,
                                                },
                                            ),
                                            function_hir_eager_expr_idx: 73,
                                            parent_template_arguments: None,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    74,
                                                ),
                                                Regular(
                                                    75,
                                                ),
                                                Regular(
                                                    76,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 78,
                                            ident: `ilen`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `ilen`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                0,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 79,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 80,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 82,
                                            ident: `last`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `last`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 83,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::Variable(
                                            13,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 85,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `displacement`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 87,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `displacement`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            13,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 89,
                                            ident: `start`,
                                        },
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 91,
                                            ident: `end`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 90,
                                            ident: `to`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `to`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    92,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            15,
                                        ),
                                        HirEagerExprData::Variable(
                                            14,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 94,
                                            ident: `cross`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `cross`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    95,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 96,
                                            ident: `abs`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::num`,
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `abs`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::F32(
                                                NotNan(
                                                    0.001,
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 97,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 98,
                                        },
                                        HirEagerExprData::Variable(
                                            15,
                                        ),
                                        HirEagerExprData::Variable(
                                            14,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 100,
                                            ident: `dot`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `dot`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    101,
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
                                            lopd: 102,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 103,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 99,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 104,
                                        },
                                        HirEagerExprData::Variable(
                                            15,
                                        ),
                                        HirEagerExprData::Variable(
                                            16,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 106,
                                            ident: `cross`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `cross`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    107,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 108,
                                            ident: `abs`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::num`,
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `abs`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::F32(
                                                NotNan(
                                                    0.001,
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 109,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 110,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 105,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 111,
                                        },
                                        HirEagerExprData::Variable(
                                            15,
                                        ),
                                        HirEagerExprData::Variable(
                                            16,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 113,
                                            ident: `dot`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `dot`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    114,
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
                                            lopd: 115,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 116,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 112,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 117,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 119,
                                            ident: `pop`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `pop`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 120,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::AssociatedFn {
                                            associated_item_path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `new`,
                                                    item_kind: AssociatedFunctionFn,
                                                },
                                            ),
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            17,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 125,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 126,
                                            ident: `start`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::slice`,
                                                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `start`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 128,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 129,
                                            ident: `end`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::slice`,
                                                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `end`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::AssociatedFunctionFnCall {
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `new`,
                                                    item_kind: AssociatedFunctionFn,
                                                },
                                            ),
                                            function_hir_eager_expr_idx: 123,
                                            parent_template_arguments: None,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    124,
                                                ),
                                                Regular(
                                                    127,
                                                ),
                                                Regular(
                                                    130,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 122,
                                            opr: Assign,
                                            ropd: 131,
                                        },
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 135,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 136,
                                            ident: `ilen`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `ilen`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 134,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 137,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 133,
                                            opr: Assign,
                                            ropd: 138,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            12,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 140,
                                            ident: `push`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `push`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    141,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 143,
                                            opr: Assign,
                                            ropd: 144,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 147,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 148,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 146,
                                            opr: Assign,
                                            ropd: 149,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 151,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 152,
                                            ident: `ilen`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `ilen`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 154,
                                            ident: `first`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `first`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 155,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 156,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 157,
                                            ident: `end`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::slice`,
                                                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `end`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 159,
                                            ident: `last`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `last`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 160,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::Variable(
                                            20,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 162,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 163,
                                            ident: `end`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::slice`,
                                                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `end`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            19,
                                        ),
                                        HirEagerExprData::Variable(
                                            18,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 165,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 166,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 164,
                                            opr: Comparison(
                                                Geq,
                                            ),
                                            ropd: 167,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 169,
                                            ident: `pop`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `pop`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 170,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 172,
                                            ident: `first`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `first`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 173,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::AssociatedFn {
                                            associated_item_path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `new`,
                                                    item_kind: AssociatedFunctionFn,
                                                },
                                            ),
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            21,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 177,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 178,
                                            ident: `start`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::slice`,
                                                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `start`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            18,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 179,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 180,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 182,
                                            ident: `first`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `first`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 183,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 184,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 185,
                                            ident: `end`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::slice`,
                                                        ty_path: TypePath(`core::slice::CyclicSlice`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `end`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 186,
                                            opr: Closed(
                                                Sub,
                                            ),
                                            ropd: 187,
                                        },
                                        HirEagerExprData::AssociatedFunctionFnCall {
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::line_segment_sketch`,
                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `new`,
                                                    item_kind: AssociatedFunctionFn,
                                                },
                                            ),
                                            function_hir_eager_expr_idx: 175,
                                            parent_template_arguments: None,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    176,
                                                ),
                                                Regular(
                                                    181,
                                                ),
                                                Regular(
                                                    188,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 174,
                                            opr: Assign,
                                            ropd: 189,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                28..38,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 9,
                                                ty: None,
                                            },
                                            initial_value: 47,
                                        },
                                        Eval {
                                            expr_idx: 60,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 63,
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 7,
                                                ty: None,
                                            },
                                            initial_value: 28,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 8,
                                                ty: None,
                                            },
                                            initial_value: 32,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    44,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    1..4,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 15,
                                                ty: None,
                                            },
                                            initial_value: 121,
                                        },
                                        Eval {
                                            expr_idx: 132,
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 11,
                                                ty: None,
                                            },
                                            initial_value: 84,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 12,
                                                ty: None,
                                            },
                                            initial_value: 86,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 13,
                                                ty: None,
                                            },
                                            initial_value: 88,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 14,
                                                ty: None,
                                            },
                                            initial_value: 93,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    118,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    7..9,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 139,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 72,
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 10,
                                                ty: None,
                                            },
                                            initial_value: 77,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    81,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    9..14,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: Some(
                                                HirEagerElseBranch {
                                                    stmts: ArenaIdxRange(
                                                        14..15,
                                                    ),
                                                },
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 142,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 16,
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 5,
                                                ty: None,
                                            },
                                            initial_value: 21,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 6,
                                                ty: None,
                                            },
                                            initial_value: 22,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    26,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    4..7,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    64,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    15..19,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 145,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 150,
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 19,
                                                ty: None,
                                            },
                                            initial_value: 171,
                                        },
                                        Eval {
                                            expr_idx: 190,
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 67,
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
                                            initial_value: 2,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 3,
                                                ty: None,
                                            },
                                            initial_value: 3,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 4,
                                                ty: None,
                                            },
                                            initial_value: 6,
                                        },
                                        While {
                                            condition: HirEagerCondition(
                                                9,
                                            ),
                                            stmts: ArenaIdxRange(
                                                19..26,
                                            ),
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 16,
                                                ty: None,
                                            },
                                            initial_value: 153,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 17,
                                                ty: None,
                                            },
                                            initial_value: 158,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 18,
                                                ty: None,
                                            },
                                            initial_value: 161,
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    168,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    26..28,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 191,
                                            discarded: false,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 399,
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
                                                        value: 150,
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
                                                        value: 151,
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
                                                        value: 388,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 400,
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
                                                        value: 401,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 402,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
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
                                                        value: 304,
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
                                                        value: 404,
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
                                                        value: 387,
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
                                                        value: 304,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 408,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 409,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 409,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ct`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `r`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `line_segments`,
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
                                                    `max_end`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ls_extend_end`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `extend_start_flag`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dp_extend_end`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dp_previous`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `N`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ls`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ls_last`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dp_last`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dp`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `dp1`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ls_last`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `N`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `first_line_segment_points_end`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `last_line_segment`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::LetVariable,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `last_line_segment`,
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
                    path: TraitForTypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        ),
                        disambiguator: 0,
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
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TraitForTypeImplBlock(
                                    TraitForTypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        ),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        ),
                        hir_eager_expr_arena: Arena {
                            data: [],
                        },
                        hir_eager_stmt_arena: Arena {
                            data: [],
                        },
                        hir_eager_pattern_expr_arena: Arena {
                            data: [],
                        },
                        hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                    path: TraitForTypeItemPath {
                        impl_block: TraitForTypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                        ident: `visualize`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TraitForTypeMethodFnHirDecl {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                            ident: `visualize`,
                            item_kind: MethodFn,
                        },
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
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath {
                                            impl_block: TraitForTypeImplBlockPath {
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                ),
                                                disambiguator: 0,
                                            },
                                            ident: `visualize`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                                        AssociatedItemPath::TraitForTypeItem(
                                            TraitForTypeItemPath {
                                                impl_block: TraitForTypeImplBlockPath {
                                                    module_path: `mnist_classifier::line_segment_sketch`,
                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                                ident: `visualize`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 1,
                                            ident: `start`,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 3,
                                            ident: `end`,
                                        },
                                        HirEagerExprData::EmptyHtmlTag {
                                            function_ident: `LineSegment`,
                                            arguments: [
                                                HirEagerHtmlArgumentExpr {
                                                    property_ident: Ident(
                                                        Coword(
                                                            Id {
                                                                value: 150,
                                                            },
                                                        ),
                                                    ),
                                                    expr: 2,
                                                },
                                                HirEagerHtmlArgumentExpr {
                                                    property_ident: Ident(
                                                        Coword(
                                                            Id {
                                                                value: 151,
                                                            },
                                                        ),
                                                    ),
                                                    expr: 4,
                                                },
                                            ],
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 5,
                                            discarded: false,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [],
                                },
                                hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                    path: TypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                        disambiguator: 0,
                    },
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TypeImplBlock(
                                    TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        ),
                        hir_eager_expr_arena: Arena {
                            data: [],
                        },
                        hir_eager_stmt_arena: Arena {
                            data: [],
                        },
                        hir_eager_pattern_expr_arena: Arena {
                            data: [],
                        },
                        hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `new`,
                        item_kind: AssociatedFunctionFn,
                    },
                    hir_decl: TypeAssociatedFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `new`,
                            item_kind: AssociatedFunctionFn,
                        },
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                Ordinary {
                                    pattern_expr_idx: 1,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                },
                                Ordinary {
                                    pattern_expr_idx: 2,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                },
                                Ordinary {
                                    pattern_expr_idx: 3,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 6,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `new`,
                                            item_kind: AssociatedFunctionFn,
                                        },
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
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
                                                    value: 373,
                                                },
                                            ),
                                        ),
                                    },
                                    Ident {
                                        symbol_modifier: None,
                                        ident: Ident(
                                            Coword(
                                                Id {
                                                    value: 307,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `ct`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `from`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `to`,
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
                            13,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath {
                                                impl_block: TypeImplBlockPath {
                                                    module_path: `mnist_classifier::line_segment_sketch`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `new`,
                                                item_kind: AssociatedFunctionFn,
                                            },
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 1,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            ropd: 2,
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 5,
                                            ident: `points`,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::I32(
                                                1,
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 8,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 9,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 6,
                                            ident: `cyclic_slice_leashed`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `cyclic_slice_leashed`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    7,
                                                ),
                                                Regular(
                                                    10,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::TypeConstructorFnCall {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                            function_hir_eager_expr_idx: 4,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    11,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..3,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Assert {
                                            condition: HirEagerCondition(
                                                3,
                                            ),
                                        },
                                        Eval {
                                            expr_idx: 12,
                                            discarded: false,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [],
                                },
                                hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ct`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `from`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `to`,
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
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `displacement`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `displacement`,
                            item_kind: MethodFn,
                        },
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
                                        TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `displacement`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                                            TypeItemPath {
                                                impl_block: TypeImplBlockPath {
                                                    module_path: `mnist_classifier::line_segment_sketch`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `displacement`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 1,
                                            ident: `start`,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 3,
                                            ident: `end`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 2,
                                            ident: `to`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `mnist_classifier::geom2d`,
                                                        ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `to`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    4,
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
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 5,
                                            discarded: false,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [],
                                },
                                hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
        ImplBlockHirDefn::TraitForType(
            TraitForTypeImplBlockHirDefn {
                hir_decl: TraitForTypeImplBlockHirDecl {
                    path: TraitForTypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        trai_path: TraitPath(`core::visual::Visualize`),
                        ty_sketch: TypeSketch::Path(
                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        ),
                        disambiguator: 0,
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
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TraitForTypeImplBlock(
                                    TraitForTypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch`,
                                        trai_path: TraitPath(`core::visual::Visualize`),
                                        ty_sketch: TypeSketch::Path(
                                            TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        ),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        ),
                        hir_eager_expr_arena: Arena {
                            data: [],
                        },
                        hir_eager_stmt_arena: Arena {
                            data: [],
                        },
                        hir_eager_pattern_expr_arena: Arena {
                            data: [],
                        },
                        hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                    path: TraitForTypeItemPath {
                        impl_block: TraitForTypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            trai_path: TraitPath(`core::visual::Visualize`),
                            ty_sketch: TypeSketch::Path(
                                TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            ),
                            disambiguator: 0,
                        },
                        ident: `visualize`,
                        item_kind: MethodFn,
                    },
                    hir_decl: TraitForTypeMethodFnHirDecl {
                        path: TraitForTypeItemPath {
                            impl_block: TraitForTypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                trai_path: TraitPath(`core::visual::Visualize`),
                                ty_sketch: TypeSketch::Path(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ),
                                disambiguator: 0,
                            },
                            ident: `visualize`,
                            item_kind: MethodFn,
                        },
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
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath {
                                            impl_block: TraitForTypeImplBlockPath {
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                trai_path: TraitPath(`core::visual::Visualize`),
                                                ty_sketch: TypeSketch::Path(
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                ),
                                                disambiguator: 0,
                                            },
                                            ident: `visualize`,
                                            item_kind: MethodFn,
                                        },
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                                            TraitForTypeItemPath {
                                                impl_block: TraitForTypeImplBlockPath {
                                                    module_path: `mnist_classifier::line_segment_sketch`,
                                                    trai_path: TraitPath(`core::visual::Visualize`),
                                                    ty_sketch: TypeSketch::Path(
                                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    ),
                                                    disambiguator: 0,
                                                },
                                                ident: `visualize`,
                                                item_kind: MethodFn,
                                            },
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 1,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 2,
                                            ident: `visualize`,
                                            path: AssociatedItemPath::TraitForTypeItem(
                                                TraitForTypeItemPath {
                                                    impl_block: TraitForTypeImplBlockPath {
                                                        module_path: `core::visual`,
                                                        trai_path: TraitPath(`core::visual::Visualize`),
                                                        ty_sketch: TypeSketch::DeriveAny,
                                                        disambiguator: 0,
                                                    },
                                                    ident: `visualize`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                1..2,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 3,
                                            discarded: false,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [],
                                },
                                hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                    path: TypeImplBlockPath {
                        module_path: `mnist_classifier::line_segment_sketch`,
                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                        disambiguator: 0,
                    },
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TypeImplBlock(
                                    TypeImplBlockPath {
                                        module_path: `mnist_classifier::line_segment_sketch`,
                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                        disambiguator: 0,
                                    },
                                ),
                            ),
                        ),
                        hir_eager_expr_arena: Arena {
                            data: [],
                        },
                        hir_eager_stmt_arena: Arena {
                            data: [],
                        },
                        hir_eager_pattern_expr_arena: Arena {
                            data: [],
                        },
                        hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `concave_components`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `concave_components`,
                            item_kind: MemoizedField,
                        },
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                template_arguments: [
                                    Type(
                                        PathLeading(
                                            HirTypePathLeading(
                                                Id {
                                                    value: 42,
                                                },
                                            ),
                                        ),
                                    ),
                                ],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `concave_components`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                                            TypeItemPath {
                                                impl_block: TypeImplBlockPath {
                                                    module_path: `mnist_classifier::line_segment_sketch`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `concave_components`,
                                                item_kind: MemoizedField,
                                            },
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 1,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    2,
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
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 3,
                                            discarded: false,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [],
                                },
                                hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `bounding_box`,
                        item_kind: MemoizedField,
                    },
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `bounding_box`,
                            item_kind: MemoizedField,
                        },
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `bounding_box`,
                                            item_kind: MemoizedField,
                                        },
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
                            56,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath {
                                                impl_block: TypeImplBlockPath {
                                                    module_path: `mnist_classifier::line_segment_sketch`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `bounding_box`,
                                                item_kind: MemoizedField,
                                            },
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 1,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::USize(
                                                TermUSizeLiteral {
                                                    value: 0,
                                                },
                                            ),
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 2,
                                            items: [
                                                3,
                                            ],
                                        },
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 4,
                                            ident: `start`,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 6,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 8,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 10,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 12,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 14,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 15,
                                            ident: `ilen`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::vec`,
                                                        ty_path: TypePath(`core::vec::Vec`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `ilen`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 17,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::Variable(
                                            7,
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 18,
                                            items: [
                                                19,
                                            ],
                                        },
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 20,
                                            ident: `end`,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 24,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 23,
                                            ident: `min`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::num`,
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `min`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    25,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 22,
                                            opr: Assign,
                                            ropd: 26,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 30,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 29,
                                            ident: `max`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::num`,
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `max`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    31,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 28,
                                            opr: Assign,
                                            ropd: 32,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 36,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 35,
                                            ident: `min`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::num`,
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `min`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    37,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 34,
                                            opr: Assign,
                                            ropd: 38,
                                        },
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Field {
                                            owner_hir_expr_idx: 42,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 41,
                                            ident: `max`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath {
                                                    impl_block: TypeImplBlockPath {
                                                        module_path: `core::num`,
                                                        ty_path: TypePath(`core::num::f32`, `Extern`),
                                                        disambiguator: 0,
                                                    },
                                                    ident: `max`,
                                                    item_kind: MethodFn,
                                                },
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    43,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 40,
                                            opr: Assign,
                                            ropd: 44,
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::TypeConstructorFnCall {
                                            path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                            function_hir_eager_expr_idx: 47,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    48,
                                                ),
                                                Regular(
                                                    49,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::TypeConstructorFnCall {
                                            path: TypePath(`mnist_classifier::geom2d::ClosedRange`, `Struct`),
                                            function_hir_eager_expr_idx: 51,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    52,
                                                ),
                                                Regular(
                                                    53,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::TypeConstructorFnCall {
                                            path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            function_hir_eager_expr_idx: 46,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    50,
                                                ),
                                                Regular(
                                                    54,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                6..13,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 6,
                                                ty: None,
                                            },
                                            initial_value: 21,
                                        },
                                        Eval {
                                            expr_idx: 27,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 33,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 39,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 45,
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
                                                            value: 260,
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
                                                            16,
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
                                            result: 55,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 293,
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
                                                        value: 294,
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
                                                        value: 295,
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
                                                        value: 296,
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
                                                        value: 297,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 298,
                                                    },
                                                ),
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
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
            TypeItemHirDefn::AssociatedFn(
                TypeAssociatedFnHirDefn {
                    path: TypeItemPath {
                        impl_block: TypeImplBlockPath {
                            module_path: `mnist_classifier::line_segment_sketch`,
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                            disambiguator: 0,
                        },
                        ident: `new`,
                        item_kind: AssociatedFunctionFn,
                    },
                    hir_decl: TypeAssociatedFnHirDecl {
                        path: TypeItemPath {
                            impl_block: TypeImplBlockPath {
                                module_path: `mnist_classifier::line_segment_sketch`,
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                disambiguator: 0,
                            },
                            ident: `new`,
                            item_kind: AssociatedFunctionFn,
                        },
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                Ordinary {
                                    pattern_expr_idx: 1,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 32,
                                            },
                                        ),
                                    ),
                                },
                                Ordinary {
                                    pattern_expr_idx: 2,
                                    ty: PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 15,
                                            },
                                        ),
                                    ),
                                },
                            ],
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath {
                                            impl_block: TypeImplBlockPath {
                                                module_path: `mnist_classifier::line_segment_sketch`,
                                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                disambiguator: 0,
                                            },
                                            ident: `new`,
                                            item_kind: AssociatedFunctionFn,
                                        },
                                    ),
                                ),
                            ),
                            hir_eager_expr_arena: Arena {
                                data: [],
                            },
                            hir_eager_stmt_arena: Arena {
                                data: [],
                            },
                            hir_eager_pattern_expr_arena: Arena {
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
                                                    value: 378,
                                                },
                                            ),
                                        ),
                                    },
                                ],
                            },
                            hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                arena: Arena {
                                    data: [],
                                },
                            },
                            hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                arena: Arena {
                                    data: [
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::SelfValue,
                                            data: HirEagerRuntimeSymbolData::SelfValue,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `ct`,
                                            ),
                                            data: HirEagerRuntimeSymbolData::ParenateParameter,
                                        },
                                        HirEagerRuntimeSymbolEntry {
                                            name: HirEagerRuntimeSymbolName::Ident(
                                                `r`,
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
                            8,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath {
                                                impl_block: TypeImplBlockPath {
                                                    module_path: `mnist_classifier::line_segment_sketch`,
                                                    ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                    disambiguator: 0,
                                                },
                                                ident: `new`,
                                                item_kind: AssociatedFunctionFn,
                                            },
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::find_line_segments`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 3,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    4,
                                                ),
                                                Regular(
                                                    5,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::TypeConstructorFnCall {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                            function_hir_eager_expr_idx: 1,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    2,
                                                ),
                                                Regular(
                                                    6,
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
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 7,
                                            discarded: false,
                                        },
                                    ],
                                },
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [],
                                },
                                hir_eager_comptime_symbol_region_data: HirEagerComptimeSymbolRegionData {
                                    arena: Arena {
                                        data: [],
                                    },
                                },
                                hir_eager_runtime_symbol_region_data: HirEagerRuntimeSymbolRegionData {
                                    arena: Arena {
                                        data: [
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `ct`,
                                                ),
                                                data: HirEagerRuntimeSymbolData::ParenateParameter,
                                            },
                                            HirEagerRuntimeSymbolEntry {
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `r`,
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
]