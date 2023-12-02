[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                template_arguments: [],
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
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
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                6,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 214,
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
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
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
                                        FugitivePath(`mnist_classifier::digits::eight::is_eight`, `Val`),
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
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                29,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 216,
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
                    path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
                    hir_decl: FunctionFnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
                        template_parameters: HirTemplateParameters(
                            [],
                        ),
                        parenate_parameters: HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Ordinary {
                                    pattern_expr_idx: 1,
                                    contract: None,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
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
                                        FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
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
                                                    value: 286,
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
                                                `cc`,
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
                            22,
                            HirEagerExprRegion {
                                path: RegionPath::Defn(
                                    ItemPath::MajorItem(
                                        MajorItemPath::Fugitive(
                                            FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `FunctionFn`),
                                        ),
                                    ),
                                ),
                                expr_arena: Arena {
                                    data: [
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::MemoizedField {
                                            owner_hir_expr_idx: 1,
                                            ident: `relative_bounding_box`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`, `MemoizedField`),
                                            ),
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 2,
                                            ident: `ymax`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Literal(
                                            TermLiteral::F32(
                                                NotNan(
                                                    0.5,
                                                ),
                                            ),
                                        ),
                                        HirEagerExprData::Binary {
                                            lopd: 3,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 4,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 6,
                                            ident: `strokes`,
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 7,
                                            ident: `first`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::slice::CyclicSlice(0)::first`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [
                                                    (
                                                        HirComptimeSymbol::Type(
                                                            Type {
                                                                attrs: HirSymbolAttrs,
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        HirTermSymbolResolution::Explicit(
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                                        template_arguments: [],
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
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 8,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 9,
                                            ident: `start`,
                                        },
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 10,
                                            ident: `x`,
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
                                            ident: `first`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(core::slice::CyclicSlice(0)::first`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [
                                                    (
                                                        HirComptimeSymbol::Type(
                                                            Type {
                                                                attrs: HirSymbolAttrs,
                                                                variance: None,
                                                                disambiguator: 0,
                                                            },
                                                        ),
                                                        HirTermSymbolResolution::Explicit(
                                                            HirTemplateArgument::Type(
                                                                HirType::PathLeading(
                                                                    HirTypePathLeading {
                                                                        ty_path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                                                        template_arguments: [],
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
                                        HirEagerExprData::Suffix {
                                            opd_hir_expr_idx: 14,
                                            opr: Unwrap,
                                        },
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 15,
                                            ident: `end`,
                                        },
                                        HirEagerExprData::PropsStructField {
                                            owner_hir_expr_idx: 16,
                                            ident: `x`,
                                        },
                                        HirEagerExprData::Binary {
                                            lopd: 11,
                                            opr: Comparison(
                                                Greater,
                                            ),
                                            ropd: 17,
                                        },
                                        HirEagerExprData::Variable(
                                            1,
                                        ),
                                        HirEagerExprData::MemoizedField {
                                            owner_hir_expr_idx: 19,
                                            ident: `relative_bounding_box`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent(0)::relative_bounding_box`, `MemoizedField`),
                                            ),
                                        },
                                        HirEagerExprData::MethodFnCall {
                                            self_argument: 20,
                                            ident: `ymax`,
                                            path: AssociatedItemPath::TypeItem(
                                                TypeItemPath(`(mnist_classifier::geom2d::RelativeBoundingBox(0)::ymax`, `MethodFn`),
                                            ),
                                            instantiation: HirInstantiation {
                                                symbol_map: [],
                                                separator: Some(
                                                    0,
                                                ),
                                            },
                                            item_groups: [],
                                        },
                                        HirEagerExprData::Block {
                                            stmts: ArenaIdxRange(
                                                2..4,
                                            ),
                                        },
                                    ],
                                },
                                stmt_arena: Arena {
                                    data: [
                                        Require {
                                            condition: HirEagerCondition(
                                                18,
                                            ),
                                        },
                                        IfElse {
                                            if_branch: HirEagerIfBranch {
                                                condition: HirEagerCondition(
                                                    5,
                                                ),
                                                stmts: ArenaIdxRange(
                                                    1..2,
                                                ),
                                            },
                                            elif_branches: [],
                                            else_branch: None,
                                        },
                                        Eval {
                                            expr_idx: 21,
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
                                                name: HirEagerRuntimeSymbolName::Ident(
                                                    `cc`,
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