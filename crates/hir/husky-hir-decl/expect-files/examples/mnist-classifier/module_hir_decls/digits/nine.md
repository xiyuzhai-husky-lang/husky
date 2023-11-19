[
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::Val(
                ValFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::digits::nine::nine_match`, `Val`),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 203,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::Val(
                ValFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::digits::nine::nine_match_refine`, `Val`),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 204,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::Val(
                ValFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::digits::nine::is_nine`, `Val`),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 205,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::FunctionFn(
                FunctionFnFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::digits::nine::downmost`, `FunctionFn`),
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
                                            value: 42,
                                        },
                                    ),
                                ),
                            },
                        ],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::option::Option`, `Enum`),
                            template_arguments: [
                                Type(
                                    PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 15,
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
                                    FugitivePath(`mnist_classifier::digits::nine::downmost`, `FunctionFn`),
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
                                                value: 286,
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
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::FunctionFn(
                FunctionFnFugitiveHirDecl {
                    path: FugitivePath(`mnist_classifier::digits::nine::big_cc`, `FunctionFn`),
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
                                            value: 42,
                                        },
                                    ),
                                ),
                            },
                        ],
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::option::Option`, `Enum`),
                            template_arguments: [
                                Type(
                                    PathLeading(
                                        HirTypePathLeading(
                                            Id {
                                                value: 15,
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
                                    FugitivePath(`mnist_classifier::digits::nine::big_cc`, `FunctionFn`),
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
                                                value: 286,
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
            ),
        ),
    ),
]