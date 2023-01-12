Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
        module_specific_symbols: [
            ModuleItem {
                ident: `ConcaveComponent`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                ast_idx: 74,
                path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
            },
            ModuleItem {
                ident: `find_concave_components`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                ast_idx: 75,
                path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Function`),
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 70,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                    },
                    use_tree_expr_children: ArenaIdxRange(
                        1..2,
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 71,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                    },
                    use_tree_expr_children: ArenaIdxRange(
                        5..6,
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 72,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                    },
                    use_tree_expr_children: ArenaIdxRange(
                        9..10,
                    ),
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 73,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::concave_component`,
                        ),
                    },
                    use_tree_expr_children: ArenaIdxRange(
                        12..13,
                    ),
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)