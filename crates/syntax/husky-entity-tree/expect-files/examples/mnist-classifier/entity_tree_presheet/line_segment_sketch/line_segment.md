Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
        module_specific_symbols: [
            ModuleItem {
                ident: `LineSegment`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch::line_segment`,
                ),
                ast_idx: 16,
                path: TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 15,
                    accessibility: Done {
                        accessibility: PubicUnder(
                            `mnist_classifier::line_segment_sketch::line_segment`,
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