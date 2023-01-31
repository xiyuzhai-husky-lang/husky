[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Some(
                    Ok(
                        Application(
                            TermApplication(
                                Id {
                                    value: 1,
                                },
                            ),
                        ),
                    ),
                ),
            },
        ],
        stmt_ty_infos: [],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectations: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
]