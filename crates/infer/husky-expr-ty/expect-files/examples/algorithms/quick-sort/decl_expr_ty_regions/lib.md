[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`quick_sort::quick_sort`, `Function`),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`quick_sort::quick_sort_aux`, `Function`),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`quick_sort::partition`, `Function`),
            ),
        ),
        expr_ty_infos: [
            ExprTypeInfo {
                ty_result: Err(
                    Derived(
                        ExprError,
                    ),
                ),
                opt_expectation: None,
            },
        ],
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`quick_sort::quick_sort_works_for_integers`, `Feature`),
            ),
        ),
        expr_ty_infos: [],
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`quick_sort::quick_sort_works_for_strs`, `Feature`),
            ),
        ),
        expr_ty_infos: [],
    },
]