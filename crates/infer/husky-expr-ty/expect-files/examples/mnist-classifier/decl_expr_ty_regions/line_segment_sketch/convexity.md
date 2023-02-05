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
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                expectation_rule: None,
                resolve_progress: Resolved {
                    implicit_conversion: None,
                    term: Category(
                        TermCategory {
                            universe: TermUniverse(
                                1,
                            ),
                        },
                    ),
                },
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            implicit_symbol_registry: ImplicitSymbolRegistry {
                next: 0,
            },
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
]