[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            PropsStruct(
                PropsStructTypeHirDecl(
                    Id {
                        value: 13,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
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
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
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
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            TypeItemHirDecl::MethodFn(
                TypeMethodFnHirDecl {
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
            ),
        ),
    ),
]