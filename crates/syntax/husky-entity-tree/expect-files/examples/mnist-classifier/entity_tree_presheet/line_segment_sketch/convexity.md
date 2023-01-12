Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch::convexity`,
        module_specific_symbols: [
            ModuleItem {
                ident: `is_convex`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch::convexity`,
                ),
                ast_idx: 22,
                path: FormPath(`mnist_classifier::line_segment_sketch::convexity::is_convex`, `Function`),
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 19,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::convexity`,
                        ),
                    },
                    use_tree_expr_children: ArenaIdxRange(
                        1..2,
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 20,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::convexity`,
                        ),
                    },
                    use_tree_expr_children: ArenaIdxRange(
                        4..5,
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 21,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::convexity`,
                        ),
                    },
                    use_tree_expr_children: ArenaIdxRange(
                        7..8,
                    ),
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)