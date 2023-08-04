[
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            Enum(
                EnumTypeHirDecl(
                    Id {
                        value: 7,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Type(
            TupleStruct(
                TupleStructTypeHirDecl(
                    Id {
                        value: 1,
                    },
                ),
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Fugitive(
            FugitiveHirDecl::Val(
                ValFugitiveHirDecl {
                    path: FugitivePath(`mnist::input`, `Val`),
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
]