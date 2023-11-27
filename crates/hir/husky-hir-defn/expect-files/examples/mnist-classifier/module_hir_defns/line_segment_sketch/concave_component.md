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
                                                    },
                                                ),
                                            ),
                                        ],
                                    },
                                ),
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
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                    },
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
                                        TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
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
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                                            template_arguments: [],
                                                        },
                                                    ),
                                                ),
                                            ],
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
                                            },
                                        ),
                                    ),
                                ],
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
                                                    value: 220,
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
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::NewList {
                                            items: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 2,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 3,
                                            ident: `ilen`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 203,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
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
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Prefix {
                                            opr: Minus,
                                            opd_hir_expr_idx: 8,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 7,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 9,
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 11,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    12,
                                                ),
                                                Regular(
                                                    13,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Prefix {
                                            opr: Not,
                                            opd_hir_expr_idx: 14,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 10,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 15,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 17,
                                            opr: Decr,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 21,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 22,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 20,
                                            opr: Comparison(
                                                Less,
                                            ),
                                            ropd: 23,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 26,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 27,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 25,
                                            opr: Comparison(
                                                Leq,
                                            ),
                                            ropd: 28,
                                        },
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Fugitive(
                                                    FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::FunctionFnCall {
                                            path: FugitivePath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `FunctionFn`),
                                            function_hir_eager_expr_idx: 30,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    31,
                                                ),
                                                Regular(
                                                    32,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Prefix {
                                            opr: Not,
                                            opd_hir_expr_idx: 33,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 29,
                                            opr: ShortCircuitLogic(
                                                And,
                                            ),
                                            ropd: 34,
                                        },
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 36,
                                            opr: Incr,
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
                                            lopd: 39,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 40,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 38,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 41,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 46,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 47,
                                            ident: `cyclic_slice_leashed`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 209,
                                                        },
                                                    ),
                                                ),
                                            ),
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
                                        HirEagerExprData::TypeConstructorFnCall {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            function_hir_eager_expr_idx: 44,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    45,
                                                ),
                                                Regular(
                                                    50,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 43,
                                            ident: `push`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 204,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    51,
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
                                            lopd: 53,
                                            opr: Assign,
                                            ropd: 54,
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
                                            lopd: 57,
                                            opr: Closed(
                                                Add,
                                            ),
                                            ropd: 58,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 56,
                                            opr: Assign,
                                            ropd: 59,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                8..16,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 18,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 37,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 52,
                                            discarded: false,
                                        },
                                        While {
                                            condition: HirEagerCondition(
                                                35,
                                            ),
                                            stmts: ArenaIdxRange(
                                                2..3,
                                            ),
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    42,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    3..4,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 55,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 60,
                                            discarded: false,
                                        },
                                        Let {
                                            pattern: HirEagerLetVariablesPattern {
                                                pattern_expr_idx: 1,
                                                ty: Some(
                                                    PathLeading(
                                                        HirTypePathLeading(
                                                            Id {
                                                                value: 52,
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
                                            condition: HirEagerCondition(
                                                16,
                                            ),
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
                                            condition: HirEagerCondition(
                                                24,
                                            ),
                                            stmts: ArenaIdxRange(
                                                4..8,
                                            ),
                                        },
                                        Return {
                                            result: 61,
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
                                                        value: 375,
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
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 419,
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
                    path: TraitForTypeItemPath(
                        ItemPathId(
                            Id {
                                value: 432,
                            },
                        ),
                    ),
                    hir_decl: TraitForTypeMethodFnHirDecl {
                        path: TraitForTypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 432,
                                },
                            ),
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
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TraitForTypeItem(
                                        TraitForTypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 432,
                                                },
                                            ),
                                        ),
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
                                            TraitForTypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 432,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 1,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 2,
                                            ident: `visualize`,
                                            path: AssociatedItemPath::TraitForTypeItem(
                                                TraitForTypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 211,
                                                        },
                                                    ),
                                                ),
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
                    path: TypeImplBlockPath(
                        ItemPathId(
                            Id {
                                value: 361,
                            },
                        ),
                    ),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    self_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                            template_arguments: [],
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        path: RegionPath::Decl(
                            ItemPath::ImplBlock(
                                ImplBlockPath::TypeImplBlock(
                                    TypeImplBlockPath(
                                        ItemPathId(
                                            Id {
                                                value: 361,
                                            },
                                        ),
                                    ),
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
                    path: TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 386,
                            },
                        ),
                    ),
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 386,
                                },
                            ),
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
                                        TypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 386,
                                                },
                                            ),
                                        ),
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
                            3,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 386,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::MemoizedField {
                                            owner_hir_expr_idx: 1,
                                            ident: `hausdorff_norm`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 388,
                                                        },
                                                    ),
                                                ),
                                            ),
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
                                            expr_idx: 2,
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
                    path: TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 387,
                            },
                        ),
                    ),
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 387,
                                },
                            ),
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
                                        TypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 387,
                                                },
                                            ),
                                        ),
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
                            7,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 387,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::MemoizedField {
                                            owner_hir_expr_idx: 1,
                                            ident: `norm`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 386,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 3,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 395,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 4,
                                            ident: `norm`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 410,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 2,
                                            opr: Closed(
                                                Div,
                                            ),
                                            ropd: 5,
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
                                            expr_idx: 6,
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
                    path: TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 388,
                            },
                        ),
                    ),
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 388,
                                },
                            ),
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
                                        TypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 388,
                                                },
                                            ),
                                        ),
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
                            33,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 388,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Literal(
                                            TermLiteral::F32(
                                                NotNan(
                                                    0.0,
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 2,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 3,
                                            ident: `first`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 201,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 4,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 5,
                                            ident: `start`,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 7,
                                            ident: `line_segment`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 392,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 9,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 434,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 10,
                                            ident: `norm`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 410,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 12,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 13,
                                            ident: `start`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 199,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 15,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 16,
                                            ident: `end`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 200,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 18,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::Variable(
                                            6,
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 19,
                                            items: [
                                                20,
                                            ],
                                        },
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 21,
                                            ident: `end`,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Variable(
                                            7,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 23,
                                            ident: `dist_to_point`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 435,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    24,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 26,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 27,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            8,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 29,
                                            opr: Assign,
                                            ropd: 30,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                5..11,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
                                    data: [
                                        Eval {
                                            expr_idx: 31,
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
                                                condition: HirEagerCondition(
                                                    28,
                                                ),
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
                                                            value: 260,
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
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 410,
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
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 413,
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
                                                        value: 298,
                                                    },
                                                ),
                                            ),
                                        },
                                        Ident {
                                            symbol_modifier: None,
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 414,
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
                    path: TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 389,
                            },
                        ),
                    ),
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 389,
                                },
                            ),
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
                                        TypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 389,
                                                },
                                            ),
                                        ),
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
                            30,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 389,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Literal(
                                            TermLiteral::F32(
                                                NotNan(
                                                    0.0,
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 2,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 4,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 5,
                                            ident: `start`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 199,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 3,
                                            items: [
                                                6,
                                            ],
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 7,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 407,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 9,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 10,
                                            ident: `start`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 199,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 12,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 13,
                                            ident: `end`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 200,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 15,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::Variable(
                                            4,
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 16,
                                            items: [
                                                17,
                                            ],
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 18,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 407,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Literal(
                                            TermLiteral::Bool(
                                                true,
                                            ),
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 21,
                                            ident: `angle_to`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 415,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    22,
                                                ),
                                                Regular(
                                                    23,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 20,
                                            opr: AssignClosed(
                                                Add,
                                            ),
                                            ropd: 24,
                                        },
                                        HirEagerExprData::Variable(
                                            3,
                                        ),
                                        HirEagerExprData::Variable(
                                            5,
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 26,
                                            opr: Assign,
                                            ropd: 27,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                4..8,
                                            ),
                                        },
                                    ],
                                },
                                hir_eager_stmt_arena: Arena {
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
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 28,
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
                                                            value: 260,
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
                                hir_eager_pattern_expr_arena: Arena {
                                    data: [
                                        Ident {
                                            symbol_modifier: Some(
                                                Mut,
                                            ),
                                            ident: Ident(
                                                Coword(
                                                    Id {
                                                        value: 349,
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
                                                        value: 387,
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
                    path: TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 390,
                            },
                        ),
                    ),
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 390,
                                },
                            ),
                        ),
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
                                        TypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 390,
                                                },
                                            ),
                                        ),
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
                            59,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 390,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 1,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 2,
                                            ident: `first`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 201,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 3,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 4,
                                            ident: `start`,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 6,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 8,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 10,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Variable(
                                            2,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 12,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 14,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 15,
                                            ident: `start`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 199,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 17,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 18,
                                            ident: `end`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 200,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 20,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::Variable(
                                            7,
                                        ),
                                        HirEagerExprData::Index {
                                            owner_hir_expr_idx: 21,
                                            items: [
                                                22,
                                            ],
                                        },
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 23,
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
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 27,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 26,
                                            ident: `min`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 151,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    28,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 25,
                                            opr: Assign,
                                            ropd: 29,
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
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 33,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 32,
                                            ident: `max`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 150,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    34,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 31,
                                            opr: Assign,
                                            ropd: 35,
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
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 39,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 38,
                                            ident: `min`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 151,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    40,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 37,
                                            opr: Assign,
                                            ropd: 41,
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
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 45,
                                            ident: `y`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 44,
                                            ident: `max`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 150,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    46,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 43,
                                            opr: Assign,
                                            ropd: 47,
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
                                            function_hir_eager_expr_idx: 50,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    51,
                                                ),
                                                Regular(
                                                    52,
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
                                            function_hir_eager_expr_idx: 54,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    55,
                                                ),
                                                Regular(
                                                    56,
                                                ),
                                            ],
                                        },
                                        HirEagerExprData::TypeConstructorFnCall {
                                            path: TypePath(`mnist_classifier::geom2d::BoundingBox`, `Struct`),
                                            function_hir_eager_expr_idx: 49,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    53,
                                                ),
                                                Regular(
                                                    57,
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
                                            initial_value: 24,
                                        },
                                        Eval {
                                            expr_idx: 30,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 36,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 42,
                                            discarded: false,
                                        },
                                        Eval {
                                            expr_idx: 48,
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
            TypeItemHirDefn::MemoizedField(
                TypeMemoizedFieldHirDefn {
                    path: TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 391,
                            },
                        ),
                    ),
                    hir_decl: TypeMemoizedFieldHirDecl {
                        path: TypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 391,
                                },
                            ),
                        ),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::geom2d::RelativeBoundingBox`, `Struct`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 391,
                                                },
                                            ),
                                        ),
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
                            7,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 391,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 1,
                                            ident: `line_segment_sketch`,
                                        },
                                        HirEagerExprData::MemoizedField {
                                            owner_hir_expr_idx: 2,
                                            ident: `bounding_box`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 417,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::MemoizedField {
                                            owner_hir_expr_idx: 4,
                                            ident: `bounding_box`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 390,
                                                        },
                                                    ),
                                                ),
                                            ),
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 3,
                                            ident: `relative_bounding_box`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 419,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    5,
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
                                            expr_idx: 6,
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
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 392,
                            },
                        ),
                    ),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 392,
                                },
                            ),
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
                                ty_path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 392,
                                                },
                                            ),
                                        ),
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
                            15,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 392,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::PrincipalEntityPath(
                                            PrincipalEntityPath::MajorItem(
                                                MajorItemPath::Type(
                                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 2,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 3,
                                            ident: `first`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 201,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 4,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 5,
                                            ident: `start`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 6,
                                            ident: `clone`,
                                            path: AssociatedItemPath::TraitForTypeItem(
                                                TraitForTypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 119,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 8,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 9,
                                            ident: `last`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 202,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 10,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 11,
                                            ident: `end`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 12,
                                            ident: `clone`,
                                            path: AssociatedItemPath::TraitForTypeItem(
                                                TraitForTypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 119,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::TypeConstructorFnCall {
                                            path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                            function_hir_eager_expr_idx: 1,
                                            template_arguments: None,
                                            item_groups: [
                                                Regular(
                                                    7,
                                                ),
                                                Regular(
                                                    13,
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
                                            expr_idx: 14,
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
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 393,
                            },
                        ),
                    ),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 393,
                                },
                            ),
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
                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 393,
                                                },
                                            ),
                                        ),
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
                            7,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 393,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 1,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 2,
                                            ident: `first`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 201,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 3,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 4,
                                            ident: `start`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 5,
                                            ident: `clone`,
                                            path: AssociatedItemPath::TraitForTypeItem(
                                                TraitForTypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 119,
                                                        },
                                                    ),
                                                ),
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
                                            expr_idx: 6,
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
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 394,
                            },
                        ),
                    ),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 394,
                                },
                            ),
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
                                ty_path: TypePath(`mnist_classifier::geom2d::Point2d`, `Struct`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 394,
                                                },
                                            ),
                                        ),
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
                            7,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::AssociatedItem(
                                        AssociatedItemPath::TypeItem(
                                            TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 394,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 1,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 2,
                                            ident: `last`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 202,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 3,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 4,
                                            ident: `end`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 5,
                                            ident: `clone`,
                                            path: AssociatedItemPath::TraitForTypeItem(
                                                TraitForTypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 119,
                                                        },
                                                    ),
                                                ),
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
                                            expr_idx: 6,
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
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 395,
                            },
                        ),
                    ),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 395,
                                },
                            ),
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
                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 395,
                                                },
                                            ),
                                        ),
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
                                            TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 395,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 1,
                                            ident: `line_segment`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 392,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 2,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 434,
                                                        },
                                                    ),
                                                ),
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 396,
                            },
                        ),
                    ),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 396,
                                },
                            ),
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
                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 396,
                                                },
                                            ),
                                        ),
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
                                            TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 396,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 1,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 2,
                                            ident: `first`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 201,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 3,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 4,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 407,
                                                        },
                                                    ),
                                                ),
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
    HirDefn::AssociatedItem(
        AssociatedItemHirDefn::TypeItem(
            TypeItemHirDefn::MethodFn(
                TypeMethodFnHirDefn {
                    path: TypeItemPath(
                        ItemPathId(
                            Id {
                                value: 397,
                            },
                        ),
                    ),
                    hir_decl: TypeMethodFnHirDecl {
                        path: TypeItemPath(
                            ItemPathId(
                                Id {
                                    value: 397,
                                },
                            ),
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
                                ty_path: TypePath(`mnist_classifier::geom2d::Vector2d`, `Struct`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::AssociatedItem(
                                    AssociatedItemPath::TypeItem(
                                        TypeItemPath(
                                            ItemPathId(
                                                Id {
                                                    value: 397,
                                                },
                                            ),
                                        ),
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
                                            TypeItemPath(
                                                ItemPathId(
                                                    Id {
                                                        value: 397,
                                                    },
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                                hir_eager_expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 1,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 2,
                                            ident: `last`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 202,
                                                        },
                                                    ),
                                                ),
                                            ),
                                            template_arguments: None,
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 3,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 4,
                                            ident: `displacement`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(
                                                    ItemPathId(
                                                        Id {
                                                            value: 407,
                                                        },
                                                    ),
                                                ),
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
]