Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch::concave_component`,
        module_symbols: [
            ModuleItem {
                ident: `ConcaveComponent`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                ast_idx: 77,
                path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent, Struct`),
            },
            ModuleItem {
                ident: `find_concave_components`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch::concave_component`,
                ),
                ast_idx: 78,
                path: FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components, Function`),
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                EntityUseTracker {
                    ast_idx: 73,
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
                    ast_idx: 74,
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
                    ast_idx: 75,
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
                    ast_idx: 76,
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