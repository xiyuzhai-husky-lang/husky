[
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Val(
                ValHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                    hir_decl: ValFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::eight::upper_mouth_match`, `Val`),
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    },
                    body: Some(
                        Eager(
                            5,
                        ),
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 1,
                            },
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
                        hir_expr_region: Eager(
                            HirEagerExprRegion(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    },
                    body: Some(
                        Eager(
                            28,
                        ),
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 1,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Fugitive(
            FugitiveHirDefn::Fn(
                FnHirDefn {
                    path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
                    hir_decl: FnFugitiveHirDecl {
                        path: FugitivePath(`mnist_classifier::digits::eight::big_mouth`, `Fn`),
                        template_parameters: HirTemplateParameters {
                            data: [],
                        },
                    },
                    body: Some(
                        21,
                    ),
                    hir_expr_region: HirEagerExprRegion,
                },
            ),
        ),
    ),
]