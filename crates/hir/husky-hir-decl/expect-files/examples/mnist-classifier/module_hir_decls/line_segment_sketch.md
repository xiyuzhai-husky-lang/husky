[
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmodulePath(
                `mnist_classifier::line_segment_sketch::concave_component`,
            ),
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmodulePath(
                `mnist_classifier::line_segment_sketch::convex_component`,
            ),
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmodulePath(
                `mnist_classifier::line_segment_sketch::convexity`,
            ),
        },
    ),
    HirDecl::Submodule(
        SubmoduleHirDecl {
            path: SubmodulePath(
                `mnist_classifier::line_segment_sketch::line_segment`,
            ),
        },
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            PropsStruct(
                PropsStructTypeHirDecl(
                    Id {
                        value: 14,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            PropsStruct(
                PropsStructTypeHirDecl(
                    Id {
                        value: 15,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::FunctionFn(
                FunctionFnFugitiveHirDecl {
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
                                            value: 52,
                                        },
                                    ),
                                ),
                            },
                            Ordinary {
                                pattern_expr_idx: 2,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 14,
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
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::FunctionFn(
                FunctionFnFugitiveHirDecl {
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
                                            value: 52,
                                        },
                                    ),
                                ),
                            },
                            Ordinary {
                                pattern_expr_idx: 2,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 14,
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
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::FunctionFn(
                FunctionFnFugitiveHirDecl {
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
                                            value: 31,
                                        },
                                    ),
                                ),
                            },
                            Ordinary {
                                pattern_expr_idx: 2,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                            },
                            Ordinary {
                                pattern_expr_idx: 3,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 14,
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
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::FunctionFn(
                FunctionFnFugitiveHirDecl {
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
                                            value: 31,
                                        },
                                    ),
                                ),
                            },
                            Ordinary {
                                pattern_expr_idx: 2,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                            },
                            Ordinary {
                                pattern_expr_idx: 3,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 5,
                                        },
                                    ),
                                ),
                            },
                            Ordinary {
                                pattern_expr_idx: 4,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 14,
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
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::FunctionFn(
                FunctionFnFugitiveHirDecl {
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
                                            value: 31,
                                        },
                                    ),
                                ),
                            },
                            Ordinary {
                                pattern_expr_idx: 2,
                                ty: PathLeading(
                                    HirTypePathLeading(
                                        Id {
                                            value: 14,
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
                                                value: 59,
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
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
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
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TraitForTypeItem(
            MethodFn(
                TraitForTypeMethodFnHirDecl(
                    Id {
                        value: 21,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
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
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            AssociatedFn(
                TypeAssociatedFnHirDecl(
                    Id {
                        value: 2,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MethodFn(
                TypeMethodFnHirDecl(
                    Id {
                        value: 81,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
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
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TraitForTypeItem(
            MethodFn(
                TraitForTypeMethodFnHirDecl(
                    Id {
                        value: 22,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::Type(
            TypeImplBlockHirDecl {
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
            },
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MemoizedField(
                TypeMemoizedFieldHirDecl(
                    Id {
                        value: 18,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            MemoizedField(
                TypeMemoizedFieldHirDecl(
                    Id {
                        value: 19,
                    },
                ),
            ),
        ),
    ),
    HirDecl::AssociatedItem(
        AssociatedItemHirDecl::TypeItem(
            AssociatedFn(
                TypeAssociatedFnHirDecl(
                    Id {
                        value: 3,
                    },
                ),
            ),
        ),
    ),
]