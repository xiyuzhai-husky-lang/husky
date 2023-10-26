[
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: `mnist_classifier::connected_component`,
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: `mnist_classifier::raw_contour`,
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: `mnist_classifier::geom2d`,
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: `mnist_classifier::line_segment_sketch`,
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: `mnist_classifier::fermi`,
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: `mnist_classifier::digits`,
            },
        },
    ),
    HirDefn::Submodule(
        SubmoduleHirDefn {
            hir_decl: SubmoduleHirDecl {
                path: `mnist_classifier::major`,
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
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                20,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 8,
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