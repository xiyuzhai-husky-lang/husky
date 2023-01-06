Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch::line_segment`,
        module_symbols: [
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
                EntityUseTracker {
                    ast_idx: 15,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 31,
                            },
                        ),
                    ),
                    ident: Identifier(
                        Word(
                            Id {
                                value: 31,
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