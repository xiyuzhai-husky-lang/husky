[
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::connected_component`,
                                ),
                            },
                        ),
                    },
                ),
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::raw_contour`,
                                ),
                            },
                        ),
                    },
                ),
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::geom2d`,
                                ),
                            },
                        ),
                    },
                ),
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::line_segment_sketch`,
                                ),
                            },
                        ),
                    },
                ),
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::fermi`,
                                ),
                            },
                        ),
                    },
                ),
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::digits`,
                                ),
                            },
                        ),
                    },
                ),
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `mnist_classifier::major`,
                                ),
                            },
                        ),
                    },
                ),
            },
        },
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::main`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::main`, `Val`),
                        return_ty: HirType::PathLeading(
                            HirTypePathLeading {
                                ty_path: TypePath(`malamute::Class`, `Enum`),
                                template_arguments: [
                                    HirTemplateArgument::Type(
                                        HirType::PathLeading(
                                            HirTypePathLeading {
                                                ty_path: TypePath(`mnist::MnistLabel`, `Enum`),
                                                template_arguments: [],
                                                always_copyable: false,
                                            },
                                        ),
                                    ),
                                ],
                                always_copyable: true,
                            },
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            path: RegionPath::Decl(
                                ItemPath::MajorItem(
                                    MajorItemPath::Fugitive(
                                        FugitivePath(`mnist_classifier::main`, `Val`),
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
                                20,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 179,
                                    },
                                ),
                            ),
                        ),
                    ),
                },
            ),
        ),
    ),
]