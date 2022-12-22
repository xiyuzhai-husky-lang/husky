Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
        module_symbols: [
            ModuleItem {
                ident: `displacement`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch::line_segment`,
                ),
                ast_idx: 15,
                path: `mnist_classifier::line_segment_sketch::line_segment::displacement`,
            },
            ModuleItem {
                ident: `dist_to_point`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch::line_segment`,
                ),
                ast_idx: 16,
                path: `mnist_classifier::line_segment_sketch::line_segment::dist_to_point`,
            },
            ModuleItem {
                ident: `LineSegment`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch::line_segment`,
                ),
                ast_idx: 18,
                path: `mnist_classifier::line_segment_sketch::line_segment::LineSegment`,
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                EntityUseTracker {
                    ast_idx: 17,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 32,
                            },
                        ),
                    ),
                    ident: Identifier(
                        Word(
                            Id {
                                value: 33,
                            },
                        ),
                    ),
                    use_expr_idx: 2,
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)