Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
        module_specific_symbols: [
            ModuleItem {
                ident: `ConvexCompoent`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch::convex_component`,
                ),
                ast_idx: 5,
                path: TypePath(`mnist_classifier::line_segment_sketch::convex_component::ConvexCompoent`, `Struct`),
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 4,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::convex_component`,
                        ),
                    },
                    use_tree_expr_children: ArenaIdxRange(
                        1..2,
                    ),
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)