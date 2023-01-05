Ok(
    EntityTreePresheet {
        module_path: `mnist_classifier::line_segment_sketch`,
        module_symbols: [
            Module {
                ident: `concave_component`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch`,
                ),
                ast_idx: 159,
                module_path: `mnist_classifier::line_segment_sketch::concave_component`,
            },
            Module {
                ident: `convex_component`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch`,
                ),
                ast_idx: 160,
                module_path: `mnist_classifier::line_segment_sketch::convex_component`,
            },
            Module {
                ident: `convexity`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch`,
                ),
                ast_idx: 161,
                module_path: `mnist_classifier::line_segment_sketch::convexity`,
            },
            Module {
                ident: `line_segment`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch`,
                ),
                ast_idx: 162,
                module_path: `mnist_classifier::line_segment_sketch::line_segment`,
            },
            ModuleItem {
                ident: `LineSegmentStroke`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch`,
                ),
                ast_idx: 168,
                path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
            },
            ModuleItem {
                ident: `LineSegmentSketch`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch`,
                ),
                ast_idx: 170,
                path: TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
            },
            ModuleItem {
                ident: `go_right`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch`,
                ),
                ast_idx: 172,
                path: FormPath(`mnist_classifier::line_segment_sketch::go_right`, `Function`),
            },
            ModuleItem {
                ident: `go_left`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch`,
                ),
                ast_idx: 173,
                path: FormPath(`mnist_classifier::line_segment_sketch::go_left`, `Function`),
            },
            ModuleItem {
                ident: `extend_end`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch`,
                ),
                ast_idx: 174,
                path: FormPath(`mnist_classifier::line_segment_sketch::extend_end`, `Function`),
            },
            ModuleItem {
                ident: `extend_start`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch`,
                ),
                ast_idx: 175,
                path: FormPath(`mnist_classifier::line_segment_sketch::extend_start`, `Function`),
            },
            ModuleItem {
                ident: `find_line_segments`,
                accessibility: PubicUnder(
                    `mnist_classifier::line_segment_sketch`,
                ),
                ast_idx: 176,
                path: FormPath(`mnist_classifier::line_segment_sketch::find_line_segments`, `Function`),
            },
        ],
        entity_use_roots: EntityUseExprTrackers(
            [
                EntityUseTracker {
                    ast_idx: 163,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 27,
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
                    ast_idx: 164,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 27,
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
                    ast_idx: 165,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                    ident: Identifier(
                        Word(
                            Id {
                                value: 74,
                            },
                        ),
                    ),
                    use_expr_idx: 7,
                    parent: None,
                    state: Unresolved,
                },
                EntityUseTracker {
                    ast_idx: 166,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                    ident: Identifier(
                        Word(
                            Id {
                                value: 75,
                            },
                        ),
                    ),
                    use_expr_idx: 9,
                    parent: None,
                    state: Unresolved,
                },
                EntityUseTracker {
                    ast_idx: 167,
                    accessibility: PublicUnder(
                        ModulePath(
                            Id {
                                value: 27,
                            },
                        ),
                    ),
                    ident: Identifier(
                        Word(
                            Id {
                                value: 77,
                            },
                        ),
                    ),
                    use_expr_idx: 11,
                    parent: None,
                    state: Unresolved,
                },
            ],
        ),
    },
)