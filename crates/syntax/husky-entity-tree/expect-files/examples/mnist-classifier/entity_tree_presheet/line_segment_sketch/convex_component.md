Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch::convex_component`,
        module_symbols: [
            ModuleItem {
                ident: `ConvexCompoent`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch::convex_component`,
                ),
                ast_idx: 5,
                path: TypePath {
                    module: `mnist_classifier::line_segment_sketch::convex_component`,
                    ident: Identifier(
                        Word(
                            Id {
                                value: 341,
                            },
                        ),
                    ),
                },
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                EntityUseTracker {
                    ast_idx: 4,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 29,
                            },
                        ),
                    ),
                    ident: Identifier(
                        Word(
                            Id {
                                value: 32,
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