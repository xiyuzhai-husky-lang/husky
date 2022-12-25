Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch::convexity`,
        module_symbols: [
            ModuleItem {
                ident: `is_convex`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch::convexity`,
                ),
                ast_idx: 23,
                path: Connected(
                    ConnectedModuleItemPath {
                        module_path: `mnist_classifier::line_segment_sketch::convexity`,
                        ident: Identifier(
                            Word(
                                Id {
                                    value: 338,
                                },
                            ),
                        ),
                    },
                ),
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                EntityUseTracker {
                    ast_idx: 20,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 30,
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
                EntityUseTracker {
                    ast_idx: 21,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 30,
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
                    use_expr_idx: 5,
                    parent: None,
                    state: Unresolved,
                },
                EntityUseTracker {
                    ast_idx: 22,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 30,
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
                    use_expr_idx: 8,
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)