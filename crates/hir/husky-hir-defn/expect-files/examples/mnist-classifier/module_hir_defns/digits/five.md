[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValFugitiveHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::five::is_five`, `Val`),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 2,
                                },
                            ),
                        ),
                    },
                    body_with_hir_expr_region: Some(
                        (
                            Eager(
                                2,
                            ),
                            Eager(
                                HirEagerExprRegion(
                                    Id {
                                        value: 58,
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