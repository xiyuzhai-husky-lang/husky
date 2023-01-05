Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
        module_symbols: [
            ModuleItem {
                ident: `ConcaveComponent`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                ast_idx: 7,
                path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                EntityUseTracker {
                    ast_idx: 3,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 28,
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
                    ast_idx: 4,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 28,
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
                    use_expr_idx: 6,
                    parent: None,
                    state: Unresolved,
                },
                EntityUseTracker {
                    ast_idx: 5,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 28,
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
                    use_expr_idx: 10,
                    parent: None,
                    state: Unresolved,
                },
                EntityUseTracker {
                    ast_idx: 6,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 28,
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
                    use_expr_idx: 13,
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)