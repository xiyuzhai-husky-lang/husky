Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier`,
        module_specific_symbols: [
            Module {
                ident: `connected_component`,
                accessibility: PubicUnder(
                    `mnist_classifier`,
                ),
                ast_idx: 12,
                module_path: `mnist_classifier::connected_component`,
            },
            Module {
                ident: `raw_contour`,
                accessibility: PubicUnder(
                    `mnist_classifier`,
                ),
                ast_idx: 13,
                module_path: `mnist_classifier::raw_contour`,
            },
            Module {
                ident: `geom2d`,
                accessibility: PubicUnder(
                    `mnist_classifier`,
                ),
                ast_idx: 14,
                module_path: `mnist_classifier::geom2d`,
            },
            Module {
                ident: `line_segment_sketch`,
                accessibility: PubicUnder(
                    `mnist_classifier`,
                ),
                ast_idx: 15,
                module_path: `mnist_classifier::line_segment_sketch`,
            },
            Module {
                ident: `fermi`,
                accessibility: PubicUnder(
                    `mnist_classifier`,
                ),
                ast_idx: 16,
                module_path: `mnist_classifier::fermi`,
            },
            Module {
                ident: `digits`,
                accessibility: PubicUnder(
                    `mnist_classifier`,
                ),
                ast_idx: 17,
                module_path: `mnist_classifier::digits`,
            },
            Module {
                ident: `major`,
                accessibility: PubicUnder(
                    `mnist_classifier`,
                ),
                ast_idx: 18,
                module_path: `mnist_classifier::major`,
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                UseTracker {
                    ast_idx: 19,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 13,
                            },
                        ),
                    ),
                    ident: `mnist`,
                    use_expr_idx: 1,
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 20,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 13,
                            },
                        ),
                    ),
                    ident: `mnist`,
                    use_expr_idx: 3,
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 21,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 13,
                            },
                        ),
                    ),
                    ident: `major`,
                    use_expr_idx: 5,
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 22,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 13,
                            },
                        ),
                    ),
                    ident: `digits`,
                    use_expr_idx: 7,
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 23,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 13,
                            },
                        ),
                    ),
                    ident: `raw_contour`,
                    use_expr_idx: 9,
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 24,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 13,
                            },
                        ),
                    ),
                    ident: `line_segment_sketch`,
                    use_expr_idx: 11,
                    parent: None,
                    state: Unresolved,
                },
                UseTracker {
                    ast_idx: 25,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 13,
                            },
                        ),
                    ),
                    ident: `domains`,
                    use_expr_idx: 16,
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)