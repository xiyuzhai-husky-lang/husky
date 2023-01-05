Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::major`,
        module_symbols: [
            ModuleItem {
                ident: `connected_components`,
                accessibility: PubicUnder(
                    `mnist_classifier::major`,
                ),
                ast_idx: 19,
                path: FormPath(`mnist_classifier::major::connected_components`, `Feature`),
            },
            ModuleItem {
                ident: `major_connected_component`,
                accessibility: PubicUnder(
                    `mnist_classifier::major`,
                ),
                ast_idx: 20,
                path: FormPath(`mnist_classifier::major::major_connected_component`, `Feature`),
            },
            ModuleItem {
                ident: `ignored_connected_components_row_span_sum_sum`,
                accessibility: PubicUnder(
                    `mnist_classifier::major`,
                ),
                ast_idx: 21,
                path: FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Feature`),
            },
            ModuleItem {
                ident: `major_raw_contours`,
                accessibility: PubicUnder(
                    `mnist_classifier::major`,
                ),
                ast_idx: 22,
                path: FormPath(`mnist_classifier::major::major_raw_contours`, `Feature`),
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                EntityUseTracker {
                    ast_idx: 15,
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
                                value: 32,
                            },
                        ),
                    ),
                    use_expr_idx: 2,
                    parent: None,
                    state: Unresolved,
                },
                EntityUseTracker {
                    ast_idx: 16,
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
                                value: 32,
                            },
                        ),
                    ),
                    use_expr_idx: 5,
                    parent: None,
                    state: Unresolved,
                },
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
                                value: 32,
                            },
                        ),
                    ),
                    use_expr_idx: 8,
                    parent: None,
                    state: Unresolved,
                },
                EntityUseTracker {
                    ast_idx: 18,
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
                                value: 32,
                            },
                        ),
                    ),
                    use_expr_idx: 12,
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)